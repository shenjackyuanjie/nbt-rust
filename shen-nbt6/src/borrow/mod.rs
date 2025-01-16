use crate::traits::{NbtBorrowTrait, NbtTypeConversion};
use crate::{nbt_consts, nbt_versions, value, NbtError, NbtReader, NbtResult, NbtTypeId};
#[cfg(test)]
mod tests;

/// 这里的所有 usize 实际上都指向一个 &[u8]
///
/// 用于更快速的解析 Nbt 数据
///
/// 所有 usize 都指向对应数据的开始位置
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowNbtValue {
    Byte(usize),
    Short(usize),
    Int(usize),
    Long(usize),
    Float(usize),
    Double(usize),
    /// ptr, len
    ByteArray(usize, usize),
    /// ptr, len
    String(usize, usize),
    /// ptr, len, type_id, values
    List(usize, usize, NbtTypeId, Vec<BorrowNbtValue>),
    /// ptr, str_len, vec<(str_len, BorrowNbtValue)>
    /// 如果是 None, 则表示没有名称
    /// 否则表示有名称(0 != 无名称)
    Compound(usize, Option<usize>, Vec<(usize, BorrowNbtValue)>),
    /// ptr, len
    IntArray(usize, usize),
    /// ptr, len
    LongArray(usize, usize),
}

/// 虽然计划是在 borrow 里手动模拟 stack, 但是 stack 的大小还是需要限制一下
///
/// 不过既然是手动模拟了, 那就可以稍微大一些
pub const RECURSE_LIMIT: usize = 2048;

