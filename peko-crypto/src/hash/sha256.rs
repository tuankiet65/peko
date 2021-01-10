use crate::hash::HashFunction;

use std::convert::TryInto;

/// The internal SHA256 digest. On output this internal digest will be converted to an
/// array of u8s.
type InternalDigest = [u32; 8];

/// Initial digest value.
const H: InternalDigest = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// Round constants.
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Size of each message chunk in bytes.
const CHUNK_SIZE: usize = 512 / 8;

/// Type of each message chunk to be processed.
type MessageChunk = [u8; CHUNK_SIZE];

/// Produce a new digest by compressing the message chunk, and adding it to the source digest.
fn process_chunk(chunk: &MessageChunk, source_digest: &InternalDigest) -> InternalDigest {
    // Prepare the message schedule.
    let mut w = [0u32; 64];

    // Word 0..15 comes from the chunk.
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            chunk[i * 4],
            chunk[i * 4 + 1],
            chunk[i * 4 + 2],
            chunk[i * 4 + 3],
        ])
    }

    // Extend the first 16 bytes to the remaining words.
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);

        w[i] = w[i - 16]
            .wrapping_add(w[i - 7])
            .wrapping_add(s1)
            .wrapping_add(s0);
    }

    let mut a = source_digest[0];
    let mut b = source_digest[1];
    let mut c = source_digest[2];
    let mut d = source_digest[3];
    let mut e = source_digest[4];
    let mut f = source_digest[5];
    let mut g = source_digest[6];
    let mut h = source_digest[7];

    // Compression function.
    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(w[i])
            .wrapping_add(K[i])
            .wrapping_add(s1)
            .wrapping_add(ch);

        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    [
        source_digest[0].wrapping_add(a),
        source_digest[1].wrapping_add(b),
        source_digest[2].wrapping_add(c),
        source_digest[3].wrapping_add(d),
        source_digest[4].wrapping_add(e),
        source_digest[5].wrapping_add(f),
        source_digest[6].wrapping_add(g),
        source_digest[7].wrapping_add(h),
    ]
}

/// Secure Hashing Algorithm-2 with 256 bits of output.
///
/// # Example using the new-update-digest flow
/// ```
/// use hex_literal::hex;
/// use peko_crypto::hash::{HashFunction, SHA256};
///
/// let data = b"The quick brown fox jumps over the lazy dog";
/// let correct_digest = hex!("d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592");
///
/// let mut hasher = SHA256::new();
/// hasher.update(data);
/// assert_eq!(hasher.digest(), correct_digest)
/// ```
///
/// # Example using the one-shot function
/// ```
/// use hex_literal::hex;
/// use peko_crypto::hash::{HashFunction, SHA256};
///
/// let data = b"The quick brown fox jumps over the lazy dog";
/// let correct_digest = hex!("d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592");
///
/// assert_eq!(SHA256::hash(data), correct_digest);
/// ```
pub struct SHA256 {
    digest: InternalDigest,
    processed_data_length: usize,
    remaining_data: Vec<u8>,
}

impl SHA256 {
    /// Finalize the hash by hashing the remaining messages and any required paddings.
    fn finalize(&self) -> InternalDigest {
        let mut remaining_with_padding: Vec<u8> = self.remaining_data.clone();

        // First byte in the padding is 0b10000000
        remaining_with_padding.push(0b10000000);

        // Pad the remaining with 0x00 so that the message can be divided into chunks of
        // CHUNKS_SIZE bytes, after the total message length is added (below).
        while ((remaining_with_padding.len() + 8) % CHUNK_SIZE) != 0 {
            remaining_with_padding.push(0x00);
        }

        // The last 8 bytes is the total message length, in big-endian.
        let final_length_bytes =
            ((self.processed_data_length + self.remaining_data.len()) * 8).to_be_bytes();
        remaining_with_padding.extend_from_slice(&final_length_bytes);

        assert_eq!(
            remaining_with_padding.len() % CHUNK_SIZE,
            0,
            "Length after padding must be divisible by 64 bytes"
        );

        remaining_with_padding
            .chunks_exact(CHUNK_SIZE)
            .fold(self.digest, |prev_digest, chunk| {
                process_chunk(
                    chunk.try_into().expect("Chunk must be 64 bytes long"),
                    &prev_digest,
                )
            })
    }
}

impl HashFunction<{ 256 / 8 }> for SHA256 {
    type Output = [u8; 256 / 8];

    fn new() -> SHA256 {
        SHA256 {
            digest: H,
            processed_data_length: 0,
            remaining_data: Vec::new(),
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.remaining_data.extend_from_slice(data);

        let iter = self.remaining_data.chunks_exact(CHUNK_SIZE);

        self.digest = iter.clone().fold(self.digest, |prev_digest, chunk| {
            process_chunk(
                chunk.try_into().expect("Chunk must be 64 bytes long"),
                &prev_digest,
            )
        });

        self.processed_data_length += iter.clone().count() * CHUNK_SIZE;

        assert!(
            iter.remainder().len() < CHUNK_SIZE,
            "Remainder chunk must be smaller than 64 bytes"
        );
        self.remaining_data = Vec::from(iter.remainder());
    }

    fn digest(&self) -> Self::Output {
        let mut result = [0u8; 32];

        for (i, value) in self.finalize().iter().enumerate() {
            let be = value.to_be_bytes();
            result[i * 4] = be[0];
            result[i * 4 + 1] = be[1];
            result[i * 4 + 2] = be[2];
            result[i * 4 + 3] = be[3];
        }

        result
    }
}
