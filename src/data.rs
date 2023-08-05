use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::rc::Rc;
use std::sync::Arc;

/// NBT 里除了字符串的长度量都是 i32
pub type NbtLength = i32;

/// NBT 里的字符串独树一帜的把自己的长度用一个u32表示
/// 不如说为啥别的不用 u32 呢
pub type StringLength = u32;

/// Reader
pub type Reader<'a> = Cursor<&'a [u8]>;

#[derive(Debug, Clone)]
pub enum NbtItem {
    Value(NbtValue),
    Array(NbtList),
}

#[derive(Debug, Clone)]
pub enum NbtList {
    BoolArray(Rc<RefCell<Vec<bool>>>),
    IntArray(Rc<RefCell<Vec<i32>>>),
    LongArray(Rc<RefCell<Vec<i64>>>),
    List(Rc<RefCell<Vec<NbtItem>>>),
    Compound(Rc<RefCell<HashMap<Arc<str>, NbtItem>>>),
}

/// 基本 NBT 数据类型
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum NbtValue {
    /// 0x00
    /// 标志着一个 NBT Compound/List 的结束
    NbtEnd,
    /// 0x01
    NbtByte(bool),
    /// 0x02
    NbtShort(i16),
    /// 0x03
    NbtInt(i32),
    /// 0x04
    NbtLong(i64),
    /// 0x05
    NbtFloat(f32),
    /// 0x06
    NbtDouble(f64),
    /// 0x08
    /// 一个 UTF-8 编码的定长字符串
    NbtString(Arc<str>),
}

impl NbtItem {
    #[inline]
    pub fn as_value(&self) -> Option<&NbtValue> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<&NbtList> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }
}

impl From<NbtValue> for NbtItem {
    #[inline]
    fn from(value: NbtValue) -> Self { Self::Value(value) }
}

impl From<NbtList> for NbtItem {
    #[inline]
    fn from(value: NbtList) -> Self { Self::Array(value) }
}

impl From<Vec<NbtItem>> for NbtItem {
    #[inline]
    fn from(value: Vec<NbtItem>) -> Self { Self::Array(NbtList::from(Rc::new(RefCell::new(value)))) }
}

impl From<HashMap<Arc<str>, NbtItem>> for NbtItem {
    #[inline]
    fn from(value: HashMap<Arc<str>, NbtItem>) -> Self { Self::Array(NbtList::from(Rc::new(RefCell::new(value)))) }
}

impl From<Vec<bool>> for NbtItem {
    #[inline]
    fn from(value: Vec<bool>) -> Self { Self::Array(NbtList::from(Rc::new(RefCell::new(value)))) }
}

impl From<Vec<i32>> for NbtItem {
    #[inline]
    fn from(value: Vec<i32>) -> Self { Self::Array(NbtList::from(Rc::new(RefCell::new(value)))) }
}

impl From<Vec<i64>> for NbtItem {
    #[inline]
    fn from(value: Vec<i64>) -> Self { Self::Array(NbtList::from(Rc::new(RefCell::new(value)))) }
}

impl From<Vec<NbtItem>> for NbtList {
    #[inline]
    fn from(value: Vec<NbtItem>) -> Self { Self::List(Rc::new(RefCell::new(value))) }
}

impl From<HashMap<Arc<str>, NbtItem>> for NbtList {
    #[inline]
    fn from(value: HashMap<Arc<str>, NbtItem>) -> Self { Self::Compound(Rc::new(RefCell::new(value))) }
}

impl From<Vec<bool>> for NbtList {
    #[inline]
    fn from(value: Vec<bool>) -> Self { Self::BoolArray(Rc::new(RefCell::new(value))) }
}

impl From<Vec<i32>> for NbtList {
    #[inline]
    fn from(value: Vec<i32>) -> Self { Self::IntArray(Rc::new(RefCell::new(value))) }
}

impl From<Vec<i64>> for NbtList {
    #[inline]
    fn from(value: Vec<i64>) -> Self { Self::LongArray(Rc::new(RefCell::new(value))) }
}

macro_rules! export_data {
    ($name:ident, $nbt_name:ident, $type:ty) => {
        #[inline]
        pub fn $name(&self) -> Option<$type> {
            match self {
                Self::$nbt_name(value) => Some(value.to_owned()),
                _ => None,
            }
        }
    };
}

