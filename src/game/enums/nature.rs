#![allow(non_snake_case)]
use alloc::format;
use deku::prelude::*;

#[repr(u8)]
/// Nature ID values for the corresponding English nature name.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u8", ctx = "_endian: deku::ctx::Endian")]
pub enum Nature {
    #[deku(id = "0")]
    Hardy,
    #[deku(id = "1")]
    Lonely,
    #[deku(id = "2")]
    Brave,
    #[deku(id = "3")]
    Adamant,
    #[deku(id = "4")]
    Naughty,
    #[deku(id = "5")]
    Bold,
    #[deku(id = "6")]
    Docile,
    #[deku(id = "7")]
    Relaxed,
    #[deku(id = "8")]
    Impish,
    #[deku(id = "9")]
    Lax,
    #[deku(id = "10")]
    Timid,
    #[deku(id = "11")]
    Hasty,
    #[deku(id = "12")]
    Serious,
    #[deku(id = "13")]
    Jolly,
    #[deku(id = "14")]
    Naive,
    #[deku(id = "15")]
    Modest,
    #[deku(id = "16")]
    Mild,
    #[deku(id = "17")]
    Quiet,
    #[deku(id = "18")]
    Bashful,
    #[deku(id = "19")]
    Rash,
    #[deku(id = "20")]
    Calm,
    #[deku(id = "21")]
    Gentle,
    #[deku(id = "22")]
    Sassy,
    #[deku(id = "23")]
    Careful,
    #[deku(id = "24")]
    Quirky,
    #[deku(id = "25")]
    Random,
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
