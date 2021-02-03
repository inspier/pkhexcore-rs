use crate::game::enums::game_version::GameVersion;
use alloc::{string::String, vec::Vec};
use core::convert::TryFrom;
use deku::{ctx::Limit, prelude::*};

pub(crate) mod read {
    use super::*;

    pub(crate) fn read_string_custom(
        rest: &BitSlice<Msb0, u8>,
        byte_count: usize,
        f: fn(&[u16]) -> String,
    ) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
        let (rest, value) = Vec::<u16>::read(rest, Limit::new_count(byte_count))?;
        Ok((rest, f(&value)))
    }

    pub(crate) fn read_game_version(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, GameVersion), DekuError> {
        let (rest, value) = u8::read(rest, ())?;
        Ok((rest, GameVersion::try_from(value as i32).unwrap()))
    }
}

pub(crate) mod write {
    use super::*;

    pub(crate) fn write_string_custom(
        output: &mut BitVec<Msb0, u8>,
        field: Vec<u16>,
    ) -> Result<(), DekuError> {
        field.write(output, ())
    }

    pub(crate) fn write_game_version(
        output: &mut BitVec<Msb0, u8>,
        field: GameVersion,
    ) -> Result<(), DekuError> {
        let value = field as u8;
        value.write(output, ())
    }
}
