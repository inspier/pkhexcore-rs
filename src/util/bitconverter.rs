use log_derive::{logfn, logfn_inputs};
#[allow(unused_imports)]
use std::convert::TryInto;
use std::fmt::Debug;

/// Returns a 16-bit signed integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_int16` function converts the bytes from index start_index to
/// start_index + 1 to a `i16` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_int16;
/// assert_eq!(-256, to_int16(&[0, 255]));
/// ```
///

#[inline]
pub const fn to_int16(data: &[u8; 2]) -> i16 {
    i16::from_le_bytes(*data)
}

/// Returns a 32-bit signed integer converted from four bytes at a specified
/// position in a byte array.
///
/// The `to_int32` function converts the bytes from index start_index to
/// start_index + 3 to a `i32` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_int32;
/// assert_eq!(-265875328, to_int32(&[128, 16, 39, 240]));
/// ```
///

#[inline]
pub const fn to_int32(data: &[u8; 4]) -> i32 {
    i32::from_le_bytes(*data)
}

/// Returns a 64-bit signed integer converted from eight bytes at a specified
/// position in a byte array.
///
/// The `to_int64` function converts the bytes from index start_index to
/// start_index + 7 to a `i64` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_int64;
/// assert_eq!(-1019801265028202496, to_int64(&[0, 0, 128, 16, 39, 240, 216, 241]));
/// ```
///

#[inline]
pub const fn to_int64(data: &[u8; 8]) -> i64 {
    i64::from_le_bytes(*data)
}

/// Returns a 16-bit unsigned integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_uint16` function converts the bytes from index start_index to
/// start_index + 1 to a `u16` value. The order of bytes in the array must
/// reflect the endianness of the  computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_uint16;
/// assert_eq!(65280, to_uint16(&[0, 255]));
/// ```
///

#[inline]
pub const fn to_uint16(data: &[u8; 2]) -> u16 {
    u16::from_le_bytes(*data)
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified
/// position in a byte array.
///
/// The `to_uint32` function converts the bytes from index start_index to
/// start_index + 3 to a `u32` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_uint32;
/// assert_eq!(261888, to_uint32(&[0, 255, 3, 0]));
/// ```
///

#[inline]
pub const fn to_uint32(data: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*data)
}

/// Returns a 64-bit unsigned integer converted from eight bytes at a specified
/// position in a byte array.
///
/// The `to_uint64` function converts the bytes from index start_index to
/// start_index + 7 to a `u64` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_uint64;
/// assert_eq!(255, to_uint64(&[255, 0, 0, 0, 0, 0, 0, 0]));
/// ```
///
#[inline]
pub const fn to_uint64(data: &[u8; 8]) -> u64 {
    u64::from_le_bytes(*data)
}

#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub(crate) fn get_bytes<T: ByteDecomposable + Debug>(value: T) -> ByteArray {
    let data = <T>::get_bytes(value);
    let size = data.len();
    ByteArray::new(data, size)
}

#[derive(Debug)]
pub(crate) struct ByteArray {
    src: Vec<u8>,
    size: usize,
}

impl ByteArray {
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    fn new(data: Vec<u8>, size: usize) -> ByteArray {
        ByteArray { src: data, size }
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub(crate) fn copy_to(self: &Self, dest: &mut [u8], index: u32) {
        let start_index = index as usize;
        let end_index = start_index + self.size;
        dest[start_index..end_index].copy_from_slice(&self.src);
    }
}

pub(crate) trait ByteDecomposable {
    fn get_bytes(data: Self) -> Vec<u8>;
}

macro_rules! impl_get_bytes {
    (for $($t:tt),+) => {
        $(impl_get_bytes!($t, stringify!($t));)*
    };

    ($t:ident, $sname:expr) => {
        impl ByteDecomposable for $t {
            #[doc = "Returns the specified `"]
            #[doc = $sname]
            #[doc = "` value as a byte vector."]
            fn get_bytes(data: $t) -> Vec<u8> {
                <$t>::to_le_bytes(data).to_vec()
            }
        }
    };
}

impl_get_bytes!(for i8, i16, i32, i64, u8, u16, u32, u64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_int16_test_le() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        assert_eq!(15, to_int16(array_two!(buffer, 0)));
        assert_eq!(0, to_int16(array_two!(buffer, 1)));
        assert_eq!(-32768, to_int16(array_two!(buffer, 2)));
        assert_eq!(10000, to_int16(array_two!(buffer, 4)));
        assert_eq!(-10000, to_int16(array_two!(buffer, 6)));
        assert_eq!(-15, to_int16(array_two!(buffer, 8)));
        assert_eq!(32767, to_int16(array_two!(buffer, 9)));
    }

