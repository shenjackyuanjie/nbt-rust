#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::mutf8::Mutf8String;
use crate::NbtError;

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
    String(Mutf8String),
    /// 9
    /// 长度: i32
    List(Vec<NbtValue>),
    /// 10
    Compound(Option<Mutf8String>, Vec<(Mutf8String, NbtValue)>),
    /// 11
    /// 长度: i32
    IntArray(Vec<i32>),
    /// 12
    /// 长度: i32
    LongArray(Vec<i64>),
}

impl NbtValue {
    /// 检验所有的 mut8 字符串 是否合法
    pub fn verify_strings(&self) -> Option<Vec<NbtError>> {
        let mut errors = Vec::new();
        match self {
            NbtValue::List(list) => {
                for value in list {
                    value.inner_verify_strings(&mut errors);
                }
            }
            NbtValue::Compound(n, values) => {
                if let Some(name) = n {
                    if let Some(e) = name.verify() {
                        errors.push(e.into());
                    }
                }
                for (name, value) in values {
                    if let Some(e) = name.verify() {
                        errors.push(e.into());
                    }
                    value.inner_verify_strings(&mut errors);
                }
            }
            NbtValue::String(s) => {
                if let Some(e) = s.verify() {
                    errors.push(e.into());
                }
            }
            _ => (),
        };
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    /// 生成一个 true
    pub fn value_true() -> Self {
        NbtValue::Byte(1)
    }

    /// 生成一个 false
    pub fn value_false() -> Self {
        NbtValue::Byte(0)
    }

    /// 内部实际传递的函数
    fn inner_verify_strings(&self, errors: &mut Vec<NbtError>) {
        match self {
            NbtValue::String(s) => {
                if let Some(e) = s.verify() {
                    errors.push(e.into());
                }
            }
            NbtValue::List(list) => {
                for value in list {
                    value.inner_verify_strings(errors);
                }
            }
            NbtValue::Compound(name, list) => {
                if let Some(n) = name {
                    if let Some(e) = n.verify() {
                        errors.push(e.into());
                    }
                }
                for (name, value) in list {
                    if let Some(e) = name.verify() {
                        errors.push(e.into());
                    }
                    value.inner_verify_strings(errors);
                }
            }
            _ => (),
        }
    }
}
