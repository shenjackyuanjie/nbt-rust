use crate::borrow::BorrowNbtValue;
use crate::traits::NbtTypeConversion;
use crate::{nbt_consts, Mutf8String, NbtError, NbtReader, NbtResult, NbtValue};

/// 把一个 borrow value 转换成 owned value
pub fn own_value(value: &BorrowNbtValue, data: &mut NbtReader) -> NbtValue {
    let name_value = value.as_compound_idx().unwrap();
    let root_name = name_value
        .1
        .map(|name_len| Mutf8String::from_reader(data, name_value.0, name_len).unwrap());
    let mut root_element = NbtValue::Compound(root_name, vec![]);

    todo!()
}
