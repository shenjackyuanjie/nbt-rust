use std::collections::HashMap;

/// NBT 里除了字符串的长度量都是 i32
#[allow(unused)]
pub type NbtLength = i32;

/// NBT 里的字符串独树一帜的把自己的长度用一个u32表示
/// 不如说为啥别的不用 u32 呢
#[allow(unused)]
pub type StringLength = u16;

#[allow(unused)]
pub struct RawData<'data> {
    pub raw_data: &'data [u8],
    pub length: usize,
}

#[allow(unused)]
impl<'data> RawData<'data> {
    pub fn new(raw_data: &'data [u8], length: usize) -> Self { Self { raw_data, length } }
}

#[allow(unused)]
pub enum Value<'value> {
    Byte(RawData<'value>),
    Short(RawData<'value>),
    Int(RawData<'value>),
    Long(RawData<'value>),
    Float(RawData<'value>),
    Double(RawData<'value>),
    String(RawData<'value>),
    ByteArray(RawData<'value>),
    IntArray(RawData<'value>),
    LongArray(RawData<'value>),
    List(Vec<Value<'value>>),
    Compound(HashMap<&'value str, Value<'value>>),
}
