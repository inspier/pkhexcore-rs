#![allow(non_snake_case)]
use alloc::format;
use deku::prelude::*;

#[allow(non_camel_case_types)]
/// Gender a PKM can have
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2", ctx = "_endian: deku::ctx::Endian")]
pub enum Gender {
    Male = 0x0,
    Female = 0x1,
    Genderless = 0x2,
}

// #[allow(non_upper_case_globals)]
// impl Gender {
//     pub const Random: Gender = Gender::Genderless;
// }

impl_from! (Gender for u8, i32);
