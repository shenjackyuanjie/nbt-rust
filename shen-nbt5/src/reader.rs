use crate::nbt_version::{BedrockDisk, BedrockNetVarInt, Java, JavaNetAfter1_20_2, NbtReadTrait};
use crate::{nbt_version, NbtError, NbtResult, NbtValue};

/// 用于读取 NBT 数据
pub struct NbtReader<'data> {
    /// NBT 数据
    pub data: &'data mut [u8],
    /// 当前读取的位置
    pub cursor: usize,
}

/// Java 版 绝大部分的 NBT 格式
///
/// 除了 1.20.2+(协议号 >= 764) 及以后 的网路传输 NBT 格式 都是这个
///
/// 上面说的那玩意 请使用 `JavaNetAfter1_20_2`
impl nbt_version::NbtReadTrait for nbt_version::Java {
    #[inline]
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String> {
        let len = reader.read_be_u16() as usize;
        reader.read_string(len)
    }
    #[inline]
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>> {
        let len = reader.read_be_i32() as usize;
        let value = reader.read_i8_array(len);
        Ok(value)
    }
    #[inline]
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>> {
        let len = reader.read_be_i32() as usize;
        let value = reader.read_i32_array(len);
        Ok(value)
    }
    #[inline]
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>> {
        let len = reader.read_be_i32() as usize;
        let value = reader.read_i64_array(len);
        Ok(value)
    }
    #[inline]
    fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>> {
        let mut compound = Vec::with_capacity(10);
        loop {
            let tag_id = reader.read_u8();
            if tag_id == 0 {
                break;
            }
            let name = Java::read_nbt_string(reader)?;
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_be_i16()),
                3 => NbtValue::Int(reader.read_be_i32()),
                4 => NbtValue::Long(reader.read_be_i64()),
                5 => NbtValue::Float(reader.read_be_f32()),
                6 => NbtValue::Double(reader.read_be_f64()),
                7 => NbtValue::ByteArray(Java::read_i8_array(reader)?),
                8 => NbtValue::String(Java::read_nbt_string(reader)?),
                9 => NbtValue::List(Java::read_list(reader)?),
                10 => NbtValue::Compound(None, nbt_version::Java::read_compound(reader)?),
                11 => NbtValue::IntArray(Java::read_i32_array(reader)?),
                12 => NbtValue::LongArray(Java::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(tag_id)),
            };
            compound.push((name, value));
        }
        Ok(compound)
    }
    #[inline]
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>> {
        let type_id = reader.read_u8();
        let len = reader.read_be_i32() as usize;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            let value = match type_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_be_i16()),
                3 => NbtValue::Int(reader.read_be_i32()),
                4 => NbtValue::Long(reader.read_be_i64()),
                5 => NbtValue::Float(reader.read_be_f32()),
                6 => NbtValue::Double(reader.read_be_f64()),
                7 => NbtValue::ByteArray(Java::read_i8_array(reader)?),
                8 => NbtValue::String(Java::read_nbt_string(reader)?),
                9 => NbtValue::List(Java::read_list(reader)?),
                10 => NbtValue::Compound(None, nbt_version::Java::read_compound(reader)?),
                11 => NbtValue::IntArray(Java::read_i32_array(reader)?),
                12 => NbtValue::LongArray(Java::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(type_id)),
            };
            list.push(value);
        }
        Ok(list)
    }

    fn from_reader(mut reader: NbtReader) -> NbtResult<NbtValue> {
        // 第一个 tag, 不可能是 0
        match reader.read_u8() {
            10 => {
                let name = Java::read_nbt_string(&mut reader)?;
                Ok(NbtValue::Compound(Some(name), nbt_version::Java::read_compound(&mut reader)?))
            }
            x => Err(NbtError::WrongRootType(x)),
        }
    }
}

/// 两个最好实现的就在这里了
///
/// 网络 NBT: 1.20.2+ 的网络 NBT 根节点没有名字
impl NbtReadTrait for JavaNetAfter1_20_2 {
    #[inline]
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String> {
        Java::read_nbt_string(reader)
    }
    #[inline]
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>> { Java::read_i8_array(reader) }
    #[inline]
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>> {
        Java::read_i32_array(reader)
    }
    #[inline]
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>> {
        Java::read_i64_array(reader)
    }
    #[inline]
    fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>> {
        Java::read_compound(reader)
    }
    #[inline]
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>> { Java::read_list(reader) }

