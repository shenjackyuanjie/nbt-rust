use std::{borrow::Cow, collections::HashMap, str::Bytes};

/// NBT 里除了字符串的长度量都是 i32
#[allow(unused)]
pub type NbtLength = i32;

/// NBT 里的字符串独树一帜的把自己的长度用一个u32表示
/// 不如说为啥别的不用 u32 呢
#[allow(unused)]
pub type StringLength = u16;

pub struct NbtData {
    pub head: usize,
    pub data: Vec<u8>,
}

#[allow(unused)]
impl NbtData {
    pub fn new(data: Vec<u8>) -> Self {
        Self { head: 0, data }
    }
    pub fn get_mut(&mut self) -> &mut [u8] {
        let (_, data) = self.data.split_at_mut(self.head);
        data
    }
    pub fn push_head(&mut self, length: usize) -> usize {
        self.head += length;
        self.head
    }
    pub fn read_byte(&mut self) -> i8 {
        let value = self.data[self.head] as i8;
        self.head += 1;
        value
    }
    pub fn read_short(&mut self) -> i16 {
        let value = i16::from_le_bytes([self.data[self.head], self.data[self.head + 1]]);
        self.head += 2;
        value
    }
    pub fn read_int(&mut self) -> i32 {
        let value = i32::from_le_bytes([
            self.data[self.head],
            self.data[self.head + 1],
            self.data[self.head + 2],
            self.data[self.head + 3],
        ]);
        self.head += 4;
        value
    }
    pub fn read_long(&mut self) -> i64 {
        let value = i64::from_le_bytes([
            self.data[self.head],
            self.data[self.head + 1],
            self.data[self.head + 2],
            self.data[self.head + 3],
            self.data[self.head + 4],
            self.data[self.head + 5],
            self.data[self.head + 6],
            self.data[self.head + 7],
        ]);
        self.head += 8;
        value
    }
    pub fn read_float(&mut self) -> f32 {
        let value = f32::from_le_bytes([
            self.data[self.head],
            self.data[self.head + 1],
            self.data[self.head + 2],
            self.data[self.head + 3],
        ]);
        self.head += 4;
        value
    }
    pub fn read_double(&mut self) -> f64 {
        let value = f64::from_le_bytes([
            self.data[self.head],
            self.data[self.head + 1],
            self.data[self.head + 2],
            self.data[self.head + 3],
            self.data[self.head + 4],
            self.data[self.head + 5],
            self.data[self.head + 6],
            self.data[self.head + 7],
        ]);
        self.head += 8;
        value
    }
    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let value = self.data[self.head..self.head + length].to_vec();
        self.head += length;
        value
    }
}

#[allow(unused)]
pub struct RawData {
    pub raw_data: Vec<u8>,
    pub length: usize,
}

#[allow(unused)]
impl RawData {
    pub fn new(raw_data: Vec<u8>, length: usize) -> Self {
        Self { raw_data, length }
    }
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
    String(Cow<'value, str>),
    /// 7
    ByteArray(RawData),
    /// 11
    IntArray(RawData),
    /// 12
    LongArray(RawData),
    /// 9
    List(ListContent<'value>),
    /// 10
    Compound(HashMap<Cow<'value, str>, Value<'value>>),
}

#[allow(unused)]
#[allow(unused)]
pub enum ListContent<'value> {
    ByteList(Vec<i8>),
    ShortList(Vec<i16>),
    IntList(Vec<i32>),
    LongList(Vec<i64>),
    FloatList(Vec<f32>),
    DoubleList(Vec<f64>),
    StringList(Vec<String>),
    ByteArrayList(Vec<RawData>),
    IntArrayList(Vec<RawData>),
    LongArrayList(Vec<RawData>),
    CompoundList(Vec<HashMap<String, Value<'value>>>),
    ListList(Vec<ListContent<'value>>),
}

// #[allow(unused)]
impl<'value> Value<'value> {
    #[inline(always)]
    pub fn read_byte(data: &mut NbtData) -> Self {
        Self::Byte(data.read_byte())
    }
    #[inline(always)]
    pub fn read_short(data: &mut NbtData) -> Self {
        Self::Short(data.read_short())
    }
    #[inline(always)]
    pub fn read_int(data: &mut NbtData) -> Self {
        Self::Int(data.read_int())
    }
    #[inline(always)]
    pub fn read_long(data: &mut NbtData) -> Self {
        Self::Long(data.read_long())
    }
    #[inline(always)]
    pub fn read_float(data: &mut NbtData) -> Self {
        Self::Float(data.read_float())
    }
    #[inline(always)]
    pub fn read_double(data: &mut NbtData) -> Self {
        Self::Double(data.read_double())
    }
    #[inline(always)]
    pub fn read_string(data: &mut NbtData) -> Self {
        let length = data.read_short();
        let value = data.read_bytes(length as usize);
        Self::String(std::str::from_utf8(value.as_slice()).unwrap().to_owned().into())
    }
    pub fn read_list(data: &mut NbtData) -> Self {
        // 内容类型
        let type_id = data.read_byte();
        // 内容长度
        let length = data.read_int();
        match type_id {
            0 => panic!("WTF, type_id = 0"),
            1 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(data.read_byte());
                }
                Self::List(ListContent::ByteList(list))
            }
            2 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(data.read_short());
                }
                Self::List(ListContent::ShortList(list))
            }
            3 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(data.read_int());
                }
                Self::List(ListContent::IntList(list))
            }
            4 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(data.read_long());
                }
                Self::List(ListContent::LongList(list))
            }
            5 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(data.read_float());
                }
                Self::List(ListContent::FloatList(list))
            }
            6 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(data.read_double());
                }
                Self::List(ListContent::DoubleList(list))
            }
            7 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let length = data.read_int();
                    let value = data.read_bytes(length as usize);
                    list.push(RawData {
                        raw_data: value,
                        length: length as usize,
                    });
                }
                Self::List(ListContent::ByteArrayList(list))
            }
            8 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let length = data.read_int();
                    let value = std::str::from_utf8(data.read_bytes(length as usize).as_slice())
                        .unwrap()
                        .to_owned();
                    list.push(value);
                }
                Self::List(ListContent::StringList(list))
            }
            9 => {
                // 好好好, list 嵌套 list 是吧
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    // let inner_list = Self::read_list(data);
                }
                Self::List(ListContent::ListList(list))
            }
            _ => panic!("WTF, type_id = {}", type_id),
        }
    }
}
