pub mod error;
pub mod traits;
pub mod mutf8;

#[cfg(feature = "serde")]
pub mod serding;

pub type NbtTypeId = u8;
