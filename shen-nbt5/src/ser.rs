// use serde::{de as serde_de, forward_to_deserialize_any, ser as serde_ser, Deserialize};

use crate::{nbt_version, NbtError, NbtResult, NbtValue};

compile_error!("Serde support is not yet implemented, awaiting for PR");
// impl serde_ser::Error for NbtError {
//     fn custom<T: Display>(msg: T) -> Self { NbtError::UnknownErr(msg.to_string()) }
// }

// impl serde_de::Error for NbtError {
//     fn custom<T: Display>(msg: T) -> Self { NbtError::UnknownErr(msg.to_string()) }
// }

// pub fn from_bytes<'de, T, V>(bytes: &'de mut [u8]) -> NbtResult<T>
// where
//     T: serde_de::Deserialize<'de>,
//     V: nbt_version::NbtReadTrait,
// {
//     let value = crate::NbtValue::from_binary::<V>(bytes)?;
//     let deserializer = DeserializeNbtValue { value: &value };
//     T::deserialize(deserializer)
// }

// pub struct DeserializeNbtValue<'de> {
//     value: &'de NbtValue,
// }

// impl<'de> DeserializeNbtValue<'de> {
//     pub fn new(value: &'de NbtValue) -> Self { Self { value } }
// }

// pub struct ArrayDeserializer<'de, T> {
//     iter: std::slice::Iter<'de, T>,
// }

// impl <'de, T> ArrayDeserializer<'de, T> {
//     pub fn new(iter: std::slice::Iter<'de, T>) -> Self {
//         Self { iter }
//     }
// }

// impl <'de, T> serde_de::SeqAccess<'de> for ArrayDeserializer<'de, T> {
//     type Error = NbtError;

//     fn next_element_seed<U>(&mut self, seed: U) -> NbtResult<Option<U::Value>>
//     where
//         U: serde_de::DeserializeSeed<'de>,
//     {
//         match self.iter.next() {
//             Some(v) => seed.deserialize(DeserializeNbtValue::new(v)).map(Some),
//             None => Ok(None),
//         }
//     }
    
// }

// impl<'de, 'a> serde_de::Deserializer for DeserializeNbtValue<'de> {
//     forward_to_deserialize_any! {
//         bool i8 i16 i32 i64 f32 f64 char string bytes
//     }

//     fn deserialize_any<V>(self, visitor: V) -> NbtResult<V::Value>
//     where
//         V: serde_de::Visitor<'de>,
//     {
//         match self.value {
//             NbtValue::Byte(v) => visitor.visit_i8(*v),
//             NbtValue::Short(v) => visitor.visit_i16(*v),
//             NbtValue::Int(v) => visitor.visit_i32(*v),
//             NbtValue::Long(v) => visitor.visit_i64(*v),
//             NbtValue::Float(v) => visitor.visit_f32(*v),
//             NbtValue::Double(v) => visitor.visit_f64(*v),
//             NbtValue::ByteArray(v) => visitor.visit_seq(ArrayDeserializer::new(v.iter())),
//             NbtValue::String(v) => visitor.visit_string(v.clone()),
//             NbtValue::List(v) => visitor.visit_seq(ListDeserializer::new(v)),
//             NbtValue::Compound(name, v) => visitor.visit_map(CompoundDeserializer::new(v)),
//             NbtValue::IntArray(v) => visitor.visit_seq(ArrayDeserializer::new(v.iter())),
//             NbtValue::LongArray(v) => visitor.visit_seq(ArrayDeserializer::new(v.iter())),
            
//         }
//     }
// }
