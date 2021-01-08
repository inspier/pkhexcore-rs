use deku::ctx::{Endian, BitSize as Size};
use deku::prelude::*;

pub(crate) fn pad_before_read<T: DekuRead>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: usize,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let mut tmp_rest = rest;
    for _i in 0..pad_size {
        let (rest, _) = u8::read(tmp_rest, ())?;
        tmp_rest = rest;
    }
    let (rest, value) = T::read(tmp_rest, ())?;
    Ok((rest, value))
}

pub(crate) fn pad_after_read<T: DekuRead>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: usize,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let (rest, value) = T::read(rest, ())?;
    let mut tmp_rest = rest;
    for _i in 0..pad_size {
        let (rest, _) = u8::read(tmp_rest, ())?;
        tmp_rest = rest;
    }
    Ok((tmp_rest, value))
}

pub(crate) fn pad_bits_before_read<T: DekuRead<Endian>>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: Size,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let mut tmp_rest = rest;
    let (rest, _) = u8::read(tmp_rest, pad_size)?;
    tmp_rest = rest;
    let (rest, value) = T::read(tmp_rest, Endian::Little)?;
    Ok((rest, value))
}

pub(crate) fn pad_bits_after_read<T: DekuRead<Endian>>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: Size,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let (rest, value) = T::read(rest, Endian::Little)?;
    let mut tmp_rest = rest;
    let (rest, _) = u8::read(tmp_rest, pad_size)?;
    tmp_rest = rest;
    Ok((tmp_rest, value))
}

pub(crate) fn pad_before_write<T: DekuRead>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: usize,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let mut tmp_rest = rest;
    for _i in 0..pad_size {
        let (rest, _) = u8::read(tmp_rest, ())?;
        tmp_rest = rest;
    }
    let (rest, value) = T::read(tmp_rest, ())?;
    Ok((rest, value))
}

pub(crate) fn pad_after_write<T: DekuRead>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: usize,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let (rest, value) = T::read(rest, ())?;
    let mut tmp_rest = rest;
    for _i in 0..pad_size {
        let (rest, _) = u8::read(tmp_rest, ())?;
        tmp_rest = rest;
    }
    Ok((tmp_rest, value))
}

pub(crate) fn pad_bits_before_write<T: DekuRead<Endian>>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: Size,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let mut tmp_rest = rest;
    let (rest, _) = u8::read(tmp_rest, pad_size)?;
    tmp_rest = rest;
    let (rest, value) = T::read(tmp_rest, Endian::Little)?;
    Ok((rest, value))
}

pub(crate) fn pad_bits_after_write<T: DekuRead<Endian>>(
    rest: &BitSlice<Msb0, u8>,
    pad_size: Size,
) -> Result<(&BitSlice<Msb0, u8>, T), DekuError> {
    let (rest, value) = T::read(rest, Endian::Little)?;
    let mut tmp_rest = rest;
    let (rest, _) = u8::read(tmp_rest, pad_size)?;
    tmp_rest = rest;
    Ok((tmp_rest, value))
}
