use std::error::Error;
use std::fmt::Display;
use std::str::Utf8Error;

use crate::traits::NbtTypeConversion;
use crate::NbtTypeId;

/// Error
#[derive(Debug, Clone, PartialEq)]
pub enum NbtError {
    /// 未知错误
    UnknownErr(String),
    /// 根节点类型错误
    WrongRootType(NbtTypeId),
    /// 根节点无名称
    RootWithoutName,
    /// 未知类型
    UnknownType(NbtTypeId),
    /// 名称读取错误
    NameRead(String),
    /// 指针超出范围
    ///
    /// cursor, len, data.len()
    /// 三个参数分别表示
    /// - 当前指针
    /// - 数据长度
    /// - 数据总长度
    CursorOverflow(usize, usize, usize),
    /// Varint 过大
    VarIntTooBig(usize),
    /// Varlong 过大
    VarlongTooBig(usize),
    /// NbtList 中类型不同
    ListTypeNotSame(Vec<NbtTypeId>),
    /// NbtList/NbtArray 长度 < 0
    LenNegative(NbtTypeId, i32),
    /// 错误类型
    IncorrectType(NbtTypeId, NbtTypeId),
    /// m-utf8 解码错误
    Mutf8Error(Utf8Error),
    /// NBT 深度过大
    NbtDepthTooBig(usize),
}

impl Error for NbtError {}

impl Display for NbtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtError::UnknownErr(s) => write!(f, "未知错误: {}", s),
            NbtError::WrongRootType(n) => match n {
                9 => {
                    write!(
                        f,
                        "根节点为 NbtList(9) 类型, 是否应该使用 BedrockDisk/BedrockNetVarInt?"
                    )
                }
                _ => {
                    write!(f, "根节点类型错误: {}, 应为 NbtCompound/NbtList(bedrock only)", n)
                }
            },
            NbtError::RootWithoutName => {
                write!(f, "根节点无名称, 是否应该使用 JavaNetAfter1_20_2 解析?")
            }
            NbtError::UnknownType(n) => {
                if *n == 0 {
                    write!(f, "未知类型: NBTEnd(0), 请检查数据是否正确")
                } else {
                    write!(f, "未知类型: {}", n)
                }
            }
            NbtError::NameRead(s) => write!(f, "名称读取错误: {}", s),
            NbtError::CursorOverflow(cursor, len, data_len) => write!(
                f,
                "指针超出范围: cursor: {}, len: {}, cursor+len: {}, data.len(): {}",
                cursor,
                len,
                cursor + len,
                data_len
            ),
            NbtError::VarIntTooBig(n) => write!(f, "VarInt 过大: {} 最大长度为 5", n),
            NbtError::VarlongTooBig(n) => write!(f, "VarLong 过大: {} 最大长度为 10", n),
            NbtError::ListTypeNotSame(types) => {
                write!(f, "NbtList 中类型不同: {:?} 应相同", types)
            }
            NbtError::LenNegative(type_id, len) => {
                write!(f, "{} 长度 < 0: {}", type_id.as_nbt_type_name(), len)
            }
            NbtError::IncorrectType(expect, got) => {
                write!(f, "错误类型: 期望: {}, 实际: {}", expect, got)
            }
            NbtError::Mutf8Error(e) => write!(f, "m-utf8 解码错误: {}", e),
            NbtError::NbtDepthTooBig(n) => write!(f, "NBT 深度过大, 仅支持 {} 深度", n),
        }
    }
}

impl From<Utf8Error> for NbtError {
    fn from(e: Utf8Error) -> Self { NbtError::Mutf8Error(e) }
}
