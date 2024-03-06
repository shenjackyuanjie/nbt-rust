use crate::{NbtReader, NbtValue};

#[test]
fn basic_init() {
    let data = vec![0x01, 0x02, 0x03, 0x04];
    let reader = NbtReader::new(&data);
    assert_eq!(reader.cursor, 0);
    assert_eq!(reader.data, &data);
}


#[test]
fn read_i8() {
    let data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&data);
    assert_eq!(reader.read_i8(), 0x01);
    assert_eq!(reader.cursor, 1);
    assert_eq!(reader.read_i8(), 0x02);
    assert_eq!(reader.cursor, 2);
}

#[test]
fn read_array() {
    let data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&data);
    assert_eq!(reader.read_u8_array(2), &[0x01, 0x02]);
    assert_eq!(reader.cursor, 2);
    assert_eq!(reader.read_i8_array(2), &[0x03, 0x04]);
    assert_eq!(reader.cursor, 4);
}

#[test]
fn read_long_array() {
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let mut reader = NbtReader::new(&data);
    assert_eq!(reader.read_long_array(1), &[i64::from_ne_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08])]);
    assert_eq!(reader.cursor, 8);
}
