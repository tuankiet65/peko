mod be;
mod de;
mod error;
mod parser;
mod ser;

pub use de::RLPDeserializer;
pub use error::{Error, Result};
pub use ser::{to_bytes, RLPSerializer};

// use uint::construct_uint;
//
// construct_uint!(
//     pub struct U256(4);
// );
//
// enum Data {
//     U8(u8),
//     U16(u16),
//     U32(u32),
//     U64(u64),
//     U128(u128),
//     U256(U256),
//     Bytes(Vec<u8>),
//     Vec(Vec<Data>),
// }
//
// fn big_endian_encoding(be: &[u8]) -> Vec<u8> {
//     let first_nonzero = be.iter().find(|b| b != 0);
//
//     match first_nonzero {
//         None => Vec::from(be),
//         Some(position) => Vec::from(be[position..]),
//     }
// }
//
// trait RLPSerializable {
//     fn serialize(&self) -> Vec<u8>;
// }
//
// impl RLPSerializable for u8 {
//     fn serialize(self) -> Vec<u8> {
//         return vec![self];
//     }
// }
//
// impl RLPSerializable for u16 {
//     fn serialize(&self) -> Vec<u8> {
//         big_endian_encoding(&self.to_be_bytes())
//     }
// }
//
// impl RLPSerializable for u32 {
//     fn serialize(&self) -> Vec<u8> {
//         big_endian_encoding(&self.to_be_bytes())
//     }
// }
//
// impl RLPSerializable for u64 {
//     fn serialize(&self) -> Vec<u8> {
//         big_endian_encoding(&self.to_be_bytes())
//     }
// }
//
// impl RLPSerializable for u128 {
//     fn serialize(&self) -> Vec<u8> {
//         big_endian_encoding(&self.to_be_bytes())
//     }
// }
//
// impl RLPSerializable for U256 {
//     fn serialize(&self) -> Vec<u8> {
//         big_endian_encoding(&self.to_be_bytes())
//     }
// }
//
// impl RLPSerializable for Vec<u8> {
//     fn serialize(mut self) -> Vec<u8> {
//         if (self.len() == 1) && (self.get(0) < 128) {
//             vec![self[0]]
//         }
//
//         if self.len() < 56 {
//             let mut result = Vec::<u8>::new();
//             result.push((128 + self.len()) as u8);
//             result.append(&mut self);
//             result
//         }
//
//         let mut length_serialized = self.len().serialize();
//
//         let mut result = Vec::<u8>::new();
//         result.push((128 + self.len()) as u8);
//         result.push((183 + length_serialized.len()) as u8);
//         result.append(&mut length_serialized);
//         result.append(&mut self);
//         result
//     }
// }
//
// impl RLPSerializable for Data {
//     fn serialize(mut self) -> Vec<u8> {
//         match self {
//             Data::U8(value) => value.serialize(),
//             Data::U16(value) => value.serialize(),
//             Data::U32(value) => value.serialize(),
//             Data::U64(value) => value.serialize(),
//             Data::U128(value) => value.serialize(),
//             Data::U256(value) => value.serialize(),
//             Data::Bytes(value) => value.serialize(),
//             Data::Vec(values) => {
//                 let mut values_serialize = Vec::<u8>::new();
//                 for value in values {
//                     values_serialize.append(&mut value.serialize());
//                 }
//
//                 values_serialize.serialize()
//             }
//         }
//     }
// }
