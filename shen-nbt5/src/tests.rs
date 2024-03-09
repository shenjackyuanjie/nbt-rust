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
        let mut data: Vec<u8> = vec![0x01, 0x02, i8::MIN as u8, u8::MAX];
        let mut reader = NbtReader::new(data.as_mut_slice());
        assert_eq!(reader.read_i8(), 0x01);
        assert_eq!(reader.cursor, 1);
        assert_eq!(reader.read_u8(), 0x02);
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.read_i8(), i8::MIN);
        assert_eq!(reader.cursor, 3);
        assert_eq!(reader.read_u8(), u8::MAX);
    }

    #[test]
    fn read_x16() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04];
        data.extend(i16::MIN.to_be_bytes());
        data.extend(i16::MAX.to_be_bytes());
        let mut reader = NbtReader::new(&mut data);
        assert_eq!(reader.read_i16(), 0x0102);
        assert_eq!(reader.cursor, 2);
        assert_eq!(reader.read_u16(), 0x0304);
        assert_eq!(reader.cursor, 4);
        assert_eq!(reader.read_i16(), i16::MIN);
        assert_eq!(reader.cursor, 6);
        assert_eq!(reader.read_i16(), i16::MAX);
        assert_eq!(reader.cursor, 8);
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

/// unsafe 测试
///
/// 实际内容与 safe_test 一致
///
/// 测试方法就是 safe 读一遍，然后 unsafe 读一遍，然后比较结果
///
/// 反正只要 safe 测试过了，unsafe 直接参考 safe 的测试结果就行
mod unsafe_test {
    use super::*;

    #[test]
    fn read_x16() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_i16_unchecked();
            reader.roll_back(2);
            let safe_value = reader.read_i16();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 2);
            let value = reader.read_u16_unchecked();
            reader.roll_back(2);
            let safe_value = reader.read_u16();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 4);
        }
    }

    #[test]
    fn read_x32() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04];
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_i32_unchecked();
            reader.roll_back(4);
            let safe_value = reader.read_i32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 4);
            let value = reader.read_u32_unchecked();
            reader.roll_back(4);
            let safe_value = reader.read_u32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 8);
        }
    }

    #[test]
    fn read_x64() {
        let mut data = vec![
            0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02,
            0x03, 0x04,
        ];
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_i64_unchecked();
            reader.roll_back(8);
            let safe_value = reader.read_i64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 8);
            let value = reader.read_u64_unchecked();
            reader.roll_back(8);
            let safe_value = reader.read_u64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 16);
        }
    }

    #[test]
    fn read_fxx() {
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&std::f32::consts::PI.to_be_bytes());
        data.extend_from_slice(&std::f64::consts::PI.to_be_bytes());
        println!("{:?}", data);
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_f32_unchecked();
            reader.roll_back(4);
            let safe_value = reader.read_f32();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 4);
            let value = reader.read_f64_unchecked();
            reader.roll_back(8);
            let safe_value = reader.read_f64();
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 12);
        }
    }

    #[test]
    fn read_array() {
        let mut data = gen_datas(100);
        let mut reader = NbtReader::new(&mut data);
        unsafe {
            let value = reader.read_i8_array_unchecked(100);
            reader.roll_back(100);
            let safe_value = reader.read_i8_array(100);
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 100);
        }
    }

    #[test]
    fn read_i32_array() {
        let mut value = gen_datas(4 * 100);
        let mut reader = NbtReader::new(&mut value);
        unsafe {
            let value = reader.read_i32_array_unchecked(100);
            reader.roll_back(100 * 4);
            let safe_value = reader.read_i32_array(100);
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 100 * 4);
        }
    }

    #[test]
    fn read_i64_array() {
        let mut value = gen_datas(8 * 100);
        let mut reader = NbtReader::new(&mut value);
        unsafe {
            let value = reader.read_i64_array_unchecked(100);
            reader.roll_back(100 * 8);
            let safe_value = reader.read_i64_array(100);
            assert_eq!(value, safe_value);
            assert_eq!(reader.cursor, 100 * 8);
        }
    }
}
