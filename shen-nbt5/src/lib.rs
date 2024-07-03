//! shen-nbt5
//!
//! NBT 格式解析库 v5 by shenjack and InfyniteHeap
//!
//! 支持格式
//!
//! - Java 版 NBT
//!
//! - Java 1.20.2+(协议号 >= 764) 及以后 的网路传输 NBT 格式
//!
//! - 基岩版 实际用于存储的 NBT 格式
//!
//! - 基岩版 网络 NBT 格式
//!
//! 用例:
//! ```rust
//! use shen_nbt5::NbtValue;
//! use shen_nbt5::nbt_version::Java;
//!
//! fn main() {
//!    let mut data = vec![0x0A, 0x00, 0x0B, 0x68, 0x65,
//!        0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64,
//!        0x08, 0x00, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x09,
//!        0x42, 0x61, 0x6E, 0x61, 0x6E, 0x72, 0x61, 0x6D, 0x61, 0x00,
//!    ];
//!    let value = NbtValue::from_binary::<Java>(&mut data).unwrap();
//!    println!("{:?}", value);
//! }
//! ```

pub mod reader;
pub mod writer;

#[cfg(feature = "serde")]
pub mod ser;
#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use reader::NbtReader;

#[cfg(test)]
mod tests;

/// 不同版本的 Nbt 数据细节不同
/// 老要命了
///
/// - `Java`
///   Java 版除了 1.20.2+(协议号) 及以后的网络 NBT 格式
/// - `JavaNetAfter1_20_2`
///   1.20.2+(协议号 >= 764) 及以后的网络 NBT 格式
/// - `BedrockDisk`
///   基岩版 实际用于存储的 NBT 格式
/// - `BedrockNetVarInt`
///   基岩版 网络 NBT 格式
pub mod nbt_version {
    use super::{NbtReader, NbtResult, NbtValue};

    pub trait NbtWriteTrait {
        /// 写入一个 i8(byte) 数组
        fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]);
        /// 写入一个 i32(int) 数组
        fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]);
        /// 写入一个 i64(long) 数组
        fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]);
        /// 写入一个 NBT 字符串
        fn write_nbt_string(writer: &mut Vec<u8>, data: &str);
        /// 向 `writer` 写入一个列表类型(List)
        fn write_list(writer: &mut Vec<u8>, data: &[NbtValue]) -> NbtResult<()>;
        /// 向 `writer` 写入一个复合标签类型(Compound)
        ///
        /// 如果 `name` 为 `None` 则不写入名字
        fn write_compound(
            writer: &mut Vec<u8>,
            name: Option<&String>,
            data: &[(String, NbtValue)],
        ) -> NbtResult<()>;

        fn write_to(value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()>;
        fn write_to_with_name(name: &str, value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()>;

        fn to_bytes(value: &NbtValue) -> NbtResult<Vec<u8>> {
            let mut buff = Vec::new();
            Self::write_to(value, &mut buff)?;
            Ok(buff)
        }
    }

    pub trait NbtReadTrait {
        /// 从 `reader` 读取一个 i8(byte) 数组
        fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>>;
        /// 从 `reader` 读取一个 i32(int) 数组
        fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>>;
        /// 从 `reader` 读取一个 i64(long) 数组
        fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>>;
        /// 从 `reader` 读取一个 NBT 字符串
        fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String>;
        /// 从 `reader` 读取一个列表类型(List)
        fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>>;
        /// 从 `reader` 读取一个复合标签类型(Compound)
        fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>>;

        fn from_reader(reader: NbtReader) -> NbtResult<NbtValue>;
    }
    /// Java 版 绝大部分的 NBT 格式
    ///
    /// 除了 1.20.2+(协议号 >= 764) 及以后 的网路传输 NBT 格式 都是这个
    ///
    /// 上面说的那玩意 请使用 `JavaNetAfter1_20_2`
    ///
    /// # 编码特点
    ///
    /// 大端, 大端, 还是 xx 的 大端!
    pub enum Java {}
    /// 1.20.2+(协议号 >= 764) 及以后 的网路传输 NBT 格式
    ///
    /// # 编码特点
    ///
    /// 根节点没有名称
    pub enum JavaNetAfter1_20_2 {}
    /// 基岩版 实际用于存储的 NBT 格式
    ///
    /// # 编码特点
    ///
    /// 小端, 小端, 还是 xx 的 小端!
    pub enum BedrockDisk {}
    /// 基岩版 网络 NBT 格式
    /// 最痛苦的一集
    ///
    /// # 编码特点
    ///
    /// VarInt, VarLong, ZigZagVarInt, ZigZagVarLong
    /// 全都有
    pub enum BedrockNetVarInt {}
}

pub type NbtTypeId = u8;

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
    /// 错误类型
    IncorrectType(NbtTypeId, NbtTypeId),
}

/// 返回类型
pub type NbtResult<T> = std::result::Result<T, NbtError>;

