//! Errors returned during the serialization/deserialization process.

use std::fmt;
use std::fmt::Formatter;

use serde::{de, ser};

/// Result type returned by the serializer/deserializer.
///
/// In essence the Error type is fixed to [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Data types which can't be serialized to RLP.
///
/// These are used by [`Error::UnsupportedDataType`] to indicate which type is encountered.
#[derive(Debug)]
pub enum UnsupportedType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Option,
    Unit,
    UnitStruct,
    UnitVariant,
    NewTypeStruct,
    NewTypeVariant,
    TupleStruct,
    TupleVariant,
    StructVariant,
}

impl fmt::Display for UnsupportedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            UnsupportedType::I8 => f.write_str("i8"),
            UnsupportedType::I16 => f.write_str("i16"),
            UnsupportedType::I32 => f.write_str("i32"),
            UnsupportedType::I64 => f.write_str("i64"),
            UnsupportedType::F32 => f.write_str("f32"),
            UnsupportedType::F64 => f.write_str("f64"),
            UnsupportedType::Option => f.write_str("Option"),
            UnsupportedType::Unit => f.write_str("unit"),
            UnsupportedType::UnitStruct => f.write_str("unit_struct"),
            UnsupportedType::UnitVariant => f.write_str("unit_variant"),
            UnsupportedType::NewTypeStruct => f.write_str("newtype_struct"),
            UnsupportedType::NewTypeVariant => f.write_str("newtype_variant"),
            UnsupportedType::TupleStruct => f.write_str("tuple_struct"),
            UnsupportedType::TupleVariant => f.write_str("tuple_variant"),
            UnsupportedType::StructVariant => f.write_str("struct_variant"),
        }
    }
}

/// Errors returned during the serialization/deserialization process.
#[derive(Debug)]
pub enum Error {
    /// Error created through [`serde::ser::Error`] and [`serde::de::Error`]
    Message(String),

    /// Error when trying to serialize/deserialize any data type which can't be expressed in RLP.
    UnsupportedType(UnsupportedType),

    /// Serializer only: when serializing a map, `serialize_key` is called twice. The correct way is
    /// to call `serialize_key` and `serialized_value` in that order once for each key/value pair.
    CallingSerializeKeyTwice,

    /// Serializer only: when serializing a map, serialize_value is called before serialize_key.
    /// The correct way is to call serialize_key and serialized_value in that order once for each
    /// key/value pair.
    CallingSerializeValueWithoutKey,

    /// Deserializer only: the input is truncated, or shorter than the expected length.
    EOF,

    /// Deserializer only: deserialization is successful, but trailing data is present after
    /// the expected data,
    TrailingData,

    /// Deserializer only: input is not a byte array.
    NotByteArray,

    /// Deserializer only: input is not a sequence.
    NotSequence,

    /// Deserializer only: input can't be decoded.
    InvalidInput,

    /// Deserializer only: the target integer type is not big enough to hold the big-endian-encoded
    /// number
    OverflowedIntegerForType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::UnsupportedType(data_type) => {
                f.write_str("unsupported data type: ");
                data_type.fmt(f);
            }
            Error::CallingSerializeKeyTwice => f.write_str("calling serialize_key twice"),
            Error::CallingSerializeValueWithoutKey => {
                f.write_str("calling serialize_value without serialize_key first")
            }

            Error::EOF => f.write_str("EOF while reading input"),
            Error::TrailingData => f.write_str("trailing data"),
            Error::NotByteArray => f.write_str("not a byte array"),
            Error::NotSequence => f.write_str("not a sequence"),
            Error::InvalidInput => f.write_str("invalid input"),
            Error::OverflowedIntegerForType => {
                f.write_str("integer type is too small for big-endian-encoded number")
            }
        };

        Ok(())
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
