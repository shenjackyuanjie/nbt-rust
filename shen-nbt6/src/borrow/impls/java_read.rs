use crate::borrow::BorrowNbtValue;
use crate::traits::NbtTypeConversion;
use crate::{nbt_consts, NbtError, NbtReader, NbtResult, RECURSE_LIMIT};

/// 实际的解析函数
///
/// 为了网络格式 加一个 root_with_name 参数
pub fn java_from_reader(reader: &mut NbtReader, root_with_name: bool) -> NbtResult<BorrowNbtValue> {
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
    let mut root = if root_with_name {
        let root_name_len = reader.read_be_u16()? as usize;
        // 跳过 root_name
        reader.roll_down(root_name_len)?;
        BorrowNbtValue::Compound(0, Some(root_name_len), vec![])
    } else {
        BorrowNbtValue::Compound(0, None, vec![])
    };
    // 跳过 root_name

    // 开始解析
    // 先创建一个模拟的 stack
    // 顺便把 root 放进去
    let mut read_stack: Vec<&mut BorrowNbtValue> = Vec::with_capacity(RECURSE_LIMIT);
    read_stack.push(&mut root);
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
        let current = read_stack.last().unwrap();
        let current: &mut BorrowNbtValue = unsafe {
            // SAFETY: 这里的操作是安全的, 因为 pop 之后直接 break 了
            std::ptr::read(current)
        };
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
                    let value_name_start = reader.cursor;
                    // println!(
                    //     "Value type: {}, name_len: {}, cursor:\n{}",
                    //     value_type_id.as_nbt_type_name(),
                    //     value_name_len,
                    //     reader.show_cursor_fancy(None)
                    // );
                    // 跳过 name
                    reader.roll_down(value_name_len)?;
                    match value_type_id {
                        nbt_consts::TAG_BYTE => {
                            // 读取到了 TAG_BYTE
                            // 创建一个 Byte 对象
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::Byte(value_ptr);
                            values.push((value_name_start, value_name_len, value));
                            // 移动 cursor
                            reader.roll_down(1)?;
                        }
                        nbt_consts::TAG_SHORT => {
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::Short(value_ptr);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(2)?;
                        }
                        nbt_consts::TAG_INT => {
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::Int(value_ptr);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(4)?;
                        }
                        nbt_consts::TAG_LONG => {
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::Long(value_ptr);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(8)?;
                        }
                        nbt_consts::TAG_FLOAT => {
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::Float(value_ptr);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(4)?;
                        }
                        nbt_consts::TAG_DOUBLE => {
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::Double(value_ptr);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(8)?;
                        }
                        nbt_consts::TAG_BYTE_ARRAY => {
                            // 读取 ByteArray
                            // 长度是 i32
                            let value_len = reader.read_be_i32()?;
                            let value_ptr = reader.cursor;
                            if value_len < 0 {
                                return Err(NbtError::LenNegative(
                                    value_type_id,
                                    value_len,
                                    value_ptr,
                                ));
                            }
                            let value = BorrowNbtValue::ByteArray(value_ptr, value_len as usize);
                            values.push((value_name_start, value_name_len, value));
                            // 移动 cursor
                            reader.roll_down(value_len as usize)?;
                        }
                        nbt_consts::TAG_INT_ARRAY => {
                            let value_len = reader.read_be_i32()?;
                            let value_ptr = reader.cursor;
                            if value_len < 0 {
                                return Err(NbtError::LenNegative(
                                    value_type_id,
                                    value_len,
                                    value_ptr,
                                ));
                            }
                            let value = BorrowNbtValue::IntArray(value_ptr, value_len as usize);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(value_len as usize * 4)?;
                        }
                        nbt_consts::TAG_LONG_ARRAY => {
                            let value_len = reader.read_be_i32()?;
                            let value_ptr = reader.cursor;
                            if value_len < 0 {
                                return Err(NbtError::LenNegative(
                                    value_type_id,
                                    value_len,
                                    value_ptr,
                                ));
                            }
                            let value = BorrowNbtValue::LongArray(value_ptr, value_len as usize);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(value_len as usize * 8)?;
                        }
                        nbt_consts::TAG_STRING => {
                            let value_len = reader.read_be_u16()? as usize; // 总算不需要检查负数了
                            let value_ptr = reader.cursor;
                            let value = BorrowNbtValue::String(value_ptr, value_len);
                            values.push((value_name_start, value_name_len, value));
                            reader.roll_down(value_len)?;
                        }
                        nbt_consts::TAG_LIST => {
                            let lst_type = reader.read_u8()?;
                            // 读过 type id 再读指针位置
                            let value_ptr = reader.cursor;
                            // NbtList 里允许 TagEnd
                            if !lst_type.is_valid_nbt_type() {
                                return Err(NbtError::UnknownType(lst_type, value_ptr));
                            }
                            let lst_len = reader.read_be_i32()?;
                            if lst_len < 0 {
                                return Err(NbtError::LenNegative(lst_type, lst_len, value_ptr));
                            }
                            let lst_len = lst_len as usize;
                            if lst_type.is_list_or_compound() {
                                let sub_lst = Vec::with_capacity(lst_len);
                                // 这两个需要压栈
                                let value =
                                    BorrowNbtValue::List(value_ptr, lst_len, lst_type, sub_lst);
                                values.push((value_name_start, value_name_len, value));
                                let last = values.last_mut().unwrap();
                                read_stack.push(&mut last.2);
                                break;
                            }
                            let current_ptr = reader.cursor;
                            // 可直接读取的类型
                            match lst_type {
                                nbt_consts::TAG_END => {
                                    // 真有 end 标签……
                                    let lst_0 = Vec::with_capacity(0);
                                    let value =
                                        BorrowNbtValue::List(value_ptr, lst_len, lst_type, lst_0);
                                    values.push((value_name_start, value_name_len, value));
                                    reader.roll_down(lst_len)?;
                                }
                                // byte/short/int/long/float/double
                                nbt_consts::TAG_BYTE => {
                                    let lst_values = (0..lst_len)
                                        .map(|i| BorrowNbtValue::Byte(current_ptr + i))
                                        .collect::<Vec<BorrowNbtValue>>();
                                    values.push((
                                        value_name_start,
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
                                        value_name_start,
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
                                        value_name_start,
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
                                        value_name_start,
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
                                        value_name_start,
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
                                        value_name_start,
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
                                        let value_len = reader.read_be_i32()?;
                                        let value_ptr = reader.cursor;
                                        if value_len < 0 {
                                            return Err(NbtError::LenNegative(
                                                lst_type, value_len, value_ptr,
                                            ));
                                        }
                                        let value_len = value_len as usize;
                                        let value = BorrowNbtValue::ByteArray(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len)?;
                                    }
                                    values.push((
                                        value_name_start,
                                        value_name_len,
                                        BorrowNbtValue::List(
                                            value_ptr, lst_len, lst_type, lst_values,
                                        ),
                                    ));
                                }
                                nbt_consts::TAG_INT_ARRAY => {
                                    let mut lst_values = Vec::with_capacity(lst_len);
                                    for _ in 0..lst_len {
                                        let value_len = reader.read_be_i32()?;
                                        let value_ptr = reader.cursor;
                                        if value_len < 0 {
                                            return Err(NbtError::LenNegative(
                                                lst_type, value_len, value_ptr,
                                            ));
                                        }
                                        let value_len = value_len as usize;
                                        let value = BorrowNbtValue::IntArray(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len * 4)?;
                                    }
                                    values.push((
                                        value_name_start,
                                        value_name_len,
                                        BorrowNbtValue::List(
                                            value_ptr, lst_len, lst_type, lst_values,
                                        ),
                                    ));
                                }
                                nbt_consts::TAG_LONG_ARRAY => {
                                    let mut lst_values = Vec::with_capacity(lst_len);
                                    for _ in 0..lst_len {
                                        let value_len = reader.read_be_i32()?;
                                        let value_ptr = reader.cursor;
                                        if value_len < 0 {
                                            return Err(NbtError::LenNegative(
                                                lst_type, value_len, value_ptr,
                                            ));
                                        }
                                        let value_len = value_len as usize;
                                        let value = BorrowNbtValue::LongArray(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len * 8)?;
                                    }
                                    values.push((
                                        value_name_start,
                                        value_name_len,
                                        BorrowNbtValue::List(
                                            value_ptr, lst_len, lst_type, lst_values,
                                        ),
                                    ));
                                }
                                nbt_consts::TAG_STRING => {
                                    let mut lst_values = Vec::with_capacity(lst_len);
                                    for _ in 0..lst_len {
                                        let value_len = reader.read_be_u16()? as usize;
                                        let value_ptr = reader.cursor;
                                        let value = BorrowNbtValue::String(value_ptr, value_len);
                                        lst_values.push(value);
                                        reader.roll_down(value_len)?;
                                    }
                                    values.push((
                                        value_name_start,
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
                            values.push((value_name_start, value_name_len, value));
                            let last = values.last_mut().unwrap();
                            read_stack.push(&mut last.2);
                            break;
                        }
                        nbt_consts::TAG_END => {
                            unreachable!("前面处理过了")
                        }
                        _ => {
                            return Err(NbtError::UnknownType(value_type_id, reader.cursor));
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
                // println!(
                //     "list type: {}, len: {}, cursor:\n{}",
                //     lst_type.as_nbt_type_name(),
                //     lst_len,
                //     reader.show_cursor_fancy(None)
                // );
                match *lst_type {
                    nbt_consts::TAG_LIST => {
                        // 读取子 list 的类型
                        let sub_lst_type = reader.read_u8()?;
                        if !sub_lst_type.is_valid_nbt_type() {
                            return Err(NbtError::UnknownType(sub_lst_type, reader.cursor));
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
                            let sub_lst = Vec::with_capacity(sub_lst_len);
                            let value = BorrowNbtValue::List(
                                reader.cursor,
                                sub_lst_len,
                                sub_lst_type,
                                sub_lst,
                            );
                            values.push(value);
                            read_stack.push(values.last_mut().unwrap());
                            continue;
                        }
                        let current_ptr = reader.cursor;

                        // 可直接读取的类型
                        match sub_lst_type {
                            nbt_consts::TAG_END => {
                                // 真有
                                let lst_0 = Vec::with_capacity(0);
                                values.push(BorrowNbtValue::List(
                                    current_ptr,
                                    sub_lst_len,
                                    sub_lst_type,
                                    lst_0,
                                ));
                                reader.roll_down(sub_lst_len)?;
                            }
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
                                    let value_len = reader.read_be_i32()?;
                                    let value_ptr = reader.cursor;
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
                                    let value_len = reader.read_be_i32()?;
                                    let value_ptr = reader.cursor;
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
                                    let value_len = reader.read_be_i32()?;
                                    let value_ptr = reader.cursor;
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
                            nbt_consts::TAG_STRING => {
                                let mut lst_values = Vec::with_capacity(sub_lst_len);
                                for _ in 0..sub_lst_len {
                                    let value_len = reader.read_be_u16()? as usize;
                                    let value_ptr = reader.cursor;
                                    let value = BorrowNbtValue::String(value_ptr, value_len);
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
                            _ => unreachable!("其他的都预处理过了"),
                        }
                    }
                    nbt_consts::TAG_COMPOUND => {
                        let value_ptr = reader.cursor;
                        // 非 root 的 Compound
                        let value = BorrowNbtValue::Compound(value_ptr, None, vec![]);
                        values.push(value);
                        let last = values.last_mut().unwrap();
                        read_stack.push(last);
                        continue;
                    }
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