    fn from_reader(mut reader: NbtReader) -> NbtResult<NbtValue> {
        // 第一个 tag, 不可能是 0
        match reader.read_u8() {
            10 => {
                // Java 1.20.2+ 的网络 NBT 没有名字
                Ok(NbtValue::Compound(None, nbt_version::Java::read_compound(&mut reader)?))
            }
            x => Err(NbtError::WrongRootType(x)),
        }
    }
}

/// 基岩版的其实也还行, 就是有点麻烦
///
/// 所有都是小端
impl NbtReadTrait for BedrockDisk {
    #[inline]
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String> {
        let len = reader.read_le_u16() as usize;
        reader.read_string(len)
    }
    #[inline]
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>> {
        let len = reader.read_le_i32() as usize;
        let value = reader.read_i8_array(len);
        Ok(value)
    }
    #[inline]
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>> {
        let len = reader.read_le_i32() as usize;
        let value = reader.read_i32_array(len);
        Ok(value)
    }
    #[inline]
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>> {
        let len = reader.read_le_i32() as usize;
        let value = reader.read_i64_array(len);
        Ok(value)
    }
    #[inline]
    fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>> {
        let mut compound = Vec::with_capacity(10);
        loop {
            let tag_id = reader.read_u8();
            if tag_id == 0 {
                break;
            }
            let name = BedrockDisk::read_nbt_string(reader)?;
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_le_i16()),
                3 => NbtValue::Int(reader.read_le_i32()),
                4 => NbtValue::Long(reader.read_le_i64()),
                5 => NbtValue::Float(reader.read_le_f32()),
                6 => NbtValue::Double(reader.read_le_f64()),
                7 => NbtValue::ByteArray(BedrockDisk::read_i8_array(reader)?),
                8 => NbtValue::String(BedrockDisk::read_nbt_string(reader)?),
                9 => NbtValue::List(BedrockDisk::read_list(reader)?),
                10 => NbtValue::Compound(None, nbt_version::BedrockDisk::read_compound(reader)?),
                11 => NbtValue::IntArray(BedrockDisk::read_i32_array(reader)?),
                12 => NbtValue::LongArray(BedrockDisk::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(tag_id)),
            };
            compound.push((name, value));
        }
        Ok(compound)
    }
    #[inline]
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>> {
        let type_id = reader.read_u8();
        let len = reader.read_le_i32() as usize;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            let value = match type_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_le_i16()),
                3 => NbtValue::Int(reader.read_le_i32()),
                4 => NbtValue::Long(reader.read_le_i64()),
                5 => NbtValue::Float(reader.read_le_f32()),
                6 => NbtValue::Double(reader.read_le_f64()),
                7 => NbtValue::ByteArray(BedrockDisk::read_i8_array(reader)?),
                8 => NbtValue::String(BedrockDisk::read_nbt_string(reader)?),
                9 => NbtValue::List(BedrockDisk::read_list(reader)?),
                10 => NbtValue::Compound(None, nbt_version::BedrockDisk::read_compound(reader)?),
                11 => NbtValue::IntArray(BedrockDisk::read_i32_array(reader)?),
                12 => NbtValue::LongArray(BedrockDisk::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(type_id)),
            };
            list.push(value);
        }
        Ok(list)
    }

    fn from_reader(mut reader: NbtReader) -> NbtResult<NbtValue> {
        // 第一个 tag, 不可能是 0
        match reader.read_u8() {
            9 => {
                // 基岩版的 NBT 根节点可以是一个 List
                Ok(NbtValue::List(nbt_version::BedrockDisk::read_list(&mut reader)?))
            }
            10 => {
                // 或者一个有名字的 Compound
                let name = BedrockDisk::read_nbt_string(&mut reader)?;
                Ok(NbtValue::Compound(
                    Some(name),
                    nbt_version::BedrockDisk::read_compound(&mut reader)?,
                ))
            }
            // 别的不行
            x => Err(NbtError::WrongRootType(x)),
        }
    }
}

