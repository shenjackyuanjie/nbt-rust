use crate::nbt_version::{BedrockDisk, BedrockNetVarInt, Java, JavaNetAfter1_20_2, NbtWriteTrait};
use crate::{NbtError, NbtResult, NbtValue};

/// 最简单的一集
impl NbtWriteTrait for Java {
    #[inline]
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]) {
        // 写好 tag 了, 直接写入信息
        // 写入长度
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        // 写入数据
        writer.extend_from_slice(data.iter().map(|x| *x as u8).collect::<Vec<u8>>().as_slice());
    }
    #[inline]
    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]) {
        // 写好 tag 了, 直接写入信息
        // 写入长度
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        // 写入数据
        writer.extend_from_slice(
            &data.iter().map(|x| x.to_be_bytes()).collect::<Vec<[u8; 4]>>().concat(),
        );
    }
    #[inline]
    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]) {
        // 写好 tag 了, 直接写入信息
        // 写入长度
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        // 写入数据
        writer.extend_from_slice(
            &data.iter().map(|x| x.to_be_bytes()).collect::<Vec<[u8; 8]>>().concat(),
        );
    }
    #[inline]
    fn write_nbt_string(writer: &mut Vec<u8>, data: &str) {
        // 写入长度
        writer.extend_from_slice(&(data.len() as u16).to_be_bytes());
        // 写入数据
        writer.extend_from_slice(data.as_bytes());
    }
    #[inline]
    fn write_list(writer: &mut Vec<u8>, data: &[NbtValue]) -> NbtResult<()> {
        if data.is_empty() {
            // 写入一个空的 tag
            writer.extend_from_slice(&0i8.to_be_bytes());
            return Ok(());
        }
        // 遍历检查一遍所有的 tag 是否一致
        let tag = data.first().unwrap().tag();
        if !data.iter().all(|x| x.tag() == tag) {
            return Err(NbtError::ListTypeNotSame(data.iter().map(|x| x.tag()).collect()));
        }
        // 写入 tag
        writer.push(tag);
        // 写入长度
        writer.extend_from_slice(&(data.len() as i32).to_be_bytes());
        // 写入数据
        for i in data {
            match i {
                NbtValue::Byte(x) => writer.push(*x as u8),
                NbtValue::Short(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Int(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Long(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Float(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Double(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::ByteArray(x) => Self::write_i8_array(writer, x),
                NbtValue::IntArray(x) => Self::write_i32_array(writer, x),
                NbtValue::LongArray(x) => Self::write_i64_array(writer, x),
                NbtValue::String(x) => Self::write_nbt_string(writer, x),
                NbtValue::List(x) => Self::write_list(writer, x)?,
                NbtValue::Compound(name, data) => {
                    Self::write_compound(writer, name.as_ref(), data)?
                }
            }
        }
        Ok(())
    }
    #[inline]
    fn write_compound(
        writer: &mut Vec<u8>,
        name: Option<&String>,
        data: &[(String, NbtValue)],
    ) -> NbtResult<()> {
        // 写入自己的名字
        Self::write_nbt_string(writer, name.unwrap_or(&"".to_string()));
        for (key, value) in data {
            // 写入 tag
            writer.push(value.tag());
            // 写入 key
            Self::write_nbt_string(writer, key);
            // 写入 value
            match value {
                NbtValue::Byte(x) => writer.push(*x as u8),
                NbtValue::Short(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Int(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Long(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Float(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::Double(x) => writer.extend_from_slice(&x.to_be_bytes()),
                NbtValue::ByteArray(x) => Self::write_i8_array(writer, x),
                NbtValue::IntArray(x) => Self::write_i32_array(writer, x),
                NbtValue::LongArray(x) => Self::write_i64_array(writer, x),
                NbtValue::String(x) => Self::write_nbt_string(writer, x),
                NbtValue::List(x) => Self::write_list(writer, x)?,
                NbtValue::Compound(name, data) => {
                    Self::write_compound(writer, name.as_ref(), data)?
                }
            }
        }
        // 写入结束 tag
        writer.push(0);
        Ok(())
    }
    fn write_to(value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()> {
        // 写入 tag
        match value {
            NbtValue::Compound(name, data) => {
                buff.push(value.tag());
                Self::write_compound(buff, name.as_ref(), data)?
            }
            x => return Err(NbtError::WrongRootType(x.tag())),
        }
        Ok(())
    }
    fn write_to_with_name(name: &str, value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()> {
        // 写入 tag
        buff.push(value.tag());
        // 写入 key
        Self::write_nbt_string(buff, name);
        // 写入 value
        Self::write_to(value, buff)?;
        Ok(())
    }
}

impl NbtWriteTrait for JavaNetAfter1_20_2 {
    #[inline]
    fn write_i8_array(writer: &mut Vec<u8>, data: &[i8]) { Java::write_i8_array(writer, data); }
    #[inline]
    fn write_i32_array(writer: &mut Vec<u8>, data: &[i32]) { Java::write_i32_array(writer, data); }
    #[inline]
    fn write_i64_array(writer: &mut Vec<u8>, data: &[i64]) { Java::write_i64_array(writer, data); }
    #[inline]
    fn write_nbt_string(writer: &mut Vec<u8>, data: &str) { Java::write_nbt_string(writer, data); }
    #[inline]
    fn write_list(writer: &mut Vec<u8>, data: &[NbtValue]) -> NbtResult<()> {
        Java::write_list(writer, data)
    }
    #[inline]
    fn write_compound(
        writer: &mut Vec<u8>,
        name: Option<&String>,
        data: &[(String, NbtValue)],
    ) -> NbtResult<()> {
        Java::write_compound(writer, name, data)
    }
    #[inline]
    fn write_to(value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()> {
        // 写入 tag
        match value {
            NbtValue::Compound(_, data) => {
                // 忽略名字
                buff.push(value.tag());
                for (key, value) in data {
                    // 写入 tag
                    buff.push(value.tag());
                    // 写入 key
                    Self::write_nbt_string(buff, key);
                    // 写入 value
                    match value {
                        NbtValue::Byte(x) => buff.push(*x as u8),
                        NbtValue::Short(x) => buff.extend_from_slice(&x.to_be_bytes()),
                        NbtValue::Int(x) => buff.extend_from_slice(&x.to_be_bytes()),
                        NbtValue::Long(x) => buff.extend_from_slice(&x.to_be_bytes()),
                        NbtValue::Float(x) => buff.extend_from_slice(&x.to_be_bytes()),
                        NbtValue::Double(x) => buff.extend_from_slice(&x.to_be_bytes()),
                        NbtValue::ByteArray(x) => Self::write_i8_array(buff, x),
                        NbtValue::IntArray(x) => Self::write_i32_array(buff, x),
                        NbtValue::LongArray(x) => Self::write_i64_array(buff, x),
                        NbtValue::String(x) => Self::write_nbt_string(buff, x),
                        NbtValue::List(x) => Self::write_list(buff, x)?,
                        NbtValue::Compound(name, data) => {
                            Self::write_compound(buff, name.as_ref(), data)?
                        }
                    }
                }
                // 写入结束 tag
                buff.push(0);
                Ok(())
            }
            x => return Err(NbtError::WrongRootType(x.tag())),
        }
    }
    #[inline]
    fn write_to_with_name(_name: &str, value: &NbtValue, buff: &mut Vec<u8>) -> NbtResult<()> {
        // drop name
        JavaNetAfter1_20_2::write_to(value, buff)
    }
    #[inline]
    fn to_binary(value: &NbtValue) -> NbtResult<Vec<u8>> {
        let mut buff = Vec::new();
        JavaNetAfter1_20_2::write_to(value, &mut buff)?;
        Ok(buff)
    }
}
