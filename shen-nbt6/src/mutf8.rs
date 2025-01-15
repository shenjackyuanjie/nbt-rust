use std::str::Utf8Error;

use simd_cesu8::mutf8;

#[inline]
/// Check if the given slice is plain ASCII.
///
/// from https://github.com/azalea-rs/simdnbt/blob/master/simdnbt/src/mutf8.rs#L24
fn is_plain_ascii(slice: &[u8]) -> bool {
    for &c in slice {
        if c & 0b10000000 != 0 {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mutf8String {
    data: Vec<u8>,
}

impl Mutf8String {
    pub fn verify(&self) -> Option<Utf8Error> {
        if !is_plain_ascii(&self.data) {
            todo!()
        } else {
            if let Err(e) = std::str::from_utf8(&self.data) {
                return Some(e);
            }
            None
        }
    }

    pub fn decode(&self) -> String {
        if is_plain_ascii(&self.data) {
            unsafe { String::from_utf8_unchecked(self.data.clone()) }
        } else {
            mutf8::decode(&self.data).unwrap_or_default().to_string()
        }
    }
}
