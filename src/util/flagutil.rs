use alloc::format;
use deku::prelude::*;

#[derive(Debug, PartialEq, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1", ctx = "_endian: deku::ctx::Endian")]
pub enum Flag {
    #[deku(id = "0x0")]
    Unset,
    #[deku(id = "0x1")]
    Set,
}
