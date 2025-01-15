
/// 这里的所有 usize 实际上都指向一个 &[u8]
/// 
/// 用于更快速的解析 Nbt 数据
pub enum BorrowNbtValue {
    Byte(usize),
    Short(usize),
    Int(usize),
    Long(usize),
    Float(usize),
    Double(usize),
    /// ptr, len
    ByteArray(usize, usize),
    /// ptr, len
    String(usize, usize),
    /// ptr, values
    List(usize, Vec<BorrowNbtValue>),
    /// ptr, str_len, vec<(str_len, BorrowNbtValue)>
    /// 如果是 None, 则表示没有名称
    /// 否则表示有名称(0 != 无名称)
    Compound(usize, Option<usize>, Vec<(usize, BorrowNbtValue)>),
    /// ptr, len
    IntArray(usize, usize),
    /// ptr, len
    LongArray(usize, usize),
}
