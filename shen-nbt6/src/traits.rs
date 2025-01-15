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
            NbtValue::Byte(v) => write!(f, "{}", v),
            NbtValue::Short(v) => write!(f, "{}", v),
            NbtValue::Int(v) => write!(f, "{}", v),
            NbtValue::Long(v) => write!(f, "{}", v),
            // float 后面跟一个 f
            NbtValue::Float(v) => write!(f, "{}f", v),
            // double 后面跟一个 d?
            NbtValue::Double(v) => write!(f, "{}d", v),
            NbtValue::ByteArray(v) => {
                write!(f, "[")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            NbtValue::IntArray(v) => {
                write!(f, "[")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            NbtValue::LongArray(v) => {
                write!(f, "[")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
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
