#![feature(core_intrinsics)]

use std::borrow::Cow;
#[cfg(feature = "internal_opt")]
use std::intrinsics::unlikely;

pub struct NbtData {
    pub head: usize,
    pub data: Vec<u8>,
}

impl NbtData {
    pub fn new(data: Vec<u8>) -> Self { Self { head: 0, data } }
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
        let value = i16::from_be_bytes([self.data[self.head], self.data[self.head + 1]]);
        self.head += 2;
        value
    }
    pub fn read_int(&mut self) -> i32 {
        let value = i32::from_be_bytes([
            self.data[self.head],
            self.data[self.head + 1],
            self.data[self.head + 2],
            self.data[self.head + 3],
        ]);
        self.head += 4;
        value
    }
    pub fn read_long(&mut self) -> i64 {
        let value = i64::from_be_bytes([
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
        let value = f32::from_be_bytes([
            self.data[self.head],
            self.data[self.head + 1],
            self.data[self.head + 2],
            self.data[self.head + 3],
        ]);
        self.head += 4;
        value
    }
    pub fn read_double(&mut self) -> f64 {
        let value = f64::from_be_bytes([
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

pub mod raw_reading {
    #[cfg(feature = "internal_opt")]
    use std::intrinsics::unlikely;

    /// 多少有点脱裤子放屁
    #[inline(always)]
    pub fn slice_as_byte_array(slice: &[u8]) -> Vec<i8> {
        slice.to_vec().into_iter().map(|x| x as i8).collect()
    }
    /// unsafe 从这里开始
    #[inline(always)]
    pub fn slice_as_short_array(slice: &[u8]) -> Option<Vec<i16>> {
        #[cfg(feature = "internal_opt")]
        let length = if unlikely(slice.len() % 2 != 0) {
            return None;
        } else {
            (slice.len() / 2) as usize
        };
        #[cfg(not(feature = "internal_opt"))]
        let length = if slice.len() % 2 != 0 {
            return None;
        } else {
            (slice.len() / 2) as usize
        };
        Some(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *mut i16, length).to_vec() })
    }
    /// 开始 unsafe 了
    /// unsafe rust, 小子!
    #[inline(always)]
    pub fn slice_as_int_array(slice: &[u8]) -> Option<Vec<i32>> {
        #[cfg(feature = "internal_opt")]
        let length = if unlikely(slice.len() % 4 != 0) {
            return None;
        } else {
            (slice.len() / 4) as usize
        };
        #[cfg(not(feature = "internal_opt"))]
        let length = if slice.len() % 4 != 0 {
            return None;
        } else {
            (slice.len() / 4) as usize
        };
        Some(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *mut i32, length).to_vec() })
    }
    /// 这边也是 unsafe 捏
    #[inline(always)]
    pub fn slice_as_long_array(slice: &[u8]) -> Option<Vec<i64>> {
        let length = if slice.len() % 8 != 0 {
            return None;
        } else {
            (slice.len() / 8) as usize
        };
        Some(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *mut i64, length).to_vec() })
    }
    /// 这边也是 unsafe 捏
    #[inline(always)]
    pub fn slice_as_float_array(slice: &[u8]) -> Option<Vec<f32>> {
        let length = if slice.len() % 4 != 0 {
            return None;
        } else {
            (slice.len() / 4) as usize
        };
        Some(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *mut f32, length).to_vec() })
    }
    /// 这边也是 unsafe 捏
    #[inline(always)]
    pub fn slice_as_double_array(slice: &[u8]) -> Option<Vec<f64>> {
        let length = if slice.len() % 8 != 0 {
            return None;
        } else {
            (slice.len() / 8) as usize
        };
        Some(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *mut f64, length).to_vec() })
    }
}

