use std::fmt::Display;

use crate::{NbtTypeId, NbtValue};

/// 把 u8 转换成对应的 Nbt 类型名称
pub trait NbtTypeConversion {
    /// 把 u8 转换成对应的 Nbt 类型名称
    fn as_nbt_type_name(&self) -> String;
}

impl NbtTypeConversion for NbtTypeId {
    fn as_nbt_type_name(&self) -> String {
        if *self > 12 {
            return format!("未知类型({})", *self);
        }
        match *self {
            0 => "NBT_End(0)",
            1 => "NBT_Byte(1)",
            2 => "NBT_Short(2)",
            3 => "NBT_Int(3)",
            4 => "NBT_Long(4)",
            5 => "NBT_Float(5)",
            6 => "NBT_Double(6)",
            7 => "NBT_ByteArray(7)",
            8 => "NBT_String(8)",
            9 => "NBT_List(9)",
            10 => "NBT_Compound(10)",
            11 => "NBT_IntArray(11)",
            12 => "NBT_LongArray(12)",
            _ => unreachable!(),
        }
        .to_string()
    }
}

/// 输出 SNBT
/// 这里的格式是为了方便阅读
/// 更接近客户端里实际的格式(命令里使用的格式)
impl Display for NbtValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtValue::Byte(v) => write!(f, "{}b", v),
            NbtValue::Short(v) => write!(f, "{}s", v),
            NbtValue::Int(v) => write!(f, "{}", v),
            NbtValue::Long(v) => write!(f, "{}l", v),
            // float 后面跟一个 f
            NbtValue::Float(v) => write!(f, "{}f", v),
            // double 后面跟一个 d?
            NbtValue::Double(v) => write!(f, "{}d", v),
            NbtValue::ByteArray(v) => {
                write!(f, "[B; ")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}b", v)?;
                }
                write!(f, "]")
            }
            NbtValue::IntArray(v) => {
                write!(f, "[I; ")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            NbtValue::LongArray(v) => {
                write!(f, "[L; ")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}l", v)?;
                }
                write!(f, "]")
            },
            NbtValue::String(v) => {
                write!(f, "\"{}\"", v.decode())
            },
            NbtValue::List(lst) => {
                write!(f, "[")?;
                for (i, v) in lst.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            },
            NbtValue::Compound(name, map) => {
                if let Some(name) = name {
                    write!(f, "{}: {{", name.decode())?;
                } else {
                    write!(f, "{{")?;
                }
                for (i, (k, v)) in map.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k.decode(), v)?;
                }
                write!(f, "}}")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispaly_nbt_common() {
        let nbt = NbtValue::Compound(None, vec![
            ("byte".into(), NbtValue::Byte(1)),
            ("short".into(), NbtValue::Short(2)),
            ("int".into(), NbtValue::Int(3)),
            ("long".into(), NbtValue::Long(4)),
            ("float".into(), NbtValue::Float(5.0)),
            ("double".into(), NbtValue::Double(6.0)),
            ("byte_array".into(), NbtValue::ByteArray(vec![1, 2, 3])),
            ("int_array".into(), NbtValue::IntArray(vec![1, 2, 3])),
            ("long_array".into(), NbtValue::LongArray(vec![1, 2, 3])),
            ("string".into(), NbtValue::String("test".into()))
            ]);
        let str = format!("{}", nbt);
        assert_eq!(str, "{byte: 1b, short: 2s, int: 3, long: 4l, float: 5f, double: 6d, byte_array: [B; 1b, 2b, 3b], int_array: [I; 1, 2, 3], long_array: [L; 1l, 2l, 3l], string: \"test\"}");
    }

    #[test]
    fn display_byte_array() {
        let nbt = NbtValue::ByteArray(vec![1, 2, 3]);
        let str = format!("{}", nbt);
        assert_eq!(str, "[B; 1b, 2b, 3b]");
    }

    #[test]
    fn display_int_array() {
        let nbt = NbtValue::IntArray(vec![1, 2, 3]);
        let str = format!("{}", nbt);
        assert_eq!(str, "[I; 1, 2, 3]");
    }

    #[test]
    fn display_long_array() {
        let nbt = NbtValue::LongArray(vec![1, 2, 3]);
        let str = format!("{}", nbt);
        assert_eq!(str, "[L; 1l, 2l, 3l]");
    }
}