use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::Arc;

/// NBT 里除了字符串的长度量都是 i32
pub type NbtLength = i32;

/// NBT 里的字符串独树一帜的把自己的长度用一个u32表示
/// 不如说为啥别的不用 u32 呢
pub type StringLength = u32;

/// Reader
pub type Reader<'a> = Cursor<&'a [u8]>;

/// 一个 NBT list 的基本素养
pub trait NbtListTrait {
    /// 内容类型
    type ValueType;
    /// 输出类型标识符
    /// 类型标识符
    /// (0x07) Vec<bool>
    /// (0x09) Vec<NbtType>
    /// (0x0A) Compound <xxxx>
    /// (0x0B) Vec<i32>
    /// (0x0C) Vec<i64>
    fn type_tag() -> u8;
    /// 输出自身长度
    fn len(&self) -> usize;
    /// 通过索引获取内容
    fn get_index(&self, index: usize) -> Option<Self::ValueType>;
    /// 通过名称获取内容
    fn get_name(&self, name: &str) -> Option<Self::ValueType>;
}

/// 通过范型实现的 NBT List (其实包括了 NbtCompound)
#[derive(Debug, Clone)]
pub struct NbtList<T: NbtListTrait> {
    /// 内容
    pub value: T,
}

#[allow(unused)]
impl<T> NbtList<T>
where
    T: NbtListTrait,
{
    pub fn type_tag() -> u8 { T::type_tag() }

    pub fn len(&self) -> usize { self.value.len() }

    pub fn get_index(&self, index: usize) -> Option<T::ValueType> { self.value.get_index(index) }

    pub fn get_name(&self, name: &str) -> Option<T::ValueType> { self.value.get_name(name) }
}

macro_rules! add_impl {
    ($type:ty, $value:ty, $tag:expr, false) => {
        impl NbtListTrait for $type {
            type ValueType = $value;
            #[inline]
            fn type_tag() -> u8 { $tag }
            #[inline]
            fn len(&self) -> usize { self.len() }
            #[inline]
            fn get_index(&self, _: usize) -> Option<Self::ValueType> { None }
            #[inline]
            fn get_name(&self, name: &str) -> Option<Self::ValueType> { self.get(name).copied() }
        }
    };
    ($type:ty, $value:ty ,$tag:expr, true) => {
        impl NbtListTrait for $type {
            type ValueType = $value;
            #[inline]
            fn type_tag() -> u8 { $tag }
            #[inline]
            fn len(&self) -> usize { self.len() }
            #[inline]
            fn get_index(&self, index: usize) -> Option<Self::ValueType> { self.get(index).copied() }
            #[inline]
            fn get_name(&self, _: &str) -> Option<Self::ValueType> { None }
        }
    };
}

add_impl!(Vec<bool>, bool, 0x07, true);
add_impl!(HashMap<Arc<str>, i32>, i32, 0x0A, false);
add_impl!(Vec<i32>, i32, 0x0B, true);
add_impl!(Vec<i64>, i64, 0x0C, true);

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
        #[inline]
        pub fn $name(value: &mut Reader) -> Self {
            let mut buff = [0_u8];
            _ = value.read(&mut buff).unwrap();
            Self::$nbt_name(buff[0] != 0)
        }
    };
    ($name:ident, $nbt_name:ident, $type:ty, $len:expr) => {
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

    pub fn from_string(value: &mut Reader) -> Self {
        let len: StringLength = Self::from_i32(value).as_i32().unwrap() as u32;
        let mut buff = vec![0_u8; len as usize];
        _ = value.read(&mut buff).unwrap();
        Self::NbtString(Arc::from(String::from_utf8(buff).unwrap()))
    }

    /// 读取一个有类型无名称的值 (List)
    pub fn try_read_value(value: &mut Reader) -> Option<Self> {
        let mut value_type = [0_u8];
        _ = value.read(&mut value_type).unwrap();
        match value_type {
            _ => {
                value.seek(SeekFrom::Current(-1)).unwrap();
                None
            }
        }
    }
    /// 读取一个有类型有名称的值 (Other)
    pub fn try_read_value_with_name(value: &mut Reader) -> Option<(Self, Arc<str>)> {
        let mut value_type = [0_u8];
        _ = value.read(&mut value_type).unwrap();
        match value_type {
            _ => {
                value.seek(SeekFrom::Current(-1)).unwrap();
                None
            }
        }
    }
}