#[derive(Debug)]
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
    ByteArray(Vec<i8>),
    /// 11
    IntArray(Vec<i32>),
    /// 12
    LongArray(Vec<i64>),
    /// 9
    List(ListContent<'value>),
    /// 10
    Compound(Vec<(String, Value<'value>)>),
}

#[derive(Debug)]
pub enum ListContent<'value> {
    ByteList(Vec<i8>),
    ShortList(Vec<i16>),
    IntList(Vec<i32>),
    LongList(Vec<i64>),
    FloatList(Vec<f32>),
    DoubleList(Vec<f64>),
    StringList(Vec<String>),
    ByteArrayList(Vec<Vec<i8>>),
    IntArrayList(Vec<Vec<i32>>),
    LongArrayList(Vec<Vec<i64>>),
    CompoundList(Vec<Vec<(String, Value<'value>)>>),
    ListList(Vec<ListContent<'value>>),
}

impl<'value> Value<'value> {
    #[inline(always)]
    pub fn read_byte(data: &mut NbtData) -> Self { Self::Byte(data.read_byte()) }
    #[inline(always)]
    pub fn read_short(data: &mut NbtData) -> Self { Self::Short(data.read_short()) }
    #[inline(always)]
    pub fn read_int(data: &mut NbtData) -> Self { Self::Int(data.read_int()) }
    #[inline(always)]
    pub fn read_long(data: &mut NbtData) -> Self { Self::Long(data.read_long()) }
    #[inline(always)]
    pub fn read_float(data: &mut NbtData) -> Self { Self::Float(data.read_float()) }
    #[inline(always)]
    pub fn read_double(data: &mut NbtData) -> Self { Self::Double(data.read_double()) }
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
                let raw_data = data.read_bytes(length as usize);
                let list = raw_reading::slice_as_byte_array(raw_data.as_slice());
                Self::List(ListContent::ByteList(list))
            }
            2 => {
                let raw_data = data.read_bytes(length as usize * 2);
                let list = raw_reading::slice_as_short_array(raw_data.as_slice()).unwrap();
                Self::List(ListContent::ShortList(list))
            }
            3 => {
                let raw_data = data.read_bytes(length as usize * 4);
                let list = raw_reading::slice_as_int_array(raw_data.as_slice()).unwrap();
                Self::List(ListContent::IntList(list))
            }
            4 => {
                let raw_data = data.read_bytes(length as usize * 8);
                let list = raw_reading::slice_as_long_array(raw_data.as_slice()).unwrap();
                Self::List(ListContent::LongList(list))
            }
            5 => {
                let raw_data = data.read_bytes(length as usize * 4);
                let list = raw_reading::slice_as_float_array(raw_data.as_slice()).unwrap();
                Self::List(ListContent::FloatList(list))
            }
            6 => {
                let raw_data = data.read_bytes(length as usize * 8);
                let list = raw_reading::slice_as_double_array(raw_data.as_slice()).unwrap();
                Self::List(ListContent::DoubleList(list))
            }
            7 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let length = data.read_int();
                    let raw_data = data.read_bytes(length as usize);
                    let value = raw_reading::slice_as_byte_array(raw_data.as_slice());
                    list.push(value);
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
                    let inner_list = Self::read_list(data);
                    let value = inner_list.into_list().unwrap();
                    list.push(value);
                }
                Self::List(ListContent::ListList(list))
            }
            10 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let inner_compound = Self::read_compound(data);
                    let value = inner_compound.into_compound().unwrap();
                    list.push(value);
                }
                Self::List(ListContent::CompoundList(list))
            }
            11 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let length = data.read_int();
                    let raw_data = data.read_bytes(length as usize * 4);
                    let value = raw_reading::slice_as_int_array(raw_data.as_slice()).unwrap();
                    list.push(value);
                }
                Self::List(ListContent::IntArrayList(list))
            }
            12 => {
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let length = data.read_int();
                    let raw_data = data.read_bytes(length as usize * 8);
                    let value = raw_reading::slice_as_long_array(raw_data.as_slice()).unwrap();
                    list.push(value);
                }
                Self::List(ListContent::LongArrayList(list))
            }
            _ => panic!("WTF, type_id = {}", type_id),
        }
    }
    pub fn read_compound(data: &mut NbtData) -> Self {
        let mut type_id = 1;
        let mut list = Vec::with_capacity(10);
        while type_id != 0 {
            type_id = data.read_byte();
            #[cfg(feature = "internal_opt")]
            if unlikely(type_id == 0) {
                break;
            }
            #[cfg(not(feature = "internal_opt"))]
            if type_id == 0 {
                break;
            }
            let name_len = data.read_short();
            let name = if name_len != 0 {
                let name = String::from_utf8(data.read_bytes(name_len as usize)).unwrap();
                name
            } else {
                String::new()
            };
            let value = match type_id {
                0 => break,
                1 => Self::read_byte(data),
                2 => Self::read_short(data),
                3 => Self::read_int(data),
                4 => Self::read_long(data),
                5 => Self::read_float(data),
                6 => Self::read_double(data),
                7 => Self::read_byte_array(data),
                8 => Self::read_string(data),
                9 => Self::read_list(data),
                10 => Self::read_compound(data),
                11 => Self::read_int_array(data),
                12 => Self::read_long_array(data),
                _ => panic!("WTF, type_id = {}", type_id),
            };
            list.push((name, value));
        }
        Self::Compound(list)
    }
    #[inline(always)]
    pub fn read_byte_array(data: &mut NbtData) -> Self {
        let length = data.read_int();
        let raw_data = data.read_bytes(length as usize);
        let value = raw_reading::slice_as_byte_array(raw_data.as_slice());
        Self::ByteArray(value)
    }
    #[inline(always)]
    pub fn read_int_array(data: &mut NbtData) -> Self {
        let length = data.read_int();
        let raw_data = data.read_bytes(length as usize * 4);
        let value = raw_reading::slice_as_int_array(raw_data.as_slice()).unwrap();
        Self::IntArray(value)
    }
    #[inline(always)]
    pub fn read_long_array(data: &mut NbtData) -> Self {
        let length = data.read_int();
        let raw_data = data.read_bytes(length as usize * 8);
        let value = raw_reading::slice_as_long_array(raw_data.as_slice()).unwrap();
        Self::LongArray(value)
    }
    pub fn as_byte(&self) -> Option<i8> {
        match self {
            Self::Byte(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_short(&self) -> Option<i16> {
        match self {
            Self::Short(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_long(&self) -> Option<i64> {
        match self {
            Self::Long(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f32> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_double(&self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_ref()),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&ListContent> {
        match self {
            Self::List(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_compound(&self) -> Option<&Vec<(String, Value<'value>)>> {
        match self {
            Self::Compound(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_byte_array(&self) -> Option<&Vec<i8>> {
        match self {
            Self::ByteArray(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_int_array(&self) -> Option<&Vec<i32>> {
        match self {
            Self::IntArray(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_long_array(&self) -> Option<&Vec<i64>> {
        match self {
            Self::LongArray(value) => Some(value),
            _ => None,
        }
    }
    #[inline(always)]
    pub fn into_list(self) -> Option<ListContent<'value>> {
        match self {
            Self::List(value) => Some(value),
            _ => None,
        }
    }
    #[inline(always)]
    pub fn into_compound(self) -> Option<Vec<(String, Value<'value>)>> {
        match self {
            Self::Compound(value) => Some(value),
            _ => None,
        }
    }
    pub fn from_vec(data: Vec<u8>) -> Self {
        let mut nbt_data = NbtData::new(data);
        let _type_id = nbt_data.read_byte();
        let _name_len = nbt_data.read_short();
        let _name = String::from_utf8(nbt_data.read_bytes(_name_len as usize)).unwrap();
        Value::read_compound(&mut nbt_data)
    }
}
