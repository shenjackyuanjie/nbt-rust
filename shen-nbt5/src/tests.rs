use crate::{NbtReader, NbtValue};

/// 生成测试数据
pub fn gen_datas(len: usize) -> Vec<u8> {
    let mut datas = Vec::with_capacity(len);
    for i in 0..len {
        datas.push(i as u8);
    }
    datas
}

#[test]
fn basic_init() {
    let mut data = vec![0x01, 0x02, 0x03, 0x04];
    let reader = NbtReader::new(&mut data);
    assert_eq!(reader.cursor, 0);
    let same_data = vec![0x01, 0x02, 0x03, 0x04];
    assert_eq!(reader.data, &same_data);
}

#[test]
fn read_i8() {
    let mut data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&mut data);
    assert_eq!(reader.read_i8(), 0x01);
    assert_eq!(reader.cursor, 1);
    assert_eq!(reader.read_i8(), 0x02);
    assert_eq!(reader.cursor, 2);
}

#[test]
fn read_one_bytes() {
    let mut data = vec![0x01, 0x02];
    let mut reader = NbtReader::new(&mut data);
    assert_eq!(reader.read_u8(), 0x01);
    assert_eq!(reader.cursor, 1);
    assert_eq!(reader.read_u8(), 0x02);
    assert_eq!(reader.cursor, 2);
}

#[test]
fn read_data_unchecked() {
    let mut datas = gen_datas(100);
    let mut reader = NbtReader::new(&mut datas);
    assert_eq!(reader.read_u8(), 0x00);
    assert_eq!(reader.cursor, 1);
    assert_eq!(reader.read_i8(), 0x01);
    assert_eq!(reader.cursor, 2);
    assert_eq!(reader.read_u8(), 0x02);
    assert_eq!(reader.cursor, 3);
    assert_eq!(reader.read_i16_unchecked(), 0x0304);
    assert_eq!(reader.cursor, 5);
    assert_eq!(reader.read_u16_unchecked(), 0x0506);
    assert_eq!(reader.cursor, 7);
    assert_eq!(reader.read_i32_unchecked(), 0x0708090A);
    assert_eq!(reader.cursor, 11);
    assert_eq!(reader.read_u32_unchecked(), 0x0B0C0D0E);
    assert_eq!(reader.cursor, 15);
    assert_eq!(reader.read_i64_unchecked(), 0x0F10111213141516);
    assert_eq!(reader.cursor, 23);
    assert_eq!(reader.read_u64_unchecked(), 0x1718191A1B1C1D1E);
    assert_eq!(reader.cursor, 31);
    assert_eq!(reader.read_f32_unchecked().to_ne_bytes(), [0x1F, 0x20, 0x21, 0x22]);
    assert_eq!(
        reader.read_f64_unchecked().to_ne_bytes(),
        [0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A]
    );
}

#[test]
fn read_array() {
    let mut data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&mut data);
    assert_eq!(reader.read_u8_array(2), &[0x01, 0x02]);
    assert_eq!(reader.cursor, 2);
    assert_eq!(reader.read_i8_array(2), &[0x03, 0x04]);
    assert_eq!(reader.cursor, 4);
}

#[test]
fn read_int_array() {
    let mut value = 1234567890_i32.to_be_bytes();
    let mut reader = NbtReader::new(&mut value);
    assert_eq!(reader.read_int_array(1), &[1234567890_i32]);
    assert_eq!(reader.cursor, 4);
}

#[test]
fn read_long_array() {
    let mut value = 1234567890_i64.to_be_bytes();
    let mut reader = NbtReader::new(&mut value);
    assert_eq!(reader.read_long_array(1), &[1234567890_i64]);
    assert_eq!(reader.cursor, 8);
}
