use crate::{NbtError, NbtResult};

/// 用于读取 NBT 数据
pub struct NbtReader<'data> {
    /// NBT 数据
    pub data: &'data [u8],
    /// 当前读取的位置
    pub cursor: usize,
}

macro_rules! read_uncheck {
    ($be_name:ident, $le_name:ident, $ty:ty, $size:literal) => {
        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        ///
        /// 转换大小端(大端)
        ///
        /// # 安全性
        /// 允许未对齐的地址
        /// 长度溢出会导致 UB
        #[inline]
        pub unsafe fn $be_name(&mut self) -> $ty {
            // 使用 std::ptr::read_unaligned 解决未对齐地址问题
            let ptr = self.data.as_ptr().add(self.cursor) as *const $ty;
            let value = std::ptr::read_unaligned(ptr);
            self.cursor += std::mem::size_of::<$ty>();
            value.to_be()
        }

        #[doc = concat!("读取 ", stringify!($ty), " 类型 ", $size, " 长度的数据")]
        ///
        /// 转换大小端(小端)
        ///
        /// # 安全性
        /// 允许未对齐的地址
        /// 长度溢出会导致 UB
        #[inline]
        pub unsafe fn $le_name(&mut self) -> $ty {
            // 使用 std::ptr::read_unaligned 解决未对齐地址问题
            let ptr = self.data.as_ptr().add(self.cursor) as *const $ty;
            let value = std::ptr::read_unaligned(ptr);
            self.cursor += std::mem::size_of::<$ty>();
            value.to_le()
        }
    };
}

