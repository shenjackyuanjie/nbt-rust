use crate::borrow::BorrowNbtValue as BValue;
use crate::{Mutf8String, NbtReader, NbtValue, RECURSE_LIMIT};

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
    let root_vec = Vec::with_capacity(root_value.2.len());
    let mut root_element = NbtValue::Compound(root_name, root_vec);

    // 两个 FILO 栈用来解析
    // 解析栈
    let mut parse_stack = Vec::with_capacity(RECURSE_LIMIT);
    parse_stack.push(value);
    // 写入栈
    let mut write_stack: Vec<&mut NbtValue> = Vec::with_capacity(RECURSE_LIMIT);
    write_stack.push(&mut root_element);

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
                    match &reading_value.2 {
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
                            let data = reader.read_i8_array_unsafe(*len);
                            writing_value.push((value_name, NbtValue::ByteArray(data)));
                        }
                        BValue::IntArray(_, len) => {
                            let data = reader.read_be_i32_array_unsafe(*len);
                            writing_value.push((value_name, NbtValue::IntArray(data)));
                        }
                        BValue::LongArray(_, len) => {
                            let data = reader.read_be_i64_array_unsafe(*len);
                            writing_value.push((value_name, NbtValue::LongArray(data)));
                        }
                        BValue::String(str_start, len) => {
                            let data = Mutf8String::from_reader(reader, *str_start, *len).unwrap();
                            writing_value.push((value_name, NbtValue::String(data)));
                        }
                        BValue::Compound(name_start, name_len, inner_values) => {
                            let new_vec = Vec::with_capacity(inner_values.len());
                            let new_name = name_len.map(|name_len| {
                                Mutf8String::from_reader(reader, *name_start, name_len).unwrap()
                            });
                            let new_value = NbtValue::Compound(new_name, new_vec);
                            // 入栈
                            writing_value.push((value_name, new_value));
                            if inner_values.is_empty() {
                                // 如果是空的, 说明是空的 compound, 不需要解析
                                continue;
                            }
                            let new_value = writing_value.last_mut().unwrap();
                            write_stack.push(&mut new_value.1); // 代码还是常看常新啊
                            parse_stack.push(&reading_value.2);
                            // 继续解析
                            continue;
                        }
                        BValue::List(_, lst_len, _, _) => {
                            let new_vec = Vec::with_capacity(*lst_len);
                            let new_value = NbtValue::List(new_vec);
                            // 入栈 ( 反正都读完了, 肯定保证不会出现过深的情况 )
                            writing_value.push((value_name, new_value));
                            if *lst_len == 0 {
                                // 如果是空的, 说明是空的 list, 不需要解析
                                continue;
                            }
                            let new_value = writing_value.last_mut().unwrap();
                            write_stack.push(&mut new_value.1);
                            parse_stack.push(&reading_value.2);
                            continue;
                        }
                    }
                }
            }
            BValue::List(_, len, _, values) => {
                let writing_value = match write_value {
                    NbtValue::List(values) => values,
                    _ => unreachable!("parse stack 和 write stack 的类型一致"),
                };
                if writing_value.len() == *len {
                    // 如果写入的长度和解析的长度一致, 说明这个 list 已经解析完了
                    parse_stack.pop();
                    write_stack.pop();
                    continue;
                }
                let reading_value = values.get(writing_value.len()).unwrap();
                unsafe {
                    match reading_value {
                        BValue::Byte(ptr) => {
                            // 读一大堆 byte
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_i8_array_unsafe(*len);
                            for byte in data {
                                writing_value.push(NbtValue::Byte(byte));
                            }
                            // 解析完了, 出栈
                            parse_stack.pop();
                            write_stack.pop();
                            continue;
                        }
                        BValue::Short(ptr) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_i16_array_unsafe(*len);
                            for short in data {
                                writing_value.push(NbtValue::Short(short));
                            }
                            // 解析完了, 出栈
                            parse_stack.pop();
                            write_stack.pop();
                            continue;
                        }
                        BValue::Int(ptr) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_i32_array_unsafe(*len);
                            for int in data {
                                writing_value.push(NbtValue::Int(int));
                            }
                            // 解析完了, 出栈
                            parse_stack.pop();
                            write_stack.pop();
                            continue;
                        }
                        BValue::Long(ptr) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_i64_array_unsafe(*len);
                            for long in data {
                                writing_value.push(NbtValue::Long(long));
                            }
                            // 解析完了, 出栈
                            parse_stack.pop();
                            write_stack.pop();
                            continue;
                        }
                        BValue::Float(ptr) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_f32_array_unsafe(*len);
                            for float in data {
                                writing_value.push(NbtValue::Float(float));
                            }
                            // 解析完了, 出栈
                            parse_stack.pop();
                            write_stack.pop();
                            continue;
                        }
                        BValue::Double(ptr) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_f64_array_unsafe(*len);
                            for double in data {
                                writing_value.push(NbtValue::Double(double));
                            }
                            // 解析完了, 出栈
                            parse_stack.pop();
                            write_stack.pop();
                            continue;
                        }
                        // 懒得动了, 三 Array 就在大循环里一次一次读吧
                        BValue::ByteArray(ptr, len) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_i8_array_unsafe(*len);
                            writing_value.push(NbtValue::ByteArray(data));
                        }
                        BValue::IntArray(ptr, len) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_i32_array_unsafe(*len);
                            writing_value.push(NbtValue::IntArray(data));
                        }
                        BValue::LongArray(ptr, len) => {
                            let _ = reader.roll_to(*ptr);
                            let data = reader.read_be_i64_array_unsafe(*len);
                            writing_value.push(NbtValue::LongArray(data));
                        }
                        BValue::String(ptr, len) => {
                            let _ = reader.roll_to(*ptr);
                            let data = Mutf8String::from_reader(reader, *ptr, *len).unwrap();
                            writing_value.push(NbtValue::String(data));
                        }
                        BValue::List(_, sub_lst_len, _, _) => {
                            let new_vec = Vec::with_capacity(*sub_lst_len);
                            let new_value = NbtValue::List(new_vec);
                            writing_value.push(new_value);
                            if *sub_lst_len == 0 {
                                // 如果是空的, 说明是空的 list, 不需要解析
                                continue;
                            }
                            let new_value = writing_value.last_mut().unwrap();
                            write_stack.push(new_value);
                            parse_stack.push(reading_value);
                        }
                        BValue::Compound(_, _, sub_map) => {
                            let new_map = Vec::with_capacity(sub_map.len());
                            let new_value = NbtValue::Compound(None, new_map);
                            writing_value.push(new_value);
                            if sub_map.is_empty() {
                                // 如果是空的, 说明是空的 compound, 不需要解析
                                continue;
                            }
                            let new_value = writing_value.last_mut().unwrap();
                            write_stack.push(new_value);
                            parse_stack.push(reading_value);
                            continue;
                        }
                    }
                }
            }
            _ => unreachable!("解析的时候不会把非 list/compond 的东西放进来"),
        }
    }

    root_element
}
