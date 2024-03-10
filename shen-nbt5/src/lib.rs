pub mod reader;

use reader::NbtReader;

#[cfg(test)]
mod tests;

/// 后面也许会实现的
///
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
// pub enum NbtVersion {
//     Java,
//     JavaNetAfter1_20_2,
//     BedrockDisk,
//     BedrockNetVarInt,
// }
pub mod nbt_version {
    use super::{NbtReader, NbtResult, NbtValue};

    pub trait NbtReadTrait {
        fn from_reader(reader: NbtReader) -> NbtResult<NbtValue>;
        fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>>;
        fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>>;
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
    pub enum JavaNetAfter1_20_2 {}
    pub enum BedrockDisk {}
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
    WrongRootType(u8),
    /// 根节点无名称
    RootWithoutName,
    /// 未知类型
    UnknownType(u8),
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
}

pub type NbtResult<T> = Result<T, NbtError>;

impl std::fmt::Display for NbtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtError::UnknownErr(s) => write!(f, "未知错误: {}", s),
            NbtError::WrongRootType(n) => {
                match n {
                    9 => {
                        write!(f, "根节点为 NbtList(9) 类型, 是否应该使用 BedrockDisk/BedrockNetVarInt 解析?")
                    }
                    _ => {
                        write!(f, "根节点类型错误: {}, 应为 NbtCompound/NbtList(bedrock only)", n)
                    }
                }
            }
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    pub fn from_binary<T>(data: &mut [u8]) -> NbtResult<NbtValue> 
    where T: nbt_version::NbtReadTrait {
        let reader = NbtReader::new(data);
        T::from_reader(reader)
    }

    // fn read_nbt_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>> {
    //     let mut compound = Vec::with_capacity(10);
    //     loop {
    //         let tag_id = reader.read_u8();
    //         if tag_id == 0 {
    //             break;
    //         }
    //         let name = reader.read_nbt_string()?;
    //         let value = match tag_id {
    //             1 => NbtValue::Byte(reader.read_i8()),
    //             2 => NbtValue::Short(reader.read_be_i16()),
    //             3 => NbtValue::Int(reader.read_be_i32()),
    //             4 => NbtValue::Long(reader.read_be_i64()),
    //             5 => NbtValue::Float(reader.read_be_f32()),
    //             6 => NbtValue::Double(reader.read_be_f64()),
    //             7 => NbtValue::ByteArray(reader.read_nbt_i8_array()),
    //             8 => NbtValue::String(reader.read_nbt_string()?),
    //             9 => NbtValue::List(NbtValue::read_nbt_list(reader)?),
    //             10 => NbtValue::Compound(None, NbtValue::read_nbt_compound(reader)?),
    //             11 => NbtValue::IntArray(reader.read_nbt_i32_array()),
    //             12 => NbtValue::LongArray(reader.read_nbt_i64_array()),
    //             _ => unimplemented!(),
    //         };
    //         compound.push((name, value));
    //     }
    //     Ok(compound)
    // }

    // fn read_nbt_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>> {
    //     let type_id = reader.read_u8();
    //     let len = reader.read_be_i32() as usize;
    //     let mut list = Vec::with_capacity(len);
    //     for _ in 0..len {
    //         let value = match type_id {
    //             1 => NbtValue::Byte(reader.read_i8()),
    //             2 => NbtValue::Short(reader.read_be_i16()),
    //             3 => NbtValue::Int(reader.read_be_i32()),
    //             4 => NbtValue::Long(reader.read_be_i64()),
    //             5 => NbtValue::Float(reader.read_be_f32()),
    //             6 => NbtValue::Double(reader.read_be_f64()),
    //             7 => NbtValue::ByteArray(reader.read_nbt_i8_array()),
    //             8 => NbtValue::String(reader.read_nbt_string()?),
    //             9 => NbtValue::List(NbtValue::read_nbt_list(reader)?),
    //             10 => NbtValue::Compound(None, NbtValue::read_nbt_compound(reader)?),
    //             11 => NbtValue::IntArray(reader.read_nbt_i32_array()),
    //             12 => NbtValue::LongArray(reader.read_nbt_i64_array()),
    //             _ => unimplemented!(),
    //         };
    //         list.push(value);
    //     }
    //     Ok(list)
    // }

    // pub fn from_reader(mut reader: NbtReader) -> NbtValue {
    //     // 第一个 tag, 不可能是 0
    //     match reader.read_u8() {
    //         9 => NbtValue::List(NbtValue::read_nbt_list(&mut reader).unwrap()),
    //         10 => {
    //             let name = reader.read_nbt_string().unwrap();
    //             NbtValue::Compound(Some(name), NbtValue::read_nbt_compound(&mut reader).unwrap())
    //         }
    //         x => {
    //             panic!("根节点类型错误 {}", x.as_nbt_type_name());
    //         }
    //     }
    // }
}
