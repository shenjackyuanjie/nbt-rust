use crate::NbtReader;

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
fn read_x8() {
    let mut data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&mut data);
    assert_eq!(reader.read_i8(), 0x01);
    assert_eq!(reader.cursor, 1);
    assert_eq!(reader.read_u8(), 0x02);
    assert_eq!(reader.cursor, 2);
}

#[test]
fn read_data_safe() {
    let mut data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&mut data);
    assert_eq!(reader.read_u8(), 0x01);
    assert_eq!(reader.cursor, 1);
    assert_eq!(reader.read_i8(), 0x02);
    assert_eq!(reader.cursor, 2);
    assert_eq!(reader.read_u8(), 0x03);
    assert_eq!(reader.cursor, 3);
    assert_eq!(reader.read_i8(), 0x04);
    assert_eq!(reader.cursor, 4);
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
    assert_eq!(reader.read_i32_array_unchecked(1), &[1234567890_i32]);
    assert_eq!(reader.cursor, 4);
}

#[test]
fn read_long_array() {
    let mut value = 1234567890_i64.to_be_bytes();
    let mut reader = NbtReader::new(&mut value);
    assert_eq!(reader.read_i64_array(1), &[1234567890_i64]);
    assert_eq!(reader.cursor, 8);
}
