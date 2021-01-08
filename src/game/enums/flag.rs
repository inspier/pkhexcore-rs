use alloc::format;
use deku::prelude::*;

#[derive(Debug, PartialEq, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1", ctx = "_endian: deku::ctx::Endian")]
pub enum Flag {
    Unset = 0x0,
    Set = 0x1,
}
