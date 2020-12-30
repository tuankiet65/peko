//! Low-level RLP parser.
//!
//! This module provides a low-level parser which parses a RLP input into a [`Item`],
//! which is a recursive structure containing the RLP-encoded data. [`de::RLPDeserializer`]
//! then turns this structure into the Serde data model. The parser can also be used by the
//! application if it requires low-level access to the RLP data. [`Item`] can't be used to
//! construct new RLP data; use [`ser::RLPSerializer`] for that.

use crate::be::MinBigEndian;
use crate::error::{Error, Result};

pub enum Item<'a> {
    ByteArray(&'a [u8]),
    Sequence(Vec<Item<'a>>),
}

/// Parse the input as a RLP-encoded byte array.
///
/// On success, returns a tuple containing two items: the decoded byte array as an [`Item`], and
/// a slice of any trailing data.
fn parse_byte_array(input: &[u8]) -> Result<(Item, &[u8])> {
    let length_marker = input.get(0).ok_or(Err(Error::EOF))?;

    // Determine the length of the byte array, and where does it start.
    let (length, data_start) = match length_marker {
        // Byte array of one element smaller than 128.
        0..=127 => (1, &input[1..]),

        // Byte array shorter than 56 elements.
        // Follows the first byte is the byte array itself.
        128..=183 => ((length_marker - 128) as usize, &input[1..]),

        // Byte array 56 elements or longer, but shorter than 2^64.
        // Follows the first byte is a byte array encoding the actual byte array,
        // so we'll have to decode it to get the length.
        184..=191 => {
            let length_be_length = (length_marker - 183) as usize;
            let input = &input[1..];

            if input.len() < length_be_length {
                return Err(Error::EOF);
            }

            let length = usize::try_from_min_be(&input[..length_be_length]);
            match length {
                Some(length) => (length, &input[length_be_length..]),
                None => return Err(Error::OverflowedIntegerForType),
            }
        }

        _ => Err(Error::NotAByteArray),
    };

    if data_start.len() < length {
        Err(Error::EOF)
    } else {
        Ok((
            Item::ByteArray(&data_start[..length]),
            &data_start[length..],
        ))
    }
}

/// Parse the input as a RLP-encoded sequence of RLP-encoded items.
///
/// On success, returns a tuple containing two items: the decoded sequence an [`Item`], and
/// a slice of any trailing data. The parser recursively decodes all RLP-encoded items in the
/// sequence.
fn parse_sequence(input: &[u8]) -> Result<(Item, &[u8])> {
    // First byte in the input encodes the length of the sequence.
    let length_marker = input.get(0).ok_or(Err(Error::EOF))?;

    // Determine the length of the sequence, and where does it start.
    let (length, data_start) = match length_marker {
        192..=247 => ((length_marker - 192) as usize, &input[1..]),

        248..=255 => {
            let length_be_length = (length_marker - 247) as usize;
            let input = &input[1..];

            if input.len() < length_be_length {
                return Err(Error::EOF);
            }

            let length = usize::try_from_min_be(&input[..length_be_length]);
            match length {
                Some(length) => (length, &input[length_be_length..]),
                None => return Err(Error::OverflowedIntegerForType),
            }
        }

        _ => Err(Error::NotASequence),
    };

    if data_start.len() < length {
        Err(Error::EOF)
    } else {
        let mut sequence = Vec::new();

        let mut current_trailing = &input[..length];
        while !current_trailing.is_empty() {
            let (item, trailing) = try_parse(current_trailing)?;
            sequence.push(item);
            current_trailing = trailing;
        }

        Ok((Item::Sequence(sequence), &input[length..]))
    }
}

fn try_parse(input: &[u8]) -> Result<(Item, &[u8])> {
    parse_byte_array(input).or_else(|_| parse_sequence(input))
}

pub fn parse(input: &[u8]) -> Result<Item> {
    try_parse(input).and_then(|result| {
        if result.1.is_empty() {
            Ok(result.0)
        } else {
            Err(Error::TrailingData)
        }
    })
}
