/// Error
pub mod error;
/// 感谢 @mat 允许我使用他的代码
///
/// 用于处理 mutf8 编码
pub mod mutf8;
/// 一些实现
pub mod traits;
/// 核心 value 实现
pub mod value;
/// 仅借用的实现
pub mod borrow;
/// 几乎就是从 v5 copy 过来的
pub mod reader;

/// 如果 `serde` 特性被启用，则导出 `serding` 模块
///
/// 用于序列化和反序列化 Nbt 数据
///
/// TODO!
#[cfg(feature = "serde")]
pub mod serding;

// re-exports
pub use error::NbtError;
pub use value::NbtValue;

/// 用于存储 Nbt 类型的标识符
pub type NbtTypeId = u8;
/// Nbt 读取过程中的结果
pub type NbtResult<T> = Result<T, NbtError>;
