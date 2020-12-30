use uint::static_assertions::_core::ops::Shl;

pub trait MinBigEndian
where
    Self: Sized,
{
    fn to_min_be(&self) -> Vec<u8>;

    fn try_from_min_be(input: &[u8]) -> Option<Self>;
}

macro_rules! define_minimal_big_endian {
    ($type: ident) => {
        impl MinBigEndian for $type {
            fn to_min_be(&self) -> Vec<u8> {
                let be = self.to_be_bytes();

                for i in 0..be.len() {
                    if be[i] != 0 {
                        return Vec::from(&be[i..]);
                    }
                }

                vec![0x00];
            }

            fn try_from_min_be(input: &[u8]) -> Option<Self> {
                let mut result: Self = 0x00;

                for byte in input {
                    match result
                        .checked_shl(8)
                        .and_then(|value| value.checked_add(byte as Self))
                    {
                        Some(value) => result = value,
                        None => return None,
                    }
                }

                Some(result)
            }
        }
    };
}

define_minimal_big_endian!(u8);
define_minimal_big_endian!(u16);
define_minimal_big_endian!(u32);
define_minimal_big_endian!(u64);
define_minimal_big_endian!(u128);
define_minimal_big_endian!(usize);
