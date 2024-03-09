pub enum Endian {
    Big,
    Little,
}

/// 后面也许会实现的
///
/// 不同版本的 Nbt 数据细节不同
/// 老要命了
///
/// - `Java`
///   Java 版除了 1.20.2+(协议号) 及以后的网络 NBT 格式
/// - `JavaNetAfter1_20_2`
///   1.20.2+(协议号 >= 764) 及以后的网络 NBT 格式
/// - `BedrockDisk`
///   基岩版 实际用于存储的 NBT 格式
/// - `BedrockNetVarInt`
///   基岩版 网络 NBT 格式
pub enum NbtVersion {
    Java,
    JavaNetAfter1_20_2,
    BedrockDisk,
    BedrockNetVarInt,
}

/// Error
#[derive(Debug, Clone, PartialEq)]
pub enum NbtError {
    /// 未知错误
    UnknownErr(String),
    /// 根节点类型错误
    WrongRootType(u8),
    /// 根节点无名称
    RootWithoutName,
    /// 未知类型
    UnknownTypeErr(u8),
    /// 名称读取错误
    NameReadErr(String),
    /// 指针超出范围
    ///
    /// cursor, len, data.len()
    /// 三个参数分别表示
    /// - 当前指针
    /// - 数据长度
    /// - 数据总长度
    OutOfRangeErr(usize, usize, usize),
}

