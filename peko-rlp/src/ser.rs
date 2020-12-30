use crate::be::MinBigEndian;
use crate::error::Error;
use crate::error::UnsupportedType;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::fmt::Display;

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut serializer = RLPSerializer::new();
    value.serialize(&mut serializer)?;

    Ok(serializer.output)
}

pub struct RLPSerializer {
    pub output: Vec<u8>,
}

impl RLPSerializer {
    fn new() -> RLPSerializer {
        RLPSerializer { output: vec![] }
    }
}

struct RLPSequenceSerializer<'a> {
    source_serializer: &'a mut RLPSerializer,
    sequence: Vec<Vec<u8>>,
}

impl<'a> RLPSequenceSerializer<'a> {
    fn new(serializer: &'a mut RLPSerializer) -> RLPSequenceSerializer {
        RLPSequenceSerializer {
            source_serializer: serializer,
            sequence: vec![],
        }
    }
}

impl<'a> SerializeSeq for &'a mut RLPSequenceSerializer<'a> {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.sequence.push(to_bytes(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let total_length = self.sequence.iter().map(|element| element.len()).sum();

        if total_length < 56 {
            self.source_serializer
                .output
                .push((192 + total_length) as u8);
        } else {
            let mut total_length_be = total_length.to_min_be();

            self.source_serializer
                .output
                .push((247 + total_length_be.len()) as u8);
            self.source_serializer.output.append(total_length_be);
        }

        self.sequence
            .iter_mut()
            .for_each(|element| self.source_serializer.output.append(element));

        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut RLPSequenceSerializer<'a> {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        // TODO ?????
        (self as dyn SerializeSeq).serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // TODO ????
        (self as dyn SerializeSeq).end()
    }
}

impl<'a> Serializer for &'a mut RLPSerializer {
    type Ok = ();
    type Error = crate::error::Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(v as u8)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::I8))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::I16))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::I32))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::I64))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.to_min_be().as_slice())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.to_min_be().as_slice())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.to_min_be().as_slice())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.to_min_be().as_slice())
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::F32))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::F64))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut utf8_encoded = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut utf8_encoded))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if (v.len() == 1) && (v[0] < 128) {
            self.output.extend(v);
        } else if v.len() < 56 {
            self.output.push((128 + v.len()) as u8);
            self.output.extend(v);
        } else {
            let mut len_be_bytes = v.len().to_min_be();
            self.output.push((183 + len_be_bytes.len()) as u8);
            self.output.append(&mut len_be_bytes);
            self.output.extend(v);
        }

        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::Option))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::Option))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::Unit))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::UnitStruct))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::UnitVariant))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::NewTypeStruct))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::NewTypeVariant))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.items = Some(vec![]);
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.items = Some(vec![]);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::TupleStruct))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::TupleVariant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedDataType(UnsupportedType::StructVariant))
    }

    fn collect_str<T: ?Sized + Display>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        // TODO
        unimplemented!()
    }
}

impl<'a> SerializeSeq for &'a mut RLPSerializer {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        // TODO: is this optimized?

        let mut serialized = to_bytes(value)?;
        self.output.append(&mut serialized);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // TODO: empty list?

        if self.output.len() < 56 {
            self.output.insert(0, (192 + self.output.len()) as u8);
        } else {
            // TODO: need to shorten this.
            let output_len_be = self.output.len().to_be_bytes();
            self.output.insert(0, (247 + output_len_be.len()) as u8);
            // TODO: insert output_len_be at top.
        }

        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut RLPSerializer {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        // TODO: is this optimized?

        let mut serialized = to_bytes(value)?;
        self.output.append(&mut serialized);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // TODO: empty list?

        if self.output.len() < 56 {
            self.output.insert(0, (192 + self.output.len()) as u8);
        } else {
            // TODO: need to shorten this.
            let output_len_be = self.output.len().to_be_bytes();
            self.output.insert(0, (247 + output_len_be.len()) as u8);
            // TODO: insert output_len_be at top.
        }

        Ok(())
    }
}

impl<'a> SerializeMap for &'a mut RLPSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        match self.key {
            None => {
                self.key = Some(to_bytes(key)?);
                Ok(())
            }
            Some(_) => Err(Error::CallingSerializeKeyTwice),
        }
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        match self.key {
            None => Err(Error::CallingSerializeValueWithoutKey),
            Some(&key) => self.items.push(to_bytes([key, to_bytes(value)])),
            // TODO: ????
            _ => {}
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // TODO: end here.
        unimplemented!()
    }
}

impl<'a> SerializeStruct for &'a mut RLPSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeTupleStruct for &'a mut RLPSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeTupleVariant for &'a mut RLPSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeStructVariant for &'a mut RLPSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
