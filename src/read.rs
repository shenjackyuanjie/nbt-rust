use crate::data::{NbtItem, NbtList, NbtValue, Reader};
use std::convert::From;
use std::io::{BufRead, Cursor, Read};

/// 输出类型标识符
/// 类型标识符
/// (0x07) Vec<bool>
/// (0x08) UTF-8 String (Arc<str>)
/// (0x09) Vec<NbtItem>
/// (0x0A) Compound <xxxx>
/// (0x0B) Vec<i32>
/// (0x0C) Vec<i64>
pub mod read_data {
    use crate::data::{NbtItem, NbtLength, NbtList, NbtValue, Reader};
    use std::collections::HashMap;
    use std::io::Read;
    use std::sync::Arc;

    /// 直接读取长度和值 不带名称
    /// 反正名字都在外面读过
    #[inline]
    pub fn from_i8_array(value: &mut Reader) -> Vec<i8> {
        // 读取长度
        let mut buff = [0_u8; 4];
        _ = value.read(&mut buff).unwrap();
        let len = NbtLength::from_be_bytes(buff);
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(NbtValue::from_i8(value).as_i8().unwrap());
        }
        vec
    }

    /// 直接读取长度和值 不带名称
    #[inline]
    pub fn from_i32_array(value: &mut Reader) -> Vec<i32> {
        // 读取长度
        let mut buff = [0_u8; 4];
        _ = value.read(&mut buff).unwrap();
        let len = NbtLength::from_be_bytes(buff);
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(NbtValue::from_i32(value).as_i32().unwrap());
        }
        vec
    }

    /// 直接读取长度和值 不带名称
    #[inline]
    pub fn from_i64_array(value: &mut Reader) -> Vec<i64> {
        // 读取长度
        let mut buff = [0_u8; 4];
        _ = value.read(&mut buff).unwrap();
        let len = NbtLength::from_be_bytes(buff);
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(NbtValue::from_i64(value).as_i64().unwrap());
        }
        vec
    }

    /// 直接读取长度和值 不带名称
    /// 主要是为了可以直接递归 (
    pub fn from_nbt_list(value: &mut Reader) -> Vec<NbtItem> {
        // 读取长度
        let mut buff = [0_u8; 4];
        _ = value.read(&mut buff).unwrap();
        let len = NbtLength::from_be_bytes(buff);
        let mut vec: Vec<NbtItem> = Vec::with_capacity(len as usize);
        // 先读取 type
        let mut type_buff = [0_u8; 1];
        _ = value.read(&mut type_buff).unwrap();
        match type_buff {
            [0x00] => {
                // End
                todo!()
            }
            [0x01] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_i8(value)));
                }
            }
            [0x02] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_i16(value)));
                }
            }
            [0x03] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_i32(value)));
                }
            }
            [0x04] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_i64(value)));
                }
            }
            [0x05] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_f32(value)));
                }
            }
            [0x06] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_f64(value)));
                }
            }
            [0x07] => {
                // ByteArray
                for _ in 0..len {
                    vec.push(NbtItem::from(from_i8_array(value)));
                }
            }
            [0x08] => {
                // string
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_string(value)));
                }
            }
            [0x09] => {
                // NbtList
                // 要命 (虽说没 Compound 那么麻烦)
                // 直接递归就行
                for _ in 0..len {
                    vec.push(NbtItem::from(from_nbt_list(value)));
                }
            }
            [0x0A] => {
                // Compound
                // 他甚至不告诉你有多少个元素，要命
                for _ in 0..len {
                    vec.push(NbtItem::from(from_compound(value)));
                }
            }
            [0x0B] => {
                // IntArray
                for _ in 0..len {
                    vec.push(NbtItem::from(from_i32_array(value)));
                }
            }
            [0x0C] => {
                // LongArray
                for _ in 0..len {
                    vec.push(NbtItem::from(from_i64_array(value)));
                }
            }
            _ => {
                panic!(
                    "{}",
                    format!(
                        "WTF while reading Nbt List \ntype: {:?}\nreader pos: {:?}",
                        type_buff,
                        value.position()
                    )
                );
            }
        }
        vec
    }

    /// 直接读取长度和值 不带名称
    /// 只不过 Compound 不带长度，得自己试
    pub fn from_compound(value: &mut Reader) -> NbtList {
        // 进来直接是 values
        // loop 读取长度 name len name value value
        // 直到一个 End
        #[cfg(feature = "core_debug")]
        println!("compound at {} bytes", value.position());
        let mut map: HashMap<Arc<str>, NbtItem> = HashMap::new();
        loop {
            let mut type_tag = [0_u8; 1];
            _ = value.read(&mut type_tag).unwrap();
            if type_tag == [0x00] {
                // End
                break;
            }
            // 读取 name
            // 直接调之前的方法读
            let name = NbtValue::from_string(value).as_string().unwrap();
            #[cfg(feature = "core_debug")]
            println!("compound type tag {:?} with name: {:?}", type_tag, name);
            // 读取 value
            let nbt_value: NbtItem = match type_tag {
                [0x01] => NbtItem::Value(NbtValue::from_i8(value)),
                [0x02] => NbtItem::Value(NbtValue::from_i16(value)),
                [0x03] => NbtItem::Value(NbtValue::from_i32(value)),
                [0x04] => NbtItem::Value(NbtValue::from_i64(value)),
                [0x05] => NbtItem::Value(NbtValue::from_f32(value)),
                [0x06] => NbtItem::Value(NbtValue::from_f64(value)),
                [0x07] => NbtItem::from(from_i8_array(value)),
                [0x08] => NbtItem::Value(NbtValue::from_string(value)),
                [0x09] => NbtItem::from(from_nbt_list(value)),
                [0x0A] => {
                    let item = match from_compound(value) {
                        NbtList::Compound(mut get_name, item) => {
                            get_name = name.clone();
                            NbtList::Compound(get_name, item)
                        }
                        _ => panic!("WTF"),
                    };
                    NbtItem::from(item)
                }
                [0x0B] => NbtItem::from(from_i32_array(value)),
                [0x0C] => NbtItem::from(from_i64_array(value)),
                _ => {
                    panic!(
                        "{}",
                        format!(
                            "WTF while reading Nbt Compound \ntype: {:?}\nreader pos: {:?}\nname: {:?}",
                            type_tag,
                            value.position(),
                            name
                        )
                    )
                }
            };
            // 读取完了，放进去
            map.insert(name, nbt_value);
            #[cfg(feature = "core_debug")]
            println!("compound: {:?}", map);
        }
        NbtList::from((Arc::from(""), map))
    }
}

