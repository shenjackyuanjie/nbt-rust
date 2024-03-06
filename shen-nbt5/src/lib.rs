

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
    /// 8
    String(&'value str),
    /// 7
    ByteArray(Vec<i8>),
    /// 11
    IntArray(Vec<i32>),
    /// 12
    LongArray(Vec<i64>),
    /// 9
    List(Vec<NbtValue<'value>>),
    /// 10
    Compound(Vec<(String, NbtValue<'value>)>),
}

