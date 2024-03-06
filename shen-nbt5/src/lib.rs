// pub enum Endian {
//     Big,
//     Little,
// }

/// 用于读取 NBT 数据
pub struct NbtReader<'data> {
    /// NBT 数据
    pub data: &'data [u8],
    /// 当前读取的位置
    pub cursor: usize,
    // be/le
    // pub endian: Endian,
}

macro_rules! read {
    ($name:ident, $ty:ty, $size:literal) => {
        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        pub fn $name(&mut self) -> $ty {
            unsafe {
                let value = *(self.data[self.cursor..].as_ptr() as *const $ty);
                self.cursor += std::mem::size_of::<$ty>();
                value.to_be()
            }
        }
    };
    ($name:ident, $ty:ty, $size:literal, false) => {
        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        pub fn $name(&mut self) -> $ty {
            unsafe {
                let value = *(self.data[self.cursor..].as_ptr() as *const $ty);
                self.cursor += std::mem::size_of::<$ty>();
                value
            }
        }
    };
}
impl NbtReader<'_> {
    pub fn new(data: &[u8]) -> NbtReader {
        NbtReader {
            data,
            cursor: 0,
            // endian: Endian::Big,
        }
    }
    pub fn read_i8(&mut self) -> i8 {
        let value = self.data[self.cursor] as i8;
        self.cursor += 1;
        value
    }
    pub fn read_u8(&mut self) -> u8 {
        let value = self.data[self.cursor];
        self.cursor += 1;
        value
    }
    read!(read_i16, i16, 2);
    read!(read_u16, u16, 2);
    read!(read_i32, i32, 4);
    read!(read_u32, u32, 4);
    read!(read_i64, i64, 8);
    read!(read_u64, u64, 8);
    read!(read_f32, f32, 4, false);
    read!(read_f64, f64, 8, false);

    pub fn read_u8_array(&mut self, len: usize) -> &[u8] {
        let value = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        value
    }
    pub fn read_i8_array(&mut self, len: usize) -> &[i8] {
        let value = unsafe {
            std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i8, len)
        };
        self.cursor += len;
        value
    }
    pub fn read_string(&mut self, len: usize) -> String {
        let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + len]);
        self.cursor += len;
        value.into_owned()
    }
    pub fn read_int_array(&mut self, len: usize) -> &[i32] {
        let datas = self.read_u8_array(len * 4);
        let value = unsafe {
            std::slice::from_raw_parts(datas.as_ptr() as *const i32, len)
        };
    }
    pub fn read_long_array(&mut self, len: usize) -> &[i64] {
        let datas = self.read_u8_array(len * 8);
        let value = unsafe {
            std::slice::from_raw_parts(datas.as_ptr() as *const i64, len)
        };
    }

}

#[derive(Debug, Clone)]
pub enum NbtValue {
    // end: 0
    /// 1: Byte
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
    /// 7
    ByteArray(Vec<i8>),
    /// 8
    /// 或者叫 u8 array
    String(String),
    /// 9
    List(Vec<NbtValue>),
    /// 10
    Compound(Vec<(String, NbtValue)>),
    /// 11
    IntArray(Vec<i32>),
    /// 12
    LongArray(Vec<i64>),
}


