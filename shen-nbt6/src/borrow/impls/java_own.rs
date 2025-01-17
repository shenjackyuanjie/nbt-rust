use crate::borrow::BorrowNbtValue;
use crate::traits::NbtTypeConversion;
use crate::{nbt_consts, NbtError, NbtReader, NbtResult, NbtValue};

/// 把一个 borrow value 转换成 owned value
pub fn own_value(value: &BorrowNbtValue, data: &mut NbtReader) -> NbtValue {
    // let mut root_element = NbtValue::Compound(, ())

    todo!()
}
