use alloc::format;
use deku::{
    bitvec::{BitSlice, BitVec, Msb0}, prelude::*
};

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Flag {
    Unset = 0x0,
    Set = 0x1,
}

impl Default for Flag {
    fn default() -> Self { Flag::Unset }
}

impl<'a, Ctx> DekuRead<'a, Ctx> for Flag
where
    Ctx: Copy,
    u8: DekuRead<'a, Ctx>,
{
    fn read(
        input: &'a BitSlice<Msb0, u8>,
        inner_ctx: Ctx,
    ) -> Result<(&'a BitSlice<Msb0, u8>, Self), DekuError> {
        let (rest, val) = u8::read(input, inner_ctx)?;

        let ret = match val {
            0x00 => Ok(Flag::Unset),
            0x01 => Ok(Flag::Set),
            _ => Err(DekuError::Parse(format!("cannot parse Flag value: {}", val))),
        }?;

        Ok((rest, ret))
    }
}

impl<Ctx> DekuWrite<Ctx> for Flag
where
    u8: DekuWrite<Ctx>,
{
    fn write(&self, output: &mut BitVec<Msb0, u8>, inner_ctx: Ctx) -> Result<(), DekuError> {
        match self {
            Flag::Unset => (0x00u8).write(output, inner_ctx),
            Flag::Set => (0x01u8).write(output, inner_ctx),
        }
    }
}
