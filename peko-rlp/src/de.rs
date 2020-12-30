use crate::be::MinBigEndian;
use crate::error::Error;
use crate::error::UnsupportedType;
use crate::parser::{parse, Item};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

fn from_bytes<'de, T: Deserialize<'de>>(input: &'de [u8]) -> Result<T, Error> {
    let mut deserializer = RLPDeserializer::from_bytes(value)?;
    T::deserialize(&mut deserializer)
}

struct RLPDeserializer<'de> {
    item: Item<'de>,
}

impl<'de> RLPDeserializer<'de> {
    fn from_bytes(input: &[u8]) -> Result<RLPDeserializer, Error> {
        let parsed_item = parse(input)?;

        Ok(RLPDeserializer { item: parsed_item })
    }
}

impl<'de> Deserializer<'de> for &'de mut RLPDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.item {
            Item::ByteArray(_) => self.deserialize_bytes(visitor),
            Item::Sequence(_) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType(UnsupportedType::I8))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType(UnsupportedType::I16))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType(UnsupportedType::I32))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType(UnsupportedType::I64))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.item {
            Item::ByteArray(byte_array) => visitor.visit_u8(u8::try_from_min_be(byte_array)?),
            _ => Err(Error::NotByteArray),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}
