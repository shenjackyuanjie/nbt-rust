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
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&std::f32::consts::PI.to_be_bytes());
        data.extend_from_slice(&std::f64::consts::PI.to_be_bytes());
        println!("{:?}", data);
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_f32(), std::f32::consts::PI);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_f64(), std::f64::consts::PI);
        assert_eq!(reader.cursor, 12);
    }

    #[test]
    fn read_string() {
        let mut data = Vec::with_capacity(20);
        data.extend("Hello world!啊？".as_bytes());
        let len = data.len();
        println!("{:?}", data);
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_string(len), "Hello world!啊？");
        assert_eq!(reader.cursor, 18);
    }
}

mod unsafe_test {
    use super::*;

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
}
