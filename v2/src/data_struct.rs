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
    // 还有一个 End: 0
    /// 1
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
    /// 8
    String(&'value str),
    /// 7
    ByteArray(RawData<'value>),
    /// 11
    IntArray(RawData<'value>),
    /// 12
    LongArray(RawData<'value>),
    /// 9
    List(Vec<ListContent<'value>>),
    /// 10
    Compound(HashMap<&'value str, Value<'value>>),
}

#[allow(unused)]
pub enum ListContent<'value> {
    ByteList(Vec<i8>),
    ShortList(Vec<i16>),
    IntList(Vec<i32>),
    LongList(Vec<i64>),
    FloatList(Vec<f32>),
    DoubleList(Vec<f64>),
    StringList(Vec<&'value str>),
    ByteArrayList(Vec<RawData<'value>>),
    IntArrayList(Vec<RawData<'value>>),
    LongArrayList(Vec<RawData<'value>>),
    CompoundList(Vec<HashMap<String, Value<'value>>>),
    ListList(Vec<ListContent<'value>>),
}

#[allow(unused)]
impl<'value> Value<'value> {
    #[inline(always)]
    pub fn read_byte(data: &mut [u8]) -> Self {
        let (value, data) = data.split_at(1);
        Self::Byte(value[0] as i8)
    }
    #[inline(always)]
    pub fn read_short(data: &mut [u8]) -> Self {
        let (value, data) = data.split_at(2);
        #[cfg(target_endian = "little")]
        return Self::Short(i16::from_be_bytes([value[0], value[1]]));
        #[cfg(target_endian = "big")]
        return Self::Short(i16::from_le_bytes([value[0], value[1]]));
    }
    #[inline(always)]
    pub fn read_int(data: &mut [u8]) -> Self {
        let (value, data) = data.split_at(4);
        #[cfg(target_endian = "little")]
        return Self::Int(i32::from_be_bytes([value[0], value[1], value[2], value[3]]));
        #[cfg(target_endian = "big")]
        return Self::Int(i32::from_le_bytes([value[0], value[1], value[2], value[3]]));
    }
    #[inline(always)]
    pub fn read_long(data: &mut [u8]) -> Self {
        let (value, data) = data.split_at(8);
        #[cfg(target_endian = "little")]
        return Self::Long(i64::from_be_bytes([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
        ]));
        #[cfg(target_endian = "big")]
        return Self::Long(i64::from_le_bytes([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
        ]));
    }
    #[inline(always)]
    pub fn read_float(data: &mut [u8]) -> Self {
        let (value, data) = data.split_at(4);
        #[cfg(target_endian = "little")]
        return Self::Float(f32::from_be_bytes([value[0], value[1], value[2], value[3]]));
        #[cfg(target_endian = "big")]
        return Self::Float(f32::from_le_bytes([value[0], value[1], value[2], value[3]]));
    }
    #[inline(always)]
    pub fn read_double(data: &mut [u8]) -> Self {
        let (value, data) = data.split_at(8);
        #[cfg(target_endian = "little")]
        return Self::Double(f64::from_be_bytes([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
        ]));
        #[cfg(target_endian = "big")]
        return Self::Double(f64::from_le_bytes([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
        ]));
    }
    #[inline(always)]
    pub fn read_string(data: &'value mut [u8]) -> Self {
        let (length, data) = data.split_at(2);
        let length = u16::from_le_bytes([length[0], length[1]]);
        let (value, data) = data.split_at(length as usize);
        Self::String(std::str::from_utf8(value).unwrap())
    }
    pub fn read_list(data: &mut [u8]) -> Self {
        // 内容类型
        let (type_id, data) = data.split_at(1);
        // 内容长度
        let (length, data) = data.split_at(4);
        let length = i32::from_le_bytes([length[0], length[1], length[2], length[3]]);
        todo!()
    }
}
