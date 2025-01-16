/// 仅借用的实现
pub mod borrow;
/// Error
pub mod error;
/// nbt! 宏
pub mod macros;
/// 感谢 @mat 允许我使用他的代码
///
/// 用于处理 mutf8 编码
pub mod mutf8;
/// 几乎就是从 v5 copy 过来的
pub mod reader;
/// 一些实现
pub mod traits;
/// 核心 value 实现
pub mod value;


#[cfg(test)]
mod tests;

/// 如果 `serde` 特性被启用，则导出 `serding` 模块
///
/// 用于序列化和反序列化 Nbt 数据
///
/// TODO!
#[cfg(feature = "serde")]
pub mod serding;

// re-exports
pub use error::NbtError;
pub use mutf8::Mutf8String;
pub use reader::NbtReader;
pub use value::NbtValue;

/// 用于存储 Nbt 类型的标识符
pub type NbtTypeId = u8;
/// Nbt 读取过程中的结果
pub type NbtResult<T> = Result<T, NbtError>;

/// 各种 NBT 版本
pub mod nbt_versions {
    /// Java 版 绝大部分的 NBT 格式
    ///
    /// 除了 1.20.2+(协议号 >= 764) 及以后 的网路传输 NBT 格式 都是这个
    ///
    /// 上面说的那玩意 请使用 `JavaNetAfter1_20_2`
    ///
    /// # 编码特点
    ///
    /// 大端
    pub struct Java;
    /// 1.20.2+(协议号 >= 764) 及以后 的网路传输 NBT 格式
    ///
    /// # 编码特点
    ///
    /// 根节点没有名称
    pub struct JavaNetAfter1_20_2;
    /// 基岩版 实际用于存储的 NBT 格式
    ///
    /// # 编码特点
    ///
    /// 小端
    pub struct BedrockDisk;
    /// 基岩版 网络 NBT 格式
    /// 最痛苦的一集
    ///
    /// # 编码特点
    ///
    /// VarInt, VarLong, ZigZagVarInt, ZigZagVarLong
    /// 全都有
    pub struct BedrockNetVarInt;
}

/// 一些 NBT 中的常量
pub mod nbt_consts {
    pub const TAG_END: u8 = 0;
    pub const TAG_BYTE: u8 = 1;
    pub const TAG_SHORT: u8 = 2;
    pub const TAG_INT: u8 = 3;
    pub const TAG_LONG: u8 = 4;
    pub const TAG_FLOAT: u8 = 5;
    pub const TAG_DOUBLE: u8 = 6;
    pub const TAG_BYTE_ARRAY: u8 = 7;
    pub const TAG_STRING: u8 = 8;
    pub const TAG_LIST: u8 = 9;
    pub const TAG_COMPOUND: u8 = 10;
    pub const TAG_INT_ARRAY: u8 = 11;
    pub const TAG_LONG_ARRAY: u8 = 12;
}
