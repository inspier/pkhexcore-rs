use alloc::format;
use deku::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Flag {
    Unset = 0x0,
    Set = 0x1,
}

impl Default for Flag {
    fn default() -> Self {
        Flag::Unset
    }
}

impl<'a, Ctx> DekuRead<'a, Ctx> for Flag
where
    Ctx: Copy,
    u8: DekuRead<'a, Ctx>,
{
    /// wrapper around u8::read with consideration to context, such as bit size
    /// true if the result of the read is `1`, false if `0` and error otherwise
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
    /// wrapper around u8::write with consideration to context, such as bit size
    fn write(&self, output: &mut BitVec<Msb0, u8>, inner_ctx: Ctx) -> Result<(), DekuError> {
        match self {
            Flag::Unset => (0x00u8).write(output, inner_ctx),
            Flag::Set => (0x01u8).write(output, inner_ctx),
        }
    }
}