/// 最痛苦的来了
impl NbtReadTrait for BedrockNetVarInt {
    fn read_nbt_string(reader: &mut NbtReader) -> NbtResult<String> {
        let len = reader.read_var_i32()? as usize;
        reader.read_string(len)
    }
    fn read_i8_array(reader: &mut NbtReader) -> NbtResult<Vec<i8>> {
        let len = reader.read_zigzag_var_i32()? as usize;
        let value = reader.read_i8_array(len);
        Ok(value)
    }
    fn read_i32_array(reader: &mut NbtReader) -> NbtResult<Vec<i32>> {
        let len = reader.read_zigzag_var_i32()? as usize;
        let value = reader.read_i32_array(len);
        Ok(value)
    }
    fn read_i64_array(reader: &mut NbtReader) -> NbtResult<Vec<i64>> {
        let len = reader.read_zigzag_var_i32()? as usize;
        let value = reader.read_i64_array(len);
        Ok(value)
    }
    fn read_compound(reader: &mut NbtReader) -> NbtResult<Vec<(String, NbtValue)>> {
        let mut compound = Vec::with_capacity(10);
        loop {
            let tag_id = reader.read_u8();
            if tag_id == 0 {
                break;
            }
            let name = BedrockNetVarInt::read_nbt_string(reader)?;
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_le_i16()),
                3 => NbtValue::Int(reader.read_zigzag_var_i32()?),
                4 => NbtValue::Long(reader.read_zigzag_var_i64()?),
                5 => NbtValue::Float(reader.read_le_f32()),
                6 => NbtValue::Double(reader.read_le_f64()),
                7 => NbtValue::ByteArray(BedrockNetVarInt::read_i8_array(reader)?),
                8 => NbtValue::String(BedrockNetVarInt::read_nbt_string(reader)?),
                9 => NbtValue::List(BedrockNetVarInt::read_list(reader)?),
                10 => NbtValue::Compound(None, BedrockNetVarInt::read_compound(reader)?),
                11 => NbtValue::IntArray(BedrockNetVarInt::read_i32_array(reader)?),
                12 => NbtValue::LongArray(BedrockNetVarInt::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(tag_id)),
            };
            compound.push((name, value));
        }
        Ok(compound)
    }
    fn read_list(reader: &mut NbtReader) -> NbtResult<Vec<NbtValue>> {
        let type_id = reader.read_u8();
        let len = reader.read_zigzag_var_i32()? as usize;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            let value = match type_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_le_i16()),
                3 => NbtValue::Int(reader.read_zigzag_var_i32()?),
                4 => NbtValue::Long(reader.read_zigzag_var_i64()?),
                5 => NbtValue::Float(reader.read_le_f32()),
                6 => NbtValue::Double(reader.read_le_f64()),
                7 => NbtValue::ByteArray(BedrockNetVarInt::read_i8_array(reader)?),
                8 => NbtValue::String(BedrockNetVarInt::read_nbt_string(reader)?),
                9 => NbtValue::List(BedrockNetVarInt::read_list(reader)?),
                10 => NbtValue::Compound(None, BedrockNetVarInt::read_compound(reader)?),
                11 => NbtValue::IntArray(BedrockNetVarInt::read_i32_array(reader)?),
                12 => NbtValue::LongArray(BedrockNetVarInt::read_i64_array(reader)?),
                _ => return Err(NbtError::UnknownType(type_id)),
            };
            list.push(value);
        }
        Ok(list)
    }
    fn from_reader(mut reader: NbtReader) -> NbtResult<NbtValue> {
        match reader.read_u8() {
            9 => {
                // 基岩版的 NBT 根节点可以是一个 List
                Ok(NbtValue::List(BedrockNetVarInt::read_list(&mut reader)?))
            }
            10 => {
                // 或者一个有名字的 Compound
                let name = BedrockNetVarInt::read_nbt_string(&mut reader)?;
                Ok(NbtValue::Compound(Some(name), BedrockNetVarInt::read_compound(&mut reader)?))
            }
            // 别的不行
            x => Err(NbtError::WrongRootType(x)),
        }
    }
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
    pub fn new(data: &mut [u8]) -> NbtReader { NbtReader { data, cursor: 0 } }
    /// 向后滚动
    #[inline]
    pub fn roll_back(&mut self, len: usize) { self.cursor -= len; }
    /// 向前滚动
    #[inline]
    pub fn roll_down(&mut self, len: usize) { self.cursor += len; }
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
    read_uncheck!(read_be_i16_unsafe, read_le_i16_unsafe, i16, 2);
    read_uncheck!(read_be_u16_unsafe, read_le_u16_unsafe, u16, 2);
    read_uncheck!(read_be_i32_unsafe, read_le_i32_unsafe, i32, 4);
    read_uncheck!(read_be_u32_unsafe, read_le_u32_unsafe, u32, 4);
    read_uncheck!(read_be_i64_unsafe, read_le_i64_unsafe, i64, 8);
    read_uncheck!(read_be_u64_unsafe, read_le_u64_unsafe, u64, 8);
    /// 安全的读取 i16 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_i16(&mut self) -> i16 {
        let value = i16::from_be_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        value
    }
    /// 安全的读取小端 i16 数据
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_i16(&mut self) -> i16 {
        let value = i16::from_le_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;
        value
    }
    /// 安全的读取 u16 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_u16(&mut self) -> u16 { self.read_be_i16() as u16 }
    /// 安全的读取 u16 类型的数据
    ///
    /// 转换大小端(小端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_u16(&mut self) -> u16 { self.read_le_i16() as u16 }
    /// 安全的读取 i32 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_i32(&mut self) -> i32 {
        let value = i32::from_be_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        value
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
            let byte = self.read_u8();
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
            let byte = self.read_u8();
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
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes([
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
    pub fn read_be_u32(&mut self) -> u32 {
        let value = u32::from_be_bytes([
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
    /// 转换大小端(小端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += 4;
        value
    }
    /// 安全的读取 i64 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_i64(&mut self) -> i64 {
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
    /// 安全的读取 i64 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_i64(&mut self) -> i64 {
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
        value
    }
    /// 安全的读取 u64 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_u64(&mut self) -> u64 {
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
        value
    }
    /// 安全的读取 u64 类型的数据
    ///
    /// 转换大小端(大端)
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_u64(&mut self) -> u64 {
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
        value
    }
    /// 读取一个大端 f32 数据
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_f32(&mut self) -> f32 { f32::from_bits(self.read_be_u32()) }
    /// 读取一个小端 f32 数据
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_f32(&mut self) -> f32 { f32::from_bits(self.read_le_u32()) }
    /// 读取一个大端 f64 数据
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_be_f64(&mut self) -> f64 { f64::from_bits(self.read_be_u64()) }
    /// 读取一个小端 f64 数据
    ///
    /// 会在超出长度时 panic
    #[inline]
    pub fn read_le_f64(&mut self) -> f64 { f64::from_bits(self.read_le_u64()) }
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
    /// 读取指定长度的 i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_i32_array_unsafe(&mut self, len: usize) -> Vec<i32> {
        let value =
            std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i32, len);
        let mut value = value.to_vec();
        for n in &mut value {
            *n = n.to_be();
        }
        self.cursor += len * 4;
        value
    }
    /// 读取指定长度的 i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_i32_array(&mut self, len: usize) -> Vec<i32> {
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
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_i64_array_unsafe(&mut self, len: usize) -> Vec<i64> {
        let value =
            std::slice::from_raw_parts(self.data[self.cursor..].as_ptr() as *const i64, len);
        let mut value = value.to_vec();
        for n in &mut value {
            *n = n.to_be();
        }
        self.cursor += len * 8;
        value
    }
    /// 读取指定长度的 i64 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_i64_array(&mut self, len: usize) -> Vec<i64> {
        let value = self.data[self.cursor..self.cursor + len * 8]
            .chunks_exact(8)
            .map(|n| i64::from_be_bytes(n[0..8].try_into().unwrap()))
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
