use deku::prelude::*;

#[derive(Debug, PartialEq, Clone, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", ctx = "_endian: deku::ctx::Endian")]
pub enum Flag {
    #[deku(id = "0x0")]
    Unset,
    #[deku(id = "0x1")]
    Set,
}