    #[test]
    #[should_panic]
    fn to_int16_panic_test() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        to_int16(array_two!(buffer, 11));
    }

    #[test]
    fn to_int32_test_le() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        assert_eq!(15, to_int32(array_four!(buffer, 0)));
        assert_eq!(268435456, to_int32(array_four!(buffer, 2)));
        assert_eq!(-16773120, to_int32(array_four!(buffer, 4)));
        assert_eq!(67043344, to_int32(array_four!(buffer, 5)));
        assert_eq!(-905969661, to_int32(array_four!(buffer, 8)));
        assert_eq!(-12870966, to_int32(array_four!(buffer, 11)));
        assert_eq!(-50278, to_int32(array_four!(buffer, 12)));
    }

    #[test]
    #[should_panic]
    fn to_int32_panic_test() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        to_int32(array_four!(buffer, 16));
    }

    #[test]
    fn to_int64_test_le() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        assert_eq!(-1000000000, to_int64(array_eight!(buffer, 0)));
        assert_eq!(16777215, to_int64(array_eight!(buffer, 5)));
        assert_eq!(0, to_int64(array_eight!(buffer, 8)));
        assert_eq!(-9223372036854775808, to_int64(array_eight!(buffer, 9)));
        assert_eq!(1000000000, to_int64(array_eight!(buffer, 17)));
        assert_eq!(4294967296, to_int64(array_eight!(buffer, 21)));
        assert_eq!(-4294967296, to_int64(array_eight!(buffer, 26)));
        assert_eq!(-16777215, to_int64(array_eight!(buffer, 34)));
        assert_eq!(-187649984473770, to_int64(array_eight!(buffer, 45)));
        assert_eq!(187649984473770, to_int64(array_eight!(buffer, 53)));
        assert_eq!(1000000000000000000, to_int64(array_eight!(buffer, 59)));
    }

    #[test]
    #[should_panic]
    fn to_int64_panic_test() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        to_int64(array_eight!(buffer, 68));
    }

    #[test]
    fn to_uint16_test_le() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(15, to_uint16(array_two!(buffer, 0)));
        assert_eq!(0, to_uint16(array_two!(buffer, 1)));
        assert_eq!(1023, to_uint16(array_two!(buffer, 3)));
        assert_eq!(10000, to_uint16(array_two!(buffer, 5)));
        assert_eq!(32767, to_uint16(array_two!(buffer, 8)));
        assert_eq!(65535, to_uint16(array_two!(buffer, 7)));
    }

    #[test]
    #[should_panic]
    fn to_uint16_panic_test() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        to_uint16(array_two!(buffer, 9));
    }

    #[test]
    fn to_uint32_test_le() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        assert_eq!(15, to_uint32(array_four!(buffer, 0)));
        assert_eq!(0, to_uint32(array_four!(buffer, 1)));
        assert_eq!(1048576, to_uint32(array_four!(buffer, 3)));
        assert_eq!(1023, to_uint32(array_four!(buffer, 7)));
        assert_eq!(1000000000, to_uint32(array_four!(buffer, 10)));
        assert_eq!(4294967295, to_uint32(array_four!(buffer, 14)));
        assert_eq!(2147483647, to_uint32(array_four!(buffer, 15)));
    }

    #[test]
    #[should_panic]
    fn to_uint32_panic_test() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        to_uint32(array_four!(buffer, 16));
    }

    #[test]
    fn to_uint64_test_le() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        assert_eq!(16777215, to_uint64(array_eight!(buffer, 0)));
        assert_eq!(0, to_uint64(array_eight!(buffer, 3)));
        assert_eq!(4294967296, to_uint64(array_eight!(buffer, 7)));
        assert_eq!(1000000000000000000, to_uint64(array_eight!(buffer, 13)));
        assert_eq!(1000000000, to_uint64(array_eight!(buffer, 21)));
        assert_eq!(187649984473770, to_uint64(array_eight!(buffer, 29)));
        assert_eq!(10000000000000000000, to_uint64(array_eight!(buffer, 35)));
        assert_eq!(18446744073709551615, to_uint64(array_eight!(buffer, 43)));
        assert_eq!(9223372036854775807, to_uint64(array_eight!(buffer, 44)));
    }

    #[test]
    #[should_panic]
    fn to_uint64_panic_test() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        to_uint64(array_eight!(buffer, 45));
    }

    #[test]
    fn copy_to_i8_test() {
        let mut data = [0; 1];
        get_bytes(0x12i8).copy_to(&mut data, 0x0);
        assert_eq!([0x12], data);
    }

    #[test]
    fn copy_to_i16_test() {
        let mut data = [0; 2];
        get_bytes(0x1234i16).copy_to(&mut data, 0x0);
        assert_eq!([0x34, 0x12], data);
    }

    #[test]
    fn copy_to_i32_test() {
        let mut data = [0; 4];
        get_bytes(0x12345678i32).copy_to(&mut data, 0x0);
        assert_eq!([0x78, 0x56, 0x34, 0x12], data);
    }

    #[test]
    fn copy_to_i64_test() {
        let mut data = [0; 8];
        get_bytes(0x1234567890123456i64).copy_to(&mut data, 0x0);
        assert_eq!([0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12], data);
    }

    #[test]
    fn copy_to_u8_test() {
        let mut data = [0; 1];
        get_bytes(0x12u8).copy_to(&mut data, 0x0);
        assert_eq!([0x12], data);
    }

    #[test]
    fn copy_to_u16_test() {
        let mut data = [0; 2];
        get_bytes(0x1234u16).copy_to(&mut data, 0x0);
        assert_eq!([0x34, 0x12], data);
    }

    #[test]
    fn copy_to_u32_test() {
        let mut data = [0; 4];
        get_bytes(0x12345678u32).copy_to(&mut data, 0x0);
        assert_eq!([0x78, 0x56, 0x34, 0x12], data);
    }

    #[test]
    fn copy_to_u64_test() {
        let mut data = [0; 8];
        get_bytes(0x1234567890123456u64).copy_to(&mut data, 0x0);
        assert_eq!([0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12], data);
    }
}
