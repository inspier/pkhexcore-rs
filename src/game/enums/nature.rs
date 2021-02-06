#![allow(non_snake_case)]
use alloc::format;
use deku::prelude::*;

/// Nature ID values for the corresponding English nature name.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u8", ctx = "_endian: deku::ctx::Endian")]
#[repr(u8)]
pub enum Nature {
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,
    Random = 25,
}

impl Default for Nature {
    fn default() -> Self { Nature::Hardy }
}

impl_from! (Nature for u8, i32);

#[allow(unused_macros)]
macro_rules! nature {
    ($nature:tt) => {
        string_to_enum!(Nature, $nature)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nature_macro_test() {
        assert_eq!(Nature::Jolly, nature!(Jolly));
    }
}