impl NbtBorrowTrait for nbt_versions::Java {
    fn from_reader(reader: &mut NbtReader) -> NbtResult<BorrowNbtValue> {
        let first_type_id = reader.read_u8()?;
        match first_type_id {
            nbt_consts::TAG_COMPOUND => (),
            x => {
                return Err(NbtError::WrongRootType(x));
            }
        }
        // 这里需要注意的是, 可能会有无名称的 root Compound
        // 所以需要尝试性的读取一下, 如果读取出现错误, 则表示没有名称, 丢一个 RootWithoutName 回去
        // 当然, 有可能压根没法判断是不是没有名称……
        // 开始怀疑 RootWithoutName 这个错误码的存在意义了……
        let root_name_len = reader.read_be_u16()? as usize;
        let mut root = BorrowNbtValue::Compound(0, Some(root_name_len), vec![]);
        // 跳过 root_name
        reader.roll_down(root_name_len)?;

        // 开始解析
        // 先创建一个模拟的 stack
        // 顺便把 root 放进去
        let mut read_stack: Vec<&mut BorrowNbtValue> = vec![&mut root];
        // 堆栈规则: FILO (First In Last Out)
        // 栈顶是当前正在处理的对象
        // 栈底是 root
        // 堆栈清空时, 解析完成

        while !read_stack.is_empty() {
            // 先检查堆栈是否超出限制
            if read_stack.len() > RECURSE_LIMIT {
                return Err(NbtError::NbtDepthTooBig(RECURSE_LIMIT));
            }
            // 取出栈顶对象
            let current = read_stack.last_mut().unwrap();
            // 开始持续尝试读取对应的数据
            // 这里分 Compound 和 List 两种情况
            // 读取的时候是直接从当前的 cursor 开始读取的
            // 所以每次压栈/出栈的时候都需要先更新 cursor, 然后再 continue
            match current {
                BorrowNbtValue::Compound(_start_ptr, _name_len, values) => {
                    // 读取逻辑: 当前 cursor 是 Compound 的第n个值的开始位置
                    // 所以直接读取当前值的类型即可
                    // 重复读取值, 直到遇到需要压栈的 Compound/List
                    loop {
                        let value_type_id = reader.read_u8()?;
                        if value_type_id == nbt_consts::TAG_END {
                            // 读取到了 TAG_END
                            // 弹出栈顶对象
                            read_stack.pop();
                            break;
                        }
                        let value_name_len = reader.read_be_u16()? as usize;
                        println!(
                            "Value type: {}, name_len: {}, cursor: {}",
                            value_type_id, value_name_len, reader.cursor
                        );
                        // 跳过 name
                        reader.roll_down(value_name_len)?;
                        match value_type_id {
                            nbt_consts::TAG_BYTE => {
                                // 读取到了 TAG_BYTE
                                // 创建一个 Byte 对象
                                let value_ptr = reader.cursor;
                                let value = BorrowNbtValue::Byte(value_ptr);
                                values.push((value_name_len, value));
                                // 移动 cursor
                                reader.roll_down(1)?;
                            }
                            nbt_consts::TAG_SHORT => {
                                let value_ptr = reader.cursor;
                                let value = BorrowNbtValue::Short(value_ptr);
                                values.push((value_name_len, value));
                                reader.roll_down(2)?;
                            }
                            nbt_consts::TAG_INT => {
                                let value_ptr = reader.cursor;
                                let value = BorrowNbtValue::Int(value_ptr);
                                values.push((value_name_len, value));
                                reader.roll_down(4)?;
                            }
                            nbt_consts::TAG_LONG => {
                                let value_ptr = reader.cursor;
                                let value = BorrowNbtValue::Long(value_ptr);
                                values.push((value_name_len, value));
                                reader.roll_down(8)?;
                            }
                            nbt_consts::TAG_FLOAT => {
                                let value_ptr = reader.cursor;
                                let value = BorrowNbtValue::Float(value_ptr);
                                values.push((value_name_len, value));
                                reader.roll_down(4)?;
                            }
                            nbt_consts::TAG_DOUBLE => {
                                let value_ptr = reader.cursor;
                                let value = BorrowNbtValue::Double(value_ptr);
                                values.push((value_name_len, value));
                                reader.roll_down(8)?;
                            }
                            nbt_consts::TAG_BYTE_ARRAY => {
                                // 读取 ByteArray
                                let value_ptr = reader.cursor;
                                // 长度是 i32
                                let value_len = reader.read_be_i32()?;
                                if value_len < 0 {
                                    return Err(NbtError::LenNegative(
                                        value_type_id,
                                        value_len,
                                        value_ptr,
                                    ));
                                }
                                let value =
                                    BorrowNbtValue::ByteArray(value_ptr, value_len as usize);
                                values.push((value_name_len, value));
                                // 移动 cursor
                                reader.roll_down(value_len as usize)?;
                            }
                            nbt_consts::TAG_INT_ARRAY => {
                                let value_ptr = reader.cursor;
                                let value_len = reader.read_be_i32()?;
                                if value_len < 0 {
                                    return Err(NbtError::LenNegative(
                                        value_type_id,
                                        value_len,
                                        value_ptr,
                                    ));
                                }
                                let value = BorrowNbtValue::IntArray(value_ptr, value_len as usize);
                                values.push((value_name_len, value));
                                reader.roll_down(value_len as usize * 4)?;
                            }
                            nbt_consts::TAG_LONG_ARRAY => {
                                let value_ptr = reader.cursor;
                                let value_len = reader.read_be_i32()?;
                                if value_len < 0 {
                                    return Err(NbtError::LenNegative(
                                        value_type_id,
                                        value_len,
                                        value_ptr,
                                    ));
                                }
                                let value =
                                    BorrowNbtValue::LongArray(value_ptr, value_len as usize);
                                values.push((value_name_len, value));
                                reader.roll_down(value_len as usize * 8)?;
                            }
                            nbt_consts::TAG_STRING => {
                                let value_ptr = reader.cursor;
                                let value_len = reader.read_be_u16()? as usize; // 总算不需要检查负数了
                                let value = BorrowNbtValue::String(value_ptr, value_len);
                                values.push((value_name_len, value));
                                reader.roll_down(value_len)?;
                            }
                            nbt_consts::TAG_LIST => {
                                let value_ptr = reader.cursor;
                                let lst_type = reader.read_u8()?;
                                if !lst_type.is_valid_nbt_data_type() {
                                    return Err(NbtError::UnknownType(lst_type));
                                }
                                let lst_len = reader.read_be_i32()?;
                                if lst_len < 0 {
                                    return Err(NbtError::LenNegative(
                                        lst_type, lst_len, value_ptr,
                                    ));
                                }
                                let lst_len = lst_len as usize;
                                if lst_type.is_list_or_compound() {
                                    // 这两个需要压栈
                                    let value = BorrowNbtValue::List(
                                        value_ptr,
                                        lst_len as usize,
                                        lst_type,
                                        vec![],
                                    );
                                    values.push((value_name_len, value));
                                    break;
                                }
                                let current_ptr = reader.cursor;
                                // 可直接读取的类型
                                match lst_type {
                                    // byte/short/int/long/float/double
                                    nbt_consts::TAG_BYTE => {
                                        let lst_values = (0..lst_len)
                                            .map(|i| BorrowNbtValue::Byte(current_ptr + i))
                                            .collect::<Vec<BorrowNbtValue>>();
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                        // 检查溢出
                                        reader.roll_down(lst_len)?;
                                    }
                                    nbt_consts::TAG_SHORT => {
                                        let lst_values = (0..lst_len)
                                            .map(|i| BorrowNbtValue::Short(current_ptr + i * 2))
                                            .collect::<Vec<BorrowNbtValue>>();
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                        reader.roll_down(lst_len * 2)?;
                                    }
                                    nbt_consts::TAG_INT => {
                                        let lst_values = (0..lst_len)
                                            .map(|i| BorrowNbtValue::Int(current_ptr + i * 4))
                                            .collect::<Vec<BorrowNbtValue>>();
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                        reader.roll_down(lst_len * 4)?;
                                    }
                                    nbt_consts::TAG_LONG => {
                                        let lst_values = (0..lst_len)
                                            .map(|i| BorrowNbtValue::Long(current_ptr + i * 8))
                                            .collect::<Vec<BorrowNbtValue>>();
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                        reader.roll_down(lst_len * 8)?;
                                    }
                                    nbt_consts::TAG_FLOAT => {
                                        let lst_values = (0..lst_len)
                                            .map(|i| BorrowNbtValue::Float(current_ptr + i * 4))
                                            .collect::<Vec<BorrowNbtValue>>();
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                        reader.roll_down(lst_len * 4)?;
                                    }
                                    nbt_consts::TAG_DOUBLE => {
                                        let lst_values = (0..lst_len)
                                            .map(|i| BorrowNbtValue::Double(current_ptr + i * 8))
                                            .collect::<Vec<BorrowNbtValue>>();
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                        reader.roll_down(lst_len * 8)?;
                                    }
                                    // byte/int/long array
                                    nbt_consts::TAG_BYTE_ARRAY => {
                                        let mut lst_values = Vec::with_capacity(lst_len);
                                        for _ in 0..lst_len {
                                            let value_ptr = reader.cursor;
                                            let value_len = reader.read_be_i32()?;
                                            if value_len < 0 {
                                                return Err(NbtError::LenNegative(
                                                    lst_type, value_len, value_ptr,
                                                ));
                                            }
                                            let value_len = value_len as usize;
                                            let value =
                                                BorrowNbtValue::ByteArray(value_ptr, value_len);
                                            lst_values.push(value);
                                            reader.roll_down(value_len)?;
                                        }
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                    }
                                    nbt_consts::TAG_INT_ARRAY => {
                                        let mut lst_values = Vec::with_capacity(lst_len);
                                        for _ in 0..lst_len {
                                            let value_ptr = reader.cursor;
                                            let value_len = reader.read_be_i32()?;
                                            if value_len < 0 {
                                                return Err(NbtError::LenNegative(
                                                    lst_type, value_len, value_ptr,
                                                ));
                                            }
                                            let value_len = value_len as usize;
                                            let value =
                                                BorrowNbtValue::IntArray(value_ptr, value_len);
                                            lst_values.push(value);
                                            reader.roll_down(value_len * 4)?;
                                        }
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                    }
                                    nbt_consts::TAG_LONG_ARRAY => {
                                        let mut lst_values = Vec::with_capacity(lst_len);
                                        for _ in 0..lst_len {
                                            let value_ptr = reader.cursor;
                                            let value_len = reader.read_be_i32()?;
                                            if value_len < 0 {
                                                return Err(NbtError::LenNegative(
                                                    lst_type, value_len, value_ptr,
                                                ));
                                            }
                                            let value_len = value_len as usize;
                                            let value =
                                                BorrowNbtValue::LongArray(value_ptr, value_len);
                                            lst_values.push(value);
                                            reader.roll_down(value_len * 8)?;
                                        }
                                        values.push((
                                            value_name_len,
                                            BorrowNbtValue::List(
                                                value_ptr, lst_len, lst_type, lst_values,
                                            ),
                                        ));
                                    }
                                    _ => unreachable!("其他的都预处理过了"),
                                }
                            }
                            nbt_consts::TAG_COMPOUND => {
                                let value_ptr = reader.cursor;
                                // 非 root 的 Compound
                                let value = BorrowNbtValue::Compound(value_ptr, None, vec![]);
                                values.push((value_name_len, value));
                                break;
                            }
                            nbt_consts::TAG_END => {
                                unreachable!("前面处理过了")
                            }
                            _ => {
                                return Err(NbtError::UnknownType(value_type_id));
                            }
                        }
                    }
                }
                BorrowNbtValue::List(_start, lst_len, lst_type, values) => {
                    // cursor 的位置就是当前需要读取的下一个值的开始位置
                    // 先检查长度是不是读完了
                    if values.len() == *lst_len {
                        // 读取完了, 弹出栈顶对象
                        read_stack.pop();
                        continue;
                    }
                    match *lst_type {
                        nbt_consts::TAG_LIST => {
                            // 读取子 list 的类型
                            let sub_lst_type = reader.read_u8()?;
                            if !sub_lst_type.is_valid_nbt_data_type() {
                                return Err(NbtError::UnknownType(sub_lst_type));
                            }
                            let sub_lst_len = reader.read_be_i32()?;
                            if sub_lst_len < 0 {
                                return Err(NbtError::LenNegative(
                                    sub_lst_type,
                                    sub_lst_len,
                                    reader.cursor,
                                ));
                            }
                            let sub_lst_len = sub_lst_len as usize;
                            if sub_lst_type.is_list_or_compound() {
                                // 这两个需要压栈
                                let value = BorrowNbtValue::List(
                                    reader.cursor,
                                    sub_lst_len,
                                    sub_lst_type,
                                    vec![],
                                );
                                values.push(value);
                                continue;
                            }
                            let current_ptr = reader.cursor;

                            // 可直接读取的类型
                            match sub_lst_type {
                                // byte/short/int/long/float/double
                                nbt_consts::TAG_BYTE => {
                                    let lst_values = (0..sub_lst_len)
                                        .map(|i| BorrowNbtValue::Byte(current_ptr + i))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                    // 检查溢出
                                    reader.roll_down(sub_lst_len)?;
                                }
                                nbt_consts::TAG_SHORT => {
                                    let lst_values = (0..sub_lst_len)
                                        .map(|i| BorrowNbtValue::Short(current_ptr + i * 2))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                    reader.roll_down(sub_lst_len * 2)?;
                                }
                                nbt_consts::TAG_INT => {
                                    let lst_values = (0..sub_lst_len)
                                        .map(|i| BorrowNbtValue::Int(current_ptr + i * 4))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                    reader.roll_down(sub_lst_len * 4)?;
                                }
                                nbt_consts::TAG_LONG => {
                                    let lst_values = (0..sub_lst_len)
                                        .map(|i| BorrowNbtValue::Long(current_ptr + i * 8))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                    reader.roll_down(sub_lst_len * 8)?;
                                }
                                nbt_consts::TAG_FLOAT => {
                                    let lst_values = (0..sub_lst_len)
                                        .map(|i| BorrowNbtValue::Float(current_ptr + i * 4))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                    reader.roll_down(sub_lst_len * 4)?;
                                }
                                nbt_consts::TAG_DOUBLE => {
                                    let lst_values = (0..sub_lst_len)
                                        .map(|i| BorrowNbtValue::Double(current_ptr + i * 8))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                    reader.roll_down(sub_lst_len * 8)?;
                                }
                                // arrays
                                nbt_consts::TAG_BYTE_ARRAY => {
                                    let mut lst_values = Vec::with_capacity(sub_lst_len);
                                    for _ in 0..sub_lst_len {
                                        let value_ptr = reader.cursor;
                                        let value_len = reader.read_be_i32()?;
                                        if value_len < 0 {
                                            return Err(NbtError::LenNegative(
                                                sub_lst_type,
                                                value_len,
                                                value_ptr,
                                            ));
                                        }
                                        let value_len = value_len as usize;
                                        let value = BorrowNbtValue::ByteArray(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len)?;
                                    }
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                }
                                nbt_consts::TAG_INT_ARRAY => {
                                    let mut lst_values = Vec::with_capacity(sub_lst_len);
                                    for _ in 0..sub_lst_len {
                                        let value_ptr = reader.cursor;
                                        let value_len = reader.read_be_i32()?;
                                        if value_len < 0 {
                                            return Err(NbtError::LenNegative(
                                                sub_lst_type,
                                                value_len,
                                                value_ptr,
                                            ));
                                        }
                                        let value_len = value_len as usize;
                                        let value = BorrowNbtValue::IntArray(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len * 4)?;
                                    }
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                }
                                nbt_consts::TAG_LONG_ARRAY => {
                                    let mut lst_values = Vec::with_capacity(sub_lst_len);
                                    for _ in 0..sub_lst_len {
                                        let value_ptr = reader.cursor;
                                        let value_len = reader.read_be_i32()?;
                                        if value_len < 0 {
                                            return Err(NbtError::LenNegative(
                                                sub_lst_type,
                                                value_len,
                                                value_ptr,
                                            ));
                                        }
                                        let value_len = value_len as usize;
                                        let value = BorrowNbtValue::LongArray(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len * 8)?;
                                    }
                                    values.push(BorrowNbtValue::List(
                                        current_ptr,
                                        sub_lst_len,
                                        sub_lst_type,
                                        lst_values,
                                    ));
                                }
                                _ => unreachable!("其他的都预处理过了"),
                            }
                        }
                        nbt_consts::TAG_COMPOUND => {}
                        _ => {
                            unreachable!("在外面就检查过了")
                        }
                    }
                }
                _ => {
                    unreachable!(
                        "根节点不可能是其他类型, 读取的时候也不会把非 List/Compound 的对象放进栈中"
                    )
                }
            };
        }

        // 在所有工作都做完之后
        Ok(root)
    }
}