impl std::fmt::Display for NbtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtError::UnknownErr(s) => write!(f, "未知错误: {}", s),
            NbtError::WrongRootType(n) => {
                match n {
                    9 => {
                        write!(f, "根节点为 NbtList(9) 类型, 是否应该使用 BedrockDisk/BedrockNetVarInt 解析?")
                    }
                    _ => {
                        write!(f, "根节点类型错误: {}, 应为 NbtCompound/NbtList(bedrock only)", n)
                    }
                }
            }
            NbtError::RootWithoutName => {
                write!(f, "根节点无名称, 是否应该使用 JavaNetAfter1_20_2 解析?")
            }
            NbtError::UnknownTypeErr(n) => {
                if *n == 0 {
                    write!(f, "未知类型: NBTEnd(0), 请检查数据是否正确")
                } else {
                    write!(f, "未知类型: {}", n)
                }
            }
            NbtError::NameReadErr(s) => write!(f, "名称读取错误: {}", s),
            NbtError::OutOfRangeErr(cursor, len, data_len) => write!(
                f,
                "指针超出范围: cursor: {}, len: {}, cursor+len: {}, data.len(): {}",
                cursor,
                len,
                cursor + len,
                data_len
            ),
        }
    }
}

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
            let ptr = self.data.as_ptr().add(self.cursor) as *const $ty;
            let value = std::ptr::read_unaligned(ptr);
            self.cursor += std::mem::size_of::<$ty>();
            value.to_be()
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
    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_be_bytes([
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
    pub fn read_u64(&mut self) -> u64 {
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
    #[inline]
    pub unsafe fn read_f32_unchecked(&mut self) -> f32 {
        let value = self.read_u32_unchecked();
        std::mem::transmute::<u32, f32>(value)
    }
    /// 读取一个 f64 类型的数据
    /// 转换大小端
    ///
    /// # 安全性
    /// 允许未对齐的地址
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_f64_unchecked(&mut self) -> f64 {
        let value = self.read_u64_unchecked();
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
    pub unsafe fn read_i8_array_unchecked(&mut self, len: usize) -> Vec<i8> {
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
    /// 读取指定长度的 utf-8 字符串
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 panic
    #[inline]
    pub fn read_string(&mut self, len: usize) -> Result<String, NbtError> {
        if len + self.cursor > self.data.len() {
            return Err(NbtError::OutOfRangeErr(self.cursor, len, self.data.len()));
        }
        let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + len]);
        self.cursor += len;
        Ok(value.into_owned())
    }
    /// 读取指定长度的 i32 数组
    ///
    /// # 安全性
    ///
    /// 长度溢出会导致 UB
    #[inline]
    pub unsafe fn read_i32_array_unchecked(&mut self, len: usize) -> Vec<i32> {
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
    pub unsafe fn read_i64_array_unchecked(&mut self, len: usize) -> Vec<i64> {
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

    /// 读取一个 NBT byte array
    pub fn read_nbt_i8_array(&mut self) -> Vec<i8> {
        let len = self.read_i32() as usize;
        let value = unsafe { self.read_i8_array_unchecked(len) };
        value
    }

    /// 读取一个 NBT int array
    pub fn read_nbt_i32_array(&mut self) -> Vec<i32> {
        let len = self.read_i32() as usize;
        let value = unsafe { self.read_i32_array_unchecked(len) };
        value
    }

    /// 读取一个 NBT long array
    pub fn read_nbt_i64_array(&mut self) -> Vec<i64> {
        let len = self.read_i32() as usize;
        let value = unsafe { self.read_i64_array_unchecked(len) };
        value
    }

    /// 读取一个 NBT string
    pub fn read_nbt_string(&mut self) -> Result<String, NbtError> {
        let len = self.read_u16() as usize;
        self.read_string(len)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    /// 长度: i32
    ByteArray(Vec<i8>),
    /// 8
    /// 或者叫 u8 array
    /// 长度: u16
    String(String),
    /// 9
    /// 长度: i32
    List(Vec<NbtValue>),
    /// 10
    Compound(Option<String>, Vec<(String, NbtValue)>),
    /// 11
    /// 长度: i32
    IntArray(Vec<i32>),
    /// 12
    /// 长度: i32
    LongArray(Vec<i64>),
}

impl NbtValue {
    pub fn type_id_as_name(type_id: u8) -> String {
        if type_id > 12 {
            return format!("未知类型({})", type_id);
        }
        match type_id {
            0 => "NBT_End(0)",
            1 => "NBT_Byte(1)",
            2 => "NBT_Short(2)",
            3 => "NBT_Int(3)",
            4 => "NBT_Long(4)",
            5 => "NBT_Float(5)",
            6 => "NBT_Double(6)",
            7 => "NBT_ByteArray(7)",
            8 => "NBT_String(8)",
            9 => "NBT_List(9)",
            10 => "NBT_Compound(10)",
            11 => "NBT_IntArray(11)",
            12 => "NBT_LongArray(12)",
            _ => unreachable!(),
        }
        .to_string()
    }

    pub fn from_binary(data: &mut [u8]) -> NbtValue {
        let reader = NbtReader::new(data);
        NbtValue::from_reader_unchecked(reader)
    }

    fn read_nbt_compound(reader: &mut NbtReader) -> Vec<(String, NbtValue)> {
        let mut compound = Vec::with_capacity(10);
        loop {
            let tag_id = reader.read_u8();
            if tag_id == 0 {
                break;
            }
            let name = reader.read_nbt_string().unwrap();
            let value = match tag_id {
                1 => NbtValue::Byte(reader.read_i8()),
                2 => NbtValue::Short(reader.read_i16()),
                3 => NbtValue::Int(reader.read_i32()),
                4 => NbtValue::Long(reader.read_i64()),
                5 => NbtValue::Float(reader.read_f32()),
                6 => NbtValue::Double(reader.read_f64()),
                7 => NbtValue::ByteArray(reader.read_nbt_i8_array()),
                8 => NbtValue::String(reader.read_nbt_string().unwrap()),
                9 => NbtValue::List(NbtValue::read_nbt_list(reader)),
                10 => NbtValue::Compound(None, NbtValue::read_nbt_compound(reader)),
                11 => NbtValue::IntArray(reader.read_nbt_i32_array()),
                12 => NbtValue::LongArray(reader.read_nbt_i64_array()),
                _ => unimplemented!(),
            };
            compound.push((name, value));
        }
        compound
    }

    fn read_nbt_list(reader: &mut NbtReader) -> Vec<NbtValue> {
        let type_id = reader.read_u8();
        let len = reader.read_i32() as usize;
        let mut list = Vec::with_capacity(len);
        match type_id {
            1 => {
                for _ in 0..len {
                    list.push(NbtValue::Byte(reader.read_i8()));
                }
            }
            2 => {
                for _ in 0..len {
                    list.push(NbtValue::Short(reader.read_i16()));
                }
            }
            3 => {
                for _ in 0..len {
                    list.push(NbtValue::Int(reader.read_i32()));
                }
            }
            4 => {
                for _ in 0..len {
                    list.push(NbtValue::Long(reader.read_i64()));
                }
            }
            5 => {
                for _ in 0..len {
                    list.push(NbtValue::Float(reader.read_f32()));
                }
            }
            6 => {
                for _ in 0..len {
                    list.push(NbtValue::Double(reader.read_f64()));
                }
            }
            7 => {
                for _ in 0..len {
                    list.push(NbtValue::ByteArray(reader.read_nbt_i8_array()));
                }
            }
            8 => {
                for _ in 0..len {
                    list.push(NbtValue::String(reader.read_nbt_string().unwrap()));
                }
            }
            9 => {
                for _ in 0..len {
                    list.push(NbtValue::List(NbtValue::read_nbt_list(reader)));
                }
            }
            10 => {
                for _ in 0..len {
                    list.push(NbtValue::Compound(None, NbtValue::read_nbt_compound(reader)));
                }
            }
            11 => {
                for _ in 0..len {
                    list.push(NbtValue::IntArray(reader.read_nbt_i32_array()));
                }
            }
            12 => {
                for _ in 0..len {
                    list.push(NbtValue::LongArray(reader.read_nbt_i64_array()));
                }
            }
            _ => unimplemented!(),
        }
        list
    }

    pub fn from_reader_unchecked(mut reader: NbtReader) -> NbtValue {
        // 第一个 tag, 不可能是 0
        match reader.read_u8() {
            9 => NbtValue::List(NbtValue::read_nbt_list(&mut reader)),
            10 => {
                let name = reader.read_nbt_string().unwrap();
                NbtValue::Compound(Some(name), NbtValue::read_nbt_compound(&mut reader))
            }
            x => {
                panic!("根节点类型错误 {}", Self::type_id_as_name(x));
            }
        }
    }
}
