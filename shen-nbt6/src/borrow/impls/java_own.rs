use crate::borrow::BorrowNbtValue as BValue;
use crate::{Mutf8String, NbtReader, NbtValue};

/// 把一个 borrow value 转换成 owned value
///
/// SAFETY: 请确保 这里的 value 可以对应上 data
pub fn own_value(value: &BValue, reader: &mut NbtReader) -> NbtValue {
    // 先把 reader 指针移动到头
    reader.roll_top();

    let root_value = value.as_compound_idx().unwrap();
    let root_name = root_value
        .1
        .map(|name_len| Mutf8String::from_reader(reader, root_value.0 + 3, name_len).unwrap());
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
                let writing_value = match write_value {
                    NbtValue::Compound(_, values) => values,
                    _ => unreachable!("parse stack 和 write stack 的类型一致"),
                };
                if writing_value.len() == values.len() {
                    // 如果写入的长度和解析的长度一致, 说明这个 compound 已经解析完了
                    parse_stack.pop();
                    write_stack.pop();
                    continue;
                }
                let reading_value = values.get(writing_value.len()).unwrap();
                let name_start = reading_value.0;
                let name_len = reading_value.1;
                let value_start = reading_value.2.start_idx();
                // 额, 才发现我需要手动算一下 name 的起始位置
                // 倒也无所谓吧
                // UNWRAP safety: 这里的 name_len 是从 values 里面取出来的, 所以不会越界
                let value_name = Mutf8String::from_reader(reader, name_start, name_len).unwrap();
                // 以防万一?
                // 把 reader 指针移动到 value 的开始位置
                let _ = reader.roll_to(value_start);
                unsafe {
                    match reading_value.2 {
                        BValue::Byte(_) => {
                            writing_value
                                .push((value_name, NbtValue::Byte(reader.read_i8().unwrap())));
                        }
                        BValue::Short(_) => {
                            writing_value
                                .push((value_name, NbtValue::Short(reader.read_be_i16_unsafe())));
                        }
                        BValue::Int(_) => {
                            writing_value
                                .push((value_name, NbtValue::Int(reader.read_be_i32_unsafe())));
                        }
                        BValue::Long(_) => {
                            writing_value
                                .push((value_name, NbtValue::Long(reader.read_be_i64_unsafe())));
                        }
                        BValue::Float(_) => {
                            writing_value
                                .push((value_name, NbtValue::Float(reader.read_be_f32_unsafe())));
                        }
                        BValue::Double(_) => {
                            writing_value
                                .push((value_name, NbtValue::Double(reader.read_be_f64_unsafe())));
                        }
                        BValue::ByteArray(_, len) => {
                            let data = reader.read_i8_array_unsafe(len);
                            writing_value.push((value_name, NbtValue::ByteArray(data)));
                        }
                        BValue::IntArray(_, len) => {
                            let data = reader.read_be_i32_array_unsafe(len);
                            writing_value.push((value_name, NbtValue::IntArray(data)));
                        }
                        BValue::LongArray(_, len) => {
                            let data = reader.read_be_i64_array_unsafe(len);
                            writing_value.push((value_name, NbtValue::LongArray(data)));
                        }
                        BValue::String(str_start, len) => {
                            let data = Mutf8String::from_reader(reader, str_start, len).unwrap();
                            writing_value.push((value_name, NbtValue::String(data)));
                        }
                        _ => {
                            todo!()
                        }
                    }
                }
            }
            BValue::List(_, len, type_id, values) => {
                todo!()
            }
            _ => unreachable!("解析的时候不会把非 list/compond 的东西放进来"),
        }
    }

    root_element
}
