use crate::data::{NbtItem, NbtLength, NbtList, NbtValue, Reader};
use std::cell::RefCell;
use std::convert::{From, Into};
use std::io::{Cursor, Read};
use std::rc::Rc;

/// 输出类型标识符
/// 类型标识符
/// (0x07) Vec<bool>
/// (0x08) UTF-8 String (Arc<str>)
/// (0x09) Vec<NbtItem>
/// (0x0A) Compound <xxxx>
/// (0x0B) Vec<i32>
/// (0x0C) Vec<i64>
pub mod read {
    use crate::data::{NbtItem, NbtLength, NbtList, NbtValue, Reader};
    use std::cell::RefCell;
    use std::io::Read;
    use std::rc::Rc;

    /// 直接读取长度和值 不带名称
    /// 反正名字都在外面读过
    #[inline]
    pub fn from_bool_array(value: &mut Reader) -> Vec<bool> {
        // 读取长度
        let mut buff = [0_u8; 4];
        _ = value.read(&mut buff).unwrap();
        let len = NbtLength::from_be_bytes(buff);
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(NbtValue::from_bool(value).as_bool().unwrap());
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
    pub fn read_nbt_list(value: &mut Reader) -> Vec<NbtItem> {
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
                todo!()
            }
            [0x01] => {
                for _ in 0..len {
                    vec.push(NbtItem::Value(NbtValue::from_bool(value)));
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
                    let arr = Rc::new(RefCell::new(from_bool_array(value)));
                    vec.push(NbtItem::Array(NbtList::BoolArray(arr)));
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
            }
            [0x0A] => {
                // Compound
                // 他甚至不告诉你有多少个元素，要命
            }
            [0x0B] => {
                // IntArray
                for _ in 0..len {
                    let arr = Rc::new(RefCell::new(from_i32_array(value)));
                    vec.push(NbtItem::Array(NbtList::IntArray(arr)));
                }
            }
            [0x0C] => {
                // LongArray
                for _ in 0..len {
                    let arr = Rc::new(RefCell::new(from_i64_array(value)));
                    vec.push(NbtItem::Array(NbtList::LongArray(arr)));
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
}

/// NbtItem
/// 完整的读取逻辑就在这里了
/// 来力
#[allow(unused)]
impl From<Cursor<&[u8]>> for NbtItem {
    /// 完整逻辑~
    fn from(value: Reader) -> NbtItem { todo!() }
}
