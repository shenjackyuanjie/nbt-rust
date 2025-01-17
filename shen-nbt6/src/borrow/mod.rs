use crate::traits::{NbtBorrowTrait};
use crate::{nbt_version, NbtReader, NbtResult, NbtTypeId};
#[cfg(test)]
mod tests;

/// 实现
pub mod impls;

/// 这里的所有 usize 实际上都指向一个 &[u8]
///
/// 用于更快速的解析 Nbt 数据
///
/// 所有 usize 都指向对应数据的开始位置
#[derive(Debug, Clone, PartialEq)]
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
    /// ptr, len, type_id, values
    List(usize, usize, NbtTypeId, Vec<BorrowNbtValue>),
    /// ptr, str_len, vec<(str_len, BorrowNbtValue)>
    /// 如果是 None, 则表示没有名称
    /// 否则表示有名称(0 != 无名称)
    Compound(usize, Option<usize>, Vec<(usize, BorrowNbtValue)>),
    /// ptr, len
    IntArray(usize, usize),
    /// ptr, len
    LongArray(usize, usize),
}

impl BorrowNbtValue {
    /// 方便的创建一个没有名字的 Compound
    pub fn nameless_compound(ptr: usize, values: Vec<(usize, BorrowNbtValue)>) -> Self {
        Self::Compound(ptr, None, values)
    }
    /// 方便的创建一个有名字的 Compound
    pub fn named_compound(
        ptr: usize,
        name_len: usize,
        values: Vec<(usize, BorrowNbtValue)>,
    ) -> Self {
        Self::Compound(ptr, Some(name_len), values)
    }
    /// 方便的创建一个 Compound 里的 Compound
    pub fn sub_compound(
        name_len: usize,
        ptr: usize,
        values: Vec<(usize, BorrowNbtValue)>,
    ) -> (usize, Self) {
        (name_len, Self::Compound(ptr, None, values))
    }
    /// 方便的创建一个 Compound 里的 List
    pub fn sub_list(
        name_len: usize,
        ptr: usize,
        len: usize,
        type_id: NbtTypeId,
        values: Vec<BorrowNbtValue>,
    ) -> (usize, Self) {
        (name_len, Self::List(ptr, len, type_id, values))
    }

    pub fn from_binary<R>(data: &[u8]) -> NbtResult<(NbtReader, BorrowNbtValue)>
    where
        R: NbtBorrowTrait,
    {
        let mut reader = NbtReader::new(data);
        let data = R::from_reader(&mut reader)?;
        Ok((reader, data))
    }
}

impl NbtBorrowTrait for nbt_version::Java {
    fn from_reader(reader: &mut NbtReader) -> NbtResult<BorrowNbtValue> {
        impls::java_read::java_from_reader(reader, true)
    }
    fn read_data(&self, reader: &mut NbtReader) -> NbtResult<crate::NbtValue> {
        todo!()
    }
}

impl NbtBorrowTrait for nbt_version::JavaNetAfter1_20_2 {
    fn from_reader(reader: &mut NbtReader) -> NbtResult<BorrowNbtValue> {
        impls::java_read::java_from_reader(reader, false)
    }
    fn read_data(&self, reader: &mut NbtReader) -> NbtResult<crate::NbtValue> {
        todo!()
    }
}
