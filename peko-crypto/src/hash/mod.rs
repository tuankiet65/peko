//! Cryptographic hash functions.
mod sha256;

/// Trait for a hash function.
///
/// This trait is inspired by
/// [Python's hashlib API](https://docs.python.org/3/library/hashlib.html).
pub trait HashFunction<const OUTPUT_SIZE: usize> {
    /// Type of the output. Must be set to `[u8; OUTPUT_SIZE]` in the implementation.
    type Output;

    /// Create a new instance of the hash function.
    fn new() -> Self;

    /// Update, or add more data to the current hash.
    fn update(&mut self, data: &[u8]);

    /// Return the hash digest of all data that have been added so far.
    fn digest(&self) -> [u8; OUTPUT_SIZE];

    /// Calculate the hash digest of an array of bytes. This function is a shortcut to the
    /// new-update-digest process.
    fn hash(data: &[u8]) -> [u8; OUTPUT_SIZE] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.digest()
    }
}

pub use sha256::SHA256;
