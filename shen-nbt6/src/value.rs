#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Nbt Value!
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NbtValue {
    // end: 0
    /// 1: Byte
    Byte(i8),
    /// 2
    Short(i16),
    /// 3
    Int(i32),
    /// 4
    Long(i64),
    /// 5
    Float(f32),
    /// 6
    Double(f64),
    /// 7
    /// 长度: i32
    ByteArray(Vec<i8>),
    /// 8
    /// 或者叫 u8 array
    /// 长度: u16
    String(String),
    /// 9
    /// 长度: i32
    List(Vec<NbtValue>),
    /// 10
    Compound(Option<String>, Vec<(String, NbtValue)>),
    /// 11
    /// 长度: i32
    IntArray(Vec<i32>),
    /// 12
    /// 长度: i32
    LongArray(Vec<i64>),
}

impl NbtValue {
    /// 检验所有的 mut8 字符串 是否合法
    pub fn verify_strings(&self) -> bool {
        match self {
            NbtValue::String(_) => true,
            NbtValue::List(v) => v.iter().all(|v| v.verify_strings()),
            NbtValue::Compound(_, v) => v.iter().all(|(_, v)| v.verify_strings()),
            _ => false,
        }
    }
}
