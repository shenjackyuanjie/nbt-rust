use crate::borrow::BorrowNbtValue as BValue;
use crate::{Mutf8String, NbtError, NbtReader, NbtResult, NbtValue};

/// 把一个 borrow value 转换成 owned value
/// 
/// SAFETY: 请确保 这里的 value 可以对应上 data
pub fn own_value(value: &BValue, reader: &mut NbtReader) -> NbtValue {
    // 先把 reader 指针移动到头
    reader.roll_top();

    let root_value = value.as_compound_idx().unwrap();
    let root_name = root_value
        .1
        .map(|name_len| Mutf8String::from_reader(reader, root_value.0, name_len).unwrap());
    let mut root_element = NbtValue::Compound(root_name, vec![]);

    // 两个 FILO 栈用来解析
    // 解析栈
    let mut parse_stack = vec![value];
    // 写入栈
    let mut write_stack = vec![&mut root_element];

    while !parse_stack.is_empty() {
        // 从解析栈中取出一个
        let parse_value = *parse_stack.last().unwrap();
        // 从写入栈中取出一个
        let write_value = write_stack.last().unwrap();
        let write_value = unsafe {
            // SAFETY: 这里跟 read 同理
            std::ptr::read(write_value)
        };
        match parse_value {
            BValue::Compound(_, _, values) => {
                let unwrap_write_value = match write_value {
                    NbtValue::Compound(_, values) => values,
                    _ => unreachable!("parse stack 和 write stack 的类型一致"),
                };

            }
            _ => unreachable!("解析的时候不会把非 list/compond 的东西放进来"),
        }
    }

    root_element
}
