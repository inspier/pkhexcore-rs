#![allow(non_snake_case)]
use alloc::format;
use deku::prelude::*;

#[allow(non_camel_case_types)]
/// Gender a PKM can have
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Gender {
    Male = 0x0,
    Female = 0x1,
    Genderless = 0x2,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl<'a, Ctx> DekuRead<'a, Ctx> for Gender
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
            0x00 => Ok(Gender::Male),
            0x01 => Ok(Gender::Female),
            0x02 => Ok(Gender::Genderless),
            _ => Err(DekuError::Parse(format!("cannot parse Gender value: {}", val))),
        }?;

        Ok((rest, ret))
    }
}

impl<Ctx> DekuWrite<Ctx> for Gender
where
    u8: DekuWrite<Ctx>,
{
    /// wrapper around u8::write with consideration to context, such as bit size
    fn write(&self, output: &mut BitVec<Msb0, u8>, inner_ctx: Ctx) -> Result<(), DekuError> {
        match self {
            Gender::Male => (0x00u8).write(output, inner_ctx),
            Gender::Female => (0x01u8).write(output, inner_ctx),
            Gender::Genderless => (0x02u8).write(output, inner_ctx),
        }
    }
}

#[allow(non_upper_case_globals)]
impl Gender {
    pub const Random: Gender = Gender::Genderless;
}

impl_from! (Gender for u8, i32);