macro_rules! read_data {
    ($name:ident, $nbt_name:ident, bool, 1) => {
        /// 直接读取值 不带类型数据和名称
        #[inline]
        pub fn $name(value: &mut Reader) -> Self {
            let mut buff = [0_u8];
            _ = value.read(&mut buff).unwrap();
            Self::$nbt_name(buff[0] != 0)
        }
    };
    ($name:ident, $nbt_name:ident, $type:ty, $len:expr) => {
        /// 直接读取值 不带类型数据和名称
        #[inline]
        pub fn $name(value: &mut Reader) -> Self {
            let mut buff = [0_u8; $len];
            _ = value.read(&mut buff).unwrap();
            Self::$nbt_name(<$type>::from_be_bytes(buff))
        }
    };
}

#[allow(unused)]
impl NbtValue {
    pub fn as_end(&self) -> Option<()> {
        match self {
            Self::NbtEnd => Some(()),
            _ => None,
        }
    }

    export_data!(as_bool, NbtByte, bool);
    export_data!(as_i16, NbtShort, i16);
    export_data!(as_i32, NbtInt, i32);
    export_data!(as_i64, NbtLong, i64);
    export_data!(as_f32, NbtFloat, f32);
    export_data!(as_f64, NbtDouble, f64);
    export_data!(as_string, NbtString, Arc<str>);

    pub fn from_end(value: &mut Reader) -> Self {
        let mut buff = [0_u8];
        _ = value.read(&mut buff).unwrap();
        Self::NbtEnd
    }

    read_data!(from_bool, NbtByte, bool, 1);
    read_data!(from_i16, NbtShort, i16, 2);
    read_data!(from_i32, NbtInt, i32, 4);
    read_data!(from_i64, NbtLong, i64, 8);
    read_data!(from_f32, NbtFloat, f32, 4);
    read_data!(from_f64, NbtDouble, f64, 8);

    /// 直接读取
    pub fn from_string(value: &mut Reader) -> Self {
        let len: StringLength = Self::from_i32(value).as_i32().unwrap() as StringLength;
        if len == 0 {
            return Self::NbtString(Arc::from(""));
        }
        let mut buff = vec![0_u8; len as usize];
        _ = value.read(&mut buff).unwrap();
        Self::NbtString(Arc::from(String::from_utf8(buff).unwrap()))
    }

    /// 读取一个有类型无名称的值 (List)
    pub fn try_read_value(value: &mut Reader) -> Option<Self> {
        let mut value_type = [0_u8];
        _ = value.read(&mut value_type).unwrap();
        match value_type {
            [0x00] => Some(Self::NbtEnd),
            [0x01] => Some(Self::from_bool(value)),
            [0x02] => Some(Self::from_i16(value)),
            [0x03] => Some(Self::from_i32(value)),
            [0x04] => Some(Self::from_i64(value)),
            [0x05] => Some(Self::from_f32(value)),
            [0x06] => Some(Self::from_f64(value)),
            [0x08] => Some(Self::from_string(value)),
            _ => {
                value.seek(SeekFrom::Current(-1)).unwrap();
                // 退回一个字节
                None
            }
        }
    }
    /// 读取一个有类型有名称的值 (Other)
    pub fn try_read_value_with_name(value: &mut Reader) -> Option<(Self, Arc<str>)> {
        let mut value_type = [0_u8];
        _ = value.read(&mut value_type).unwrap();
        match value_type {
            [0x00] => Some((Self::NbtEnd, Arc::from(""))),
            [0x01] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_bool(value), name))
            }
            [0x02] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_i16(value), name))
            }
            [0x03] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_i32(value), name))
            }
            [0x04] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_i64(value), name))
            }
            [0x05] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_f32(value), name))
            }
            [0x06] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_f64(value), name))
            }
            [0x08] => {
                let name = Self::from_string(value).as_string().unwrap();
                Some((Self::from_string(value), name))
            }
            _ => {
                value.seek(SeekFrom::Current(-1)).unwrap();
                // 退回一个字节
                None
            }
        }
    }
}