impl NbtReader<'_> {
    pub fn new(data: &[u8]) -> NbtReader { NbtReader { data, cursor: 0 } }
    /// 向后滚动
    #[inline]
    pub fn roll_back(&mut self, len: usize) -> NbtResult<()> {
        if len > self.cursor {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        self.cursor -= len;
        Ok(())
    }
    /// 向前滚动
    /// 会在超出长度返回错误
    #[inline]
    #[must_use]
    pub fn roll_down(&mut self, len: usize) -> NbtResult<()> {
        if self.cursor + len > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        self.cursor += len;
        Ok(())
    }
    #[inline]
    pub fn roll_to(&mut self, pos: usize) -> NbtResult<()> {
        if pos > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, pos, self.data.len()));
        }
        self.cursor = pos;
        Ok(())
    }
    /// 滚动到顶部
    pub fn roll_top(&mut self) { self.cursor = 0 }
    /// 滚动到底部
    pub fn roll_bottom(&mut self) { self.cursor = self.data.len() }

    /// 比较华丽的展示当前指针位置
    ///
    /// 会把当前指针位置标记出来
    ///
    /// 尽量把当前指针位置放在中间
    ///
    /// 默认展示 10 个字节
    ///
    /// ```text
    /// [...., 0x01, 0x02, 0x03, 0x04, 0x05, ....]
    ///                    ^^^^ pos: 3
    /// ```
    pub fn show_cursor_fancy(&self, display_len: Option<usize>) -> String {
        let show_len = display_len.unwrap_or(10).min(self.data.len());
        // 中间位置
        let middle = show_len / 2;
        let start = if self.cursor > middle {
            self.cursor - middle
        } else {
            0
        };
        let end = (start + show_len).min(self.data.len());
        let display_data = self.data[start..end]
            .iter()
            .map(|byte| format!("0x{:02X}", byte))
            .collect::<Vec<String>>();

        let cursor_pointer = {
            let pos = self.cursor - start;
            if start != 0 {
                // 前面有个 ....
                format!("{}^^^^ pos: {}", " ".repeat(4 * (pos + 1) + 1), self.cursor)
            } else {
                // 没有 ....
                format!("{}^^^^ pos: {}", " ".repeat(4 * pos + 1), self.cursor)
            }
        };

        let values = {
            format!(
                "[{}{}{}]",
                if start != 0 { "...., " } else { "" },
                display_data.join(", "),
                if end != self.data.len() { ", ...." } else { "" }
            )
        };
        format!("{}\n{}", values, cursor_pointer)
    }

    /// 检查 cursor 是否超出范围
    /// 如果超出范围, 则返回 false
    #[inline]
    pub fn check_cursor(&self) -> bool { self.cursor < self.data.len() }
    /// 读取一个 u8 类型的数据
    #[inline]
    pub fn read_u8(&mut self) -> NbtResult<u8> {
        if self.cursor >= self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 1, self.data.len()));
        }
        let value = self.data[self.cursor];
        self.cursor += 1;
        Ok(value)
    }
    /// 读取一个 i8 类型的数据
    #[inline]
    pub fn read_i8(&mut self) -> NbtResult<i8> { Ok(self.read_u8()? as i8) }
    read_uncheck!(read_be_i16_unsafe, read_le_i16_unsafe, i16, 2);
    read_uncheck!(read_be_u16_unsafe, read_le_u16_unsafe, u16, 2);
    read_uncheck!(read_be_i32_unsafe, read_le_i32_unsafe, i32, 4);
    read_uncheck!(read_be_u32_unsafe, read_le_u32_unsafe, u32, 4);
    read_uncheck!(read_be_i64_unsafe, read_le_i64_unsafe, i64, 8);
    read_uncheck!(read_be_u64_unsafe, read_le_u64_unsafe, u64, 8);
    /// 安全的读取 i16 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_be_i16(&mut self) -> NbtResult<i16> {
        if self.cursor + 2 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 2, self.data.len()));
        }
        let value = i16::from_be_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        Ok(value)
    }
    /// 安全的读取小端 i16 数据
    #[inline]
    pub fn read_le_i16(&mut self) -> NbtResult<i16> {
        if self.cursor + 2 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 2, self.data.len()));
        }
        let value = i16::from_le_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        Ok(value)
    }
    /// 安全的读取 u16 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_be_u16(&mut self) -> NbtResult<u16> {
        if self.cursor + 2 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 2, self.data.len()));
        }
        let value = u16::from_be_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        Ok(value)
    }
    /// 安全的读取 u16 类型的数据
    ///
    /// 转换大小端(小端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_u16(&mut self) -> NbtResult<u16> {
        if self.cursor + 2 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 2, self.data.len()));
        }
        let value = u16::from_le_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        Ok(value)
    }
    /// 安全的读取 i32 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_be_i32(&mut self) -> NbtResult<i32> {
        if self.cursor + 4 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 4, self.data.len()));
        }
        let value = i32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(value)
    }
    /// 安全的读取一个 Varint 数据
    ///
    /// 他有大小端区别吗? (其实是小端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_var_i32(&mut self) -> NbtResult<i32> {
        let mut value = 0;
        let mut size = 0;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0b0111_1111) as i32) << (size * 7);
            size += 1;
            if size > 5 {
                return Err(NbtError::VarIntTooBig(value as usize));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(value)
    }
    /// 安全的读取一个 Varlong
    ///
    /// 他有大小端区别吗? (其实是小端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_var_i64(&mut self) -> NbtResult<i64> {
        let mut value = 0;
        let mut size = 0;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0b0111_1111) as i64) << (size * 7);
            size += 1;
            if size > 10 {
                return Err(NbtError::VarlongTooBig(value as usize));
            }
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(value)
    }
    /// 安全的读取一个 zigzag 编码的 varint
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_zigzag_var_i32(&mut self) -> NbtResult<i32> {
        let value = self.read_var_i32()?;
        Ok((value >> 1) ^ (-(value & 1)))
    }
    /// 安全的读取一个 zigzag 编码的 varlong
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_zigzag_var_i64(&mut self) -> NbtResult<i64> {
        let value = self.read_var_i64()?;
        Ok((value >> 1) ^ (-(value & 1)))
    }
    /// 安全的读取一个小端 i32 数据
    #[inline]
    pub fn read_le_i32(&mut self) -> NbtResult<i32> {
        if self.cursor + 4 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 4, self.data.len()));
        }
        let value = i32::from_le_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(value)
    }
    /// 安全的读取 u32 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_be_u32(&mut self) -> NbtResult<u32> {
        if self.cursor + 4 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 4, self.data.len()));
        }
        let value = u32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(value)
    }
    /// 安全的读取 u32 类型的数据
    ///
    /// 转换大小端(小端)
    #[inline]
    pub fn read_le_u32(&mut self) -> NbtResult<u32> {
        if self.cursor + 4 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 4, self.data.len()));
        }
        let value = u32::from_le_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(value)
    }
    /// 安全的读取 i64 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_be_i64(&mut self) -> NbtResult<i64> {
        if self.cursor + 8 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 8, self.data.len()));
        }
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
        Ok(value)
    }
    /// 安全的读取 i64 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_le_i64(&mut self) -> NbtResult<i64> {
        if self.cursor + 8 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 8, self.data.len()));
        }
        let value = i64::from_le_bytes([
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
        Ok(value)
    }
    /// 安全的读取 u64 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_be_u64(&mut self) -> NbtResult<u64> {
        if self.cursor + 8 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 8, self.data.len()));
        }
        let value = u64::from_be_bytes([
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
        Ok(value)
    }
    /// 安全的读取 u64 类型的数据
    ///
    /// 转换大小端(大端)
    #[inline]
    pub fn read_le_u64(&mut self) -> NbtResult<u64> {
        if self.cursor + 8 > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, 8, self.data.len()));
        }
        let value = u64::from_le_bytes([
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
        Ok(value)
    }
    /// 读取一个大端 f32 数据
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_f32(&mut self) -> NbtResult<f32> { Ok(f32::from_bits(self.read_be_u32()?)) }
    /// 读取一个小端 f32 数据
    #[inline]
    pub fn read_le_f32(&mut self) -> NbtResult<f32> { Ok(f32::from_bits(self.read_le_u32()?)) }
    /// 读取一个大端 f64 数据
    #[inline]
    pub fn read_be_f64(&mut self) -> NbtResult<f64> { Ok(f64::from_bits(self.read_be_u64()?)) }
    /// 读取一个小端 f64 数据
    #[inline]
    pub fn read_le_f64(&mut self) -> NbtResult<f64> { Ok(f64::from_bits(self.read_le_u64()?)) }
    /// 读取一个大端 f32 数据
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_be_f32_unsafe(&mut self) -> f32 {
        let value = self.read_be_u32_unsafe();
        std::mem::transmute::<u32, f32>(value)
    }
    /// 读取一个小端 f32 数据
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_le_f32_unsafe(&mut self) -> f32 {
        let value = self.read_le_u32_unsafe();
        std::mem::transmute::<u32, f32>(value)
    }
    /// 读取一个大端 f64 数据
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_be_f64_unsafe(&mut self) -> f64 {
        let value = self.read_be_u64_unsafe();
        std::mem::transmute::<u64, f64>(value)
    }
    /// 读取一个小端 f64 数据
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_le_f64_unsafe(&mut self) -> f64 {
        let value = self.read_le_u64_unsafe();
        std::mem::transmute::<u64, f64>(value)
    }
    /// 读取指定长度的 u8 数组
    #[inline]
    pub fn read_u8_array(&mut self, len: usize) -> NbtResult<&[u8]> {
        if len + self.cursor > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        let value = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(value)
    }
    /// 读取指定长度的 i8 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_i8_array_unsafe(&mut self, len: usize) -> Vec<i8> {
        let value = std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i8, len);
        self.cursor += len;
        value.to_vec()
    }
    /// 读取指定长度的 i8 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_i8_array(&mut self, len: usize) -> Vec<i8> {
        let value = self.data[self.cursor..self.cursor + len].iter().map(|&n| n as i8).collect();
        self.cursor += len;
        value
    }
    /// 读取指定长度的 i16 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_be_i16_array_unsafe(&mut self, len: usize) -> Vec<i16> {
        let mut value: Vec<i16> = Vec::with_capacity(len);
        std::ptr::copy_nonoverlapping(
            self.data[self.cursor..].as_ptr() as *const u8,
            value.as_ptr() as *mut u8,
            len * 2,
        );
        value.set_len(len);
        for n in &mut value {
            *n = n.to_be();
        }
        self.cursor += len * 2;
        value
    }
    /// 读取指定长度的 i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_be_i32_array_unsafe(&mut self, len: usize) -> Vec<i32> {
        let mut value: Vec<i32> = Vec::with_capacity(len);
        std::ptr::copy_nonoverlapping(
            self.data[self.cursor..].as_ptr() as *const u8,
            value.as_ptr() as *mut u8,
            len * 4,
        );
        value.set_len(len);
        for n in &mut value {
            *n = n.to_be();
        }
        self.cursor += len * 4;
        value
    }
    /// 读取指定长度的 i64 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_be_i64_array_unsafe(&mut self, len: usize) -> Vec<i64> {
        let mut value: Vec<i64> = Vec::with_capacity(len);
        std::ptr::copy_nonoverlapping(
            self.data[self.cursor..].as_ptr() as *const u8,
            value.as_ptr() as *mut u8,
            len * 8,
        );
        value.set_len(len);
        for n in &mut value {
            *n = n.to_be();
        }
        self.cursor += len * 8;
        value
    }
    /// 读取指定长度的 i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_be_i32_array(&mut self, len: usize) -> Vec<i32> {
        let value = self.data[self.cursor..self.cursor + len * 4]
            .chunks_exact(4)
            .map(|n| i32::from_be_bytes(n[0..4].try_into().unwrap()))
            .collect();
        self.cursor += len * 4;
        value
    }
    /// 读取指定长度的 i64 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_be_i64_array(&mut self, len: usize) -> Vec<i64> {
        let value = self.data[self.cursor..self.cursor + len * 8]
            .chunks_exact(8)
            .map(|n| i64::from_be_bytes(n[0..8].try_into().unwrap()))
            .collect();
        self.cursor += len * 8;
        value
    }
    /// 读取指定长度的 le i16 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_le_i16_array(&mut self, len: usize) -> Vec<i16> {
        let value = self.data[self.cursor..self.cursor + len * 2]
            .chunks_exact(2)
            .map(|n| i16::from_le_bytes(n[0..2].try_into().unwrap()))
            .collect();
        self.cursor += len * 2;
        value
    }
    /// 读取指定长度的 le i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_le_i32_array(&mut self, len: usize) -> Vec<i32> {
        let value = self.data[self.cursor..self.cursor + len * 4]
            .chunks_exact(4)
            .map(|n| i32::from_le_bytes(n[0..4].try_into().unwrap()))
            .collect();
        self.cursor += len * 4;
        value
    }
    /// 读取指定长度的 le i64 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_le_i64_array(&mut self, len: usize) -> Vec<i64> {
        let value = self.data[self.cursor..self.cursor + len * 8]
            .chunks_exact(8)
            .map(|n| i64::from_le_bytes(n[0..8].try_into().unwrap()))
            .collect();
        self.cursor += len * 8;
        value
    }
    /// 读取指定长度的 utf-8 字符串
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_string(&mut self, len: usize) -> Result<String, NbtError> {
        if len + self.cursor > self.data.len() {
            return Err(NbtError::CursorOverflow(self.cursor, len, self.data.len()));
        }
        let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + len]);
        self.cursor += len;
        Ok(value.into_owned())
    }
}
