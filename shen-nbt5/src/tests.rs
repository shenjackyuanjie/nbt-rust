use crate::NbtReader;

/// 生成测试数据
pub fn gen_datas(len: usize) -> Vec<u8> {
    let mut datas = Vec::with_capacity(len);
    for i in 0..len {
        datas.push(i as u8);
    }
    datas
}

mod safe_test {
    use super::*;

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
    fn read_x16() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_i16(), 0x0102);
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.read_u16(), 0x0304);
        assert_eq!(reader.cursor, 4);
    }

    #[test]
    fn read_x32() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04];
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_i32(), 0x01020304);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_u32(), 0x01020304);
        assert_eq!(reader.cursor, 8);
    }

    #[test]
    fn read_x64() {
        let mut data = vec![
            0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02,
            0x03, 0x04,
        ];
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_i64(), 0x0102030401020304);
        assert_eq!(reader.cursor, 8);
        assert_eq!(reader.read_u64(), 0x0102030401020304);
        assert_eq!(reader.cursor, 16);
    }

    #[test]
    fn read_fxx() {
        let mut data = vec![
            0x40, 0x49, 0x0f, 0xdb, 0x40, 0x49, 0x0f, 0xdb, 0x40, 0x49, 0x0f, 0xdb, 0x40, 0x49,
            0x0f, 0xdb,
        ];
        let mut reader = NbtReader::new(&mut data);
        println!("{}", f32::from_be_bytes([0x40, 0x49, 0x0f, 0xdb]));
        assert_eq!(reader.read_f32(), 3.1415927);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_f64(), 3.14159265);
        assert_eq!(reader.cursor, 12);
    }
}

mod unsafe_test {
    use super::*;
}

#[test]
fn read_array() {
    let mut data = vec![0x01, 0x02, 0x03, 0x04];
    let mut reader = NbtReader::new(&mut data);
    assert_eq!(reader.read_u8_array(2), &[0x01, 0x02]);
    assert_eq!(reader.cursor, 2);
    assert_eq!(reader.read_i8_array_unchecked(2), &[0x03, 0x04]);
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
