use alloc::{string::String, vec::Vec};
use deku::ctx::Limit;
use deku::prelude::*;

pub mod read {
    use super::*;

    pub fn read_string_custom(
        rest: &BitSlice<Msb0, u8>,
        byte_count: usize,
        f: fn(&[u16]) -> String,
    ) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
        let (rest, value) = Vec::<u16>::read(rest, Limit::new_count(byte_count))?;
        Ok((rest, f(&value)))
    }
}

pub mod write {
    use super::*;

    pub fn write_string_custom(
        output: &mut BitVec<Msb0, u8>,
        field: Vec<u16>,
    ) -> Result<(), DekuError> {
        field.write(output, ())
    }
}
