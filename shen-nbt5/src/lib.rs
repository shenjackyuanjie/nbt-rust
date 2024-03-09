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

macro_rules! read_uncheck {
    ($name:ident, $ty:ty, $size:literal) => {
        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        ///
        #[doc = "转换大小端"]
        ///
        /// # 安全性
        /// 允许未对齐的地址
        /// 长度溢出会导致 UB
        #[inline]
        pub unsafe fn $name(&mut self) -> $ty {
            // 使用 std::ptr::read_unaligned 解决未对齐地址问题
            let value = std::ptr::read_unaligned(self.data[self.cursor..].as_ptr() as *const $ty);
            self.cursor += std::mem::size_of::<$ty>();
            value.to_be()
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
    read_uncheck!(read_i16_unchecked, i16, 2);
    read_uncheck!(read_u16_unchecked, u16, 2);
    read_uncheck!(read_i32_unchecked, i32, 4);
    read_uncheck!(read_u32_unchecked, u32, 4);
    read_uncheck!(read_i64_unchecked, i64, 8);
    read_uncheck!(read_u64_unchecked, u64, 8);
    /// 安全的读取 i16 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_be_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        value
    }
    /// 安全的读取 u16 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_u16(&mut self) -> u16 { self.read_i16() as u16 }
    /// 安全的读取 i32 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        value
    }
    /// 安全的读取 u32 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_u32(&mut self) -> u32 { self.read_i32() as u32 }
    /// 安全的读取 i64 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
            self.data[self.cursor + 4],
            self.data[self.cursor + 5],
            self.data[self.cursor + 6],
            self.data[self.cursor + 7],
        ]);
        self.cursor += 8;
        value
    }
    /// 安全的读取 u64 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_u64(&mut self) -> u64 { self.read_i64() as u64 }
    /// 读取一个 f32 类型的数据
    ///
    /// 转换大小端
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_f32(&mut self) -> f32 { f32::from_bits(self.read_u32()) }
    /// 读取一个 f64 类型的数据
    ///
    /// 转换大小端
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_f64(&mut self) -> f64 { f64::from_bits(self.read_u64()) }
    /// 读取一个 f32 类型的数据
    ///
    /// 转换大小端
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    pub unsafe fn read_f32_unchecked(&mut self) -> f32 {
        let value = std::ptr::read_unaligned(self.data[self.cursor..].as_ptr() as *const u32);
        self.cursor += 4;
        std::mem::transmute::<u32, f32>(value.to_be())
    }
    /// 读取一个 f64 类型的数据
    /// 转换大小端
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    pub unsafe fn read_f64_unchecked(&mut self) -> f64 {
        let value = std::ptr::read_unaligned(self.data[self.cursor..].as_ptr() as *const u64);
        self.cursor += 8;
        std::mem::transmute::<u64, f64>(value.to_be())
    }
    /// 读取指定长度的 u8 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_u8_array(&mut self, len: usize) -> &[u8] {
        let value = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        value
    }
    /// 读取指定长度的 i8 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    pub fn read_i8_array_unchecked(&mut self, len: usize) -> &[i8] {
        let value = unsafe {
            std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i8, len)
        };
        self.cursor += len;
        value
    }
    /// 读取指定长度的 utf-8 字符串
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_string(&mut self, len: usize) -> String {
        let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + len]);
        self.cursor += len;
        value.into_owned()
    }
    /// 读取指定长度的 i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    pub fn read_i32_array_unchecked(&mut self, len: usize) -> Vec<i32> {
        unsafe {
            let value =
                std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i32, len);
            let mut value = value.to_vec();
            for n in &mut value {
                *n = n.to_be();
            }
            self.cursor += len * 4;
            value
        }
    }
    /// 读取指定长度的 i64 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    pub fn read_i64_array_unchecked(&mut self, len: usize) -> Vec<i64> {
        unsafe {
            let value =
                std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i64, len);
            let mut value = value.to_vec();
            for n in &mut value {
                *n = n.to_be();
            }
            self.cursor += len * 8;
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