use read_data::{from_compound, from_i32_array, from_i64_array, from_i8_array, from_nbt_list};

pub enum NbtStatus {
    /// 读取到了 End
    End,
    /// 继续中
    Going(NbtItem),
    /// 读取错误
    Error(std::io::Error),
}

/// NbtItem
/// 完整的读取逻辑就在这里了
/// 来力
impl TryFrom<Cursor<&[u8]>> for NbtItem {
    type Error = std::io::Error;

    /// 完整逻辑~
    fn try_from(in_value: Reader) -> Result<NbtItem, Self::Error> {
        let mut value: Reader = in_value.clone();
        let mut items: Vec<NbtItem> = Vec::new();
        #[cfg(feature = "debug")]
        println!("reader pos: {:?}", value.position());
        loop {
            // 读取类型
            let mut buff = [0_u8; 1];
            _ = value.read(&mut buff).unwrap();
            let name = NbtValue::from_string(&mut value).as_string().unwrap();
            let type_code: NbtStatus = match buff {
                [0x00] => NbtStatus::End,
                [0x01] => NbtStatus::Going(NbtItem::Value(NbtValue::from_i8(&mut value))),
                [0x02] => NbtStatus::Going(NbtItem::Value(NbtValue::from_i16(&mut value))),
                [0x03] => NbtStatus::Going(NbtItem::Value(NbtValue::from_i32(&mut value))),
                [0x04] => NbtStatus::Going(NbtItem::Value(NbtValue::from_i64(&mut value))),
                [0x05] => NbtStatus::Going(NbtItem::Value(NbtValue::from_f32(&mut value))),
                [0x06] => NbtStatus::Going(NbtItem::Value(NbtValue::from_f64(&mut value))),
                [0x07] => NbtStatus::Going(NbtItem::from(from_i8_array(&mut value))),
                [0x08] => NbtStatus::Going(NbtItem::Value(NbtValue::from_string(&mut value))),
                [0x09] => NbtStatus::Going(NbtItem::from(from_nbt_list(&mut value))),
                [0x0A] => NbtStatus::Going(NbtItem::from(from_compound(&mut value))),
                [0x0B] => NbtStatus::Going(NbtItem::from(from_i32_array(&mut value))),
                [0x0C] => NbtStatus::Going(NbtItem::from(from_i64_array(&mut value))),
                _ => NbtStatus::Error(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "WTF while reading Nbt Item \ntype: {:?}\nreader pos: {:?}",
                        buff,
                        value.position()
                    ),
                )),
            };
            #[cfg(feature = "core_debug")]
            println!("==type_code: {:?} reader pos: {:?}", buff, value.position());
            match type_code {
                NbtStatus::End => {
                    break;
                }
                NbtStatus::Going(item) => {
                    let item = match item {
                        NbtItem::Array(NbtList::Compound(mut get_name, map)) => {
                            get_name = name;
                            NbtItem::Array(NbtList::Compound(get_name, map))
                        }
                        _ => item,
                    };
                    items.push(item);
                    if !value.has_data_left().unwrap() {
                        break;
                    }
                }
                NbtStatus::Error(e) => {
                    return Err(e);
                }
            }
        }
        // 理论上 长度应该为 2
        if items.len() >= 3 {
            Ok(NbtItem::Array(NbtList::from(items)))
        } else {
            Ok(items[0].clone())
        }
    }
}
