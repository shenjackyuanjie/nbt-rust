use crate::traits::NbtBorrowTrait;
use crate::{nbt_consts, nbt_versions, NbtError, NbtReader, NbtResult, NbtTypeId};

/// 这里的所有 usize 实际上都指向一个 &[u8]
///
/// 用于更快速的解析 Nbt 数据
///
/// 所有 usize 都指向对应数据的开始位置
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
                        let value_name_len = reader.read_be_u16()? as usize;
                        // 跳过 name
                        reader.roll_down(value_name_len)?;
                        match value_type_id {
                            nbt_consts::TAG_END => {
                                // 读取到了 TAG_END, 表示当前 Compound 结束
                                // 弹出栈顶 (当前 Compound)
                                read_stack.pop();
                                break;
                            }
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
                            nbt_consts::TAG_BYTE_ARRAY => {
                                // 读取 ByteArray
                                let value_ptr = reader.cursor;
                                // 长度是 i32
                                let value_len = reader.read_be_i32()?;
                                let value =
                                    BorrowNbtValue::ByteArray(value_ptr, value_len as usize);
                                values.push((value_name_len, value));
                                // 移动 cursor
                                reader.roll_down(value_len as usize)?;
                            }
                            nbt_consts::TAG_INT_ARRAY => {
                                let value_ptr = reader.cursor;
                                let value_len = reader.read_be_i32()?;
                                let value = BorrowNbtValue::IntArray(value_ptr, value_len as usize);
                                values.push((value_name_len, value));
                                reader.roll_down(value_len as usize * 4)?;
                            }
                            nbt_consts::TAG_LONG_ARRAY => {
                                let value_ptr = reader.cursor;
                                let value_len = reader.read_be_i32()?;
                                let value = BorrowNbtValue::LongArray(value_ptr, value_len as usize);
                                values.push((value_name_len, value));
                                reader.roll_down(value_len as usize * 8)?;
                            }
                            nbt_consts::TAG_STRING => {
                                let value_ptr = reader.cursor;
                                let value_len = reader.read_be_u16()? as usize;
                                let value = BorrowNbtValue::String(value_ptr, value_len);
                                values.push((value_name_len, value));
                                reader.roll_down(value_len)?;
                            }
                            nbt_consts::TAG_LIST => {
                                let value_ptr = reader.cursor;
                                let lst_type = reader.read_u8()?;
                                let lst_len = reader.read_be_i32()?;
                                if lst_len < 0 {
                                    return Err(NbtError::ListLenNegative(lst_len as i32));
                                }
                                let value = BorrowNbtValue::List(value_ptr, lst_len as usize, lst_type, vec![]);
                                values.push((value_name_len, value));
                            }
                            nbt_consts::TAG_COMPOUND => {
                                let value_ptr = reader.cursor;
                                // 非 root 的 Compound 
                                let value = BorrowNbtValue::Compound(value_ptr, None, vec![]);
                                values.push((value_name_len, value));
                            }
                            _ => {
                                return Err(NbtError::UnknownType(value_type_id));
                            }
                        }
                    }
                }
                BorrowNbtValue::List(_, lst_len, type_id, values) => {
                    // 先看看是不是已经读过至少一个了
                    // 如果是, 则直接根据上一个的类型来读取
                    if values.is_empty() {
                        // 看来没读呢
                        // 先读取一个类型
                        let list_type_id = reader.read_u8()?;
                        // let list_len =
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
