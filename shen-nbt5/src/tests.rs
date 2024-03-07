use crate::{NbtReader, NbtValue};

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
fn read_one_datas() {
    let mut datas = vec![0x01, 0x02, 0x03, 0x04, 0x05,
                                  0x06, 0x07, 0x08, 0x09, 0x0A,
                                  0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                                  0x01, 0x02, 0x03, 0x04, 0x05,];
    let mut reader = NbtReader::new(&mut datas);
    assert_eq!(reader.read_i8(), 0x01);
    assert_eq!(reader.read_u8(), 0x02);
    assert_eq!(reader.read_i16_unchecked(), 0x0304);
    assert_eq!(reader.read_u16_unchecked(), 0x0506);
    assert_eq!(reader.read_i32_unchecked(), 0x0708090A);
    assert_eq!(reader.read_i64_unchecked(), 0x0B0C0D0E0F010203);
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
