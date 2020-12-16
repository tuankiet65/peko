//! Cryptographic hash functions.
mod sha256;

/// Trait for a hash function.
///
/// This trait is inspired by
/// [Python's hashlib API](https://docs.python.org/3/library/hashlib.html).
pub trait HashFunction: Sized {
    /// Type of the output digest.
    ///
    /// Technically all hash functions output an array of `u8`, which means
    /// the more ideal way is to specify the array size generically. However
    /// support for const generics (C++: non-type template arguments) hasn't
    /// hit stable yet, so for now we'll have to do with this. `generic-array`
    /// is an option, but I prefer avoiding using external crates.
    type Output;

    /// Create a new instance of the hash function.
    fn new() -> Self;

    /// Update, or add more data to the current hash.
    fn update(&mut self, data: &[u8]);

    /// Return the hash digest of all data that have been added so far.
    fn digest(&self) -> Self::Output;

    /// Calculate the hash digest of an array of bytes. This function is a shortcut to the
    /// new-update-digest process.
    fn hash(data: &[u8]) -> Self::Output {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.digest()
    }
}

pub use sha256::SHA256;