impl std::error::Error for NbtError {}

impl std::fmt::Display for NbtError {
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
            NbtError::IncorrectType(expect, got) => {
                write!(f, "错误类型: 期望: {}, 实际: {}", expect, got)
            }
        }
    }
}

/// 核心 Value
///
/// 暂时不支持 `from_value` 和 `to_value`
/// https://github.com/shenjackyuanjie/nbt-rust/issues/1
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
    /// 解析 Nbt 数据
    pub fn from_binary<R>(data: &mut [u8]) -> NbtResult<NbtValue>
    where
        R: nbt_version::NbtReadTrait,
    {
        let reader = NbtReader::new(data);
        R::from_reader(reader)
    }

    pub fn tag(&self) -> NbtTypeId {
        match self {
            NbtValue::Byte(_) => 1,
            NbtValue::Short(_) => 2,
            NbtValue::Int(_) => 3,
            NbtValue::Long(_) => 4,
            NbtValue::Float(_) => 5,
            NbtValue::Double(_) => 6,
            NbtValue::ByteArray(_) => 7,
            NbtValue::String(_) => 8,
            NbtValue::List(_) => 9,
            NbtValue::Compound(_, _) => 10,
            NbtValue::IntArray(_) => 11,
            NbtValue::LongArray(_) => 12,
        }
    }

    pub fn write_to<W>(&self, buff: &mut Vec<u8>) -> NbtResult<()>
    where
        W: nbt_version::NbtWriteTrait,
    {
        W::write_to(self, buff)
    }

    pub fn write_to_with_name<W>(&self, name: &str, buff: &mut Vec<u8>) -> NbtResult<()>
    where
        W: nbt_version::NbtWriteTrait,
    {
        W::write_to_with_name(name, self, buff)
    }

    pub fn to_binary<W>(&self) -> NbtResult<Vec<u8>>
    where
        W: nbt_version::NbtWriteTrait,
    {
        W::to_bytes(self)
    }

    #[inline]
    pub fn as_i18(&self) -> NbtResult<i8> {
        match self {
            NbtValue::Byte(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(1_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i16(&self) -> NbtResult<i16> {
        match self {
            NbtValue::Short(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(2_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i32(&self) -> NbtResult<i32> {
        match self {
            NbtValue::Int(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(3_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i64(&self) -> NbtResult<i64> {
        match self {
            NbtValue::Long(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(4_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_f32(&self) -> NbtResult<f32> {
        match self {
            NbtValue::Float(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(5_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_f64(&self) -> NbtResult<f64> {
        match self {
            NbtValue::Double(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(6_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i8_array(&self) -> NbtResult<Vec<i8>> {
        match self {
            NbtValue::ByteArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(7_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i32_array(&self) -> NbtResult<Vec<i32>> {
        match self {
            NbtValue::IntArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(11_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i64_array(&self) -> NbtResult<Vec<i64>> {
        match self {
            NbtValue::LongArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(12_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_string(&self) -> NbtResult<String> {
        match self {
            NbtValue::String(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(8_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_list(&self) -> NbtResult<Vec<NbtValue>> {
        match self {
            NbtValue::List(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(9_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_compound(&self) -> NbtResult<(Option<&String>, Vec<(String, NbtValue)>)> {
        match self {
            NbtValue::Compound(name, v) => Ok((name.as_ref(), v.clone())),
            _ => Err(NbtError::IncorrectType(10_u8, self.tag())),
        }
    }

    #[inline]
    pub fn is_i8(&self) -> bool { matches!(self, NbtValue::Byte(_)) }
    #[inline]
    pub fn is_i16(&self) -> bool { matches!(self, NbtValue::Short(_)) }
    #[inline]
    pub fn is_i32(&self) -> bool { matches!(self, NbtValue::Int(_)) }
    #[inline]
    pub fn is_i64(&self) -> bool { matches!(self, NbtValue::Long(_)) }
    #[inline]
    pub fn is_f32(&self) -> bool { matches!(self, NbtValue::Float(_)) }
    #[inline]
    pub fn is_f64(&self) -> bool { matches!(self, NbtValue::Double(_)) }
    #[inline]
    pub fn is_i8_array(&self) -> bool { matches!(self, NbtValue::ByteArray(_)) }
    #[inline]
    pub fn is_i32_array(&self) -> bool { matches!(self, NbtValue::IntArray(_)) }
    #[inline]
    pub fn is_i64_array(&self) -> bool { matches!(self, NbtValue::LongArray(_)) }
    #[inline]
    pub fn is_string(&self) -> bool { matches!(self, NbtValue::String(_)) }
    #[inline]
    pub fn is_list(&self) -> bool { matches!(self, NbtValue::List(_)) }
    #[inline]
    pub fn is_compound(&self) -> bool { matches!(self, NbtValue::Compound(_, _)) }
}
