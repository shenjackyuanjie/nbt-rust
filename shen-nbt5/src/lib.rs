// pub enum Endian {
//     Big,
//     Little,
// }

#[cfg(test)]
mod tests;

/// 用于读取 NBT 数据
pub struct NbtReader<'data> {
    /// NBT 数据
    pub data: &'data mut [u8],
    /// 当前读取的位置
    pub cursor: usize,
    // be/le
    // pub endian: Endian,
}

macro_rules! read {
    ($name:ident, $ty:ty, $size:literal) => {
        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        ///
        #[doc = "转换大小端"]
        pub fn $name(&mut self) -> $ty {
            unsafe {
                // 使用 std::ptr::read_unaligned 解决未对齐地址问题
                let value =
                    std::ptr::read_unaligned(self.data[self.cursor..].as_ptr() as *const $ty);
                self.cursor += std::mem::size_of::<$ty>();
                value.to_be()
            }
        }
    };
    ($name:ident, $ty:ty, $size:literal, false) => {
        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        ///
        #[doc = "不转换大小端"]
        pub fn $name(&mut self) -> $ty {
            unsafe {
                // 使用 std::ptr::read_unaligned 解决未对齐地址问题
                let value =
                    std::ptr::read_unaligned(self.data[self.cursor..].as_ptr() as *const $ty);
                self.cursor += std::mem::size_of::<$ty>();
                value
            }
        }
    };
}
impl NbtReader<'_> {
    pub fn new(data: &mut [u8]) -> NbtReader {
        NbtReader {
            data,
            cursor: 0,
            // endian: Endian::Big,
        }
    }
    /// 读取一个 u8 类型的数据
    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        let value = self.data[self.cursor];
        self.cursor += 1;
        value
    }
    /// 读取一个 i8 类型的数据
    #[inline]
    pub fn read_i8(&mut self) -> i8 { self.read_u8() as i8 }
    read!(read_i16_unchecked, i16, 2);
    read!(read_u16_unchecked, u16, 2);
    read!(read_i32_unchecked, i32, 4);
    read!(read_u32_unchecked, u32, 4);
    read!(read_i64_unchecked, i64, 8);
    read!(read_u64_unchecked, u64, 8);
    read!(read_f32_unchecked, f32, 4, false);
    read!(read_f64_unchecked, f64, 8, false);

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
        unsafe {
            println!("data: {:?}", self.data);
            let value =
                std::slice::from_raw_parts_mut(self.data[self.cursor..].as_ptr() as *mut i32, len);
            for n in &mut *value {
                *n = n.to_be();
            }
            self.cursor += len * 4;
            println!("data: {:?}", self.data);
            value
        }
    }
    pub fn read_long_array(&mut self, len: usize) -> &[i64] {
        unsafe {
            println!("data: {:?}", self.data);
            let value =
                std::slice::from_raw_parts_mut(self.data[self.cursor..].as_ptr() as *mut i64, len);
            for n in &mut *value {
                *n = n.to_be();
            }
            self.cursor += len * 8;
            println!("data: {:?}", self.data);
            value
        }
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
