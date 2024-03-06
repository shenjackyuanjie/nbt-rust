

/// 用于读取 NBT 数据
pub struct NbtReader<'data> {
    data: &'data [u8],
    cursor: usize,
}

impl NbtReader<'_> {
    pub fn new(data: &[u8]) -> NbtReader {
        NbtReader {
            data,
            cursor: 0,
        }
    }
    
    pub fn read_i8(&mut self) -> i8 {
        let value = self.data[self.cursor] as i8;
        self.cursor += 1;
        value
    }
    pub fn read_i16(&mut self) -> i16 {
        unsafe {
            let value = *(self.data[self.cursor..].as_ptr() as *const i16);
            self.cursor += 2;
            value.to_be()
        }
    }
    pub fn read_i32(&mut self) -> i32 {
        unsafe {
            let value = *(self.data[self.cursor..].as_ptr() as *const i32);
            self.cursor += 4;
            value.to_be()
        }
    }
    pub fn read_i64(&mut self) -> i64 {
        unsafe {
            let value = *(self.data[self.cursor..].as_ptr() as *const i64);
            self.cursor += 8;
            value.to_be()
        }
    }
}

#[derive(Debug, Clone)]
pub enum NbtValue<'value> {
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
    String(&'value str),
    /// 9
    List(Vec<NbtValue<'value>>),
    /// 10
    Compound(Vec<(String, NbtValue<'value>)>),
    /// 11
    IntArray(Vec<i32>),
    /// 12
    LongArray(Vec<i64>),
}

