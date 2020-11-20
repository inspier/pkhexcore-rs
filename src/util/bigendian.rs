use core::{convert::TryInto, mem};
use log_derive::{logfn, logfn_inputs};

/// Returns a 16-bit signed integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_int16` function converts the bytes from index start_index to
/// start_index + 1 to a `i16` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bigendian::to_int16;
/// let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
/// assert_eq!(255, to_int16(&buffer, 2));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn to_int16(data: &[u8], start_index: usize) -> i16 {
    i16::from_be_bytes(
        data[start_index..][..mem::size_of::<i16>()]
            .try_into()
            .expect("Failed to read i16. Invalid buffer provided."),
    )
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
/// use pkhexcore::util::bigendian::to_int32;
/// let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
/// assert_eq!(-2146424848, to_int32(&buffer, 3));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn to_int32(data: &[u8], start_index: usize) -> i32 {
    i32::from_be_bytes(
        data[start_index..][..mem::size_of::<i32>()]
            .try_into()
            .expect("Failed to read i32. Invalid buffer provided."),
    )
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
/// use pkhexcore::util::bigendian::to_int64;
/// let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
/// assert_eq!(140806877927665, to_int64(&buffer, 1));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn to_int64(data: &[u8], start_index: usize) -> i64 {
    i64::from_be_bytes(
        data[start_index..][..mem::size_of::<i64>()]
            .try_into()
            .expect("Failed to read i64. Invalid buffer provided."),
    )
}

/// Returns a 16-bit unsigned integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_uint16` function converts the bytes from index start_index to
/// start_index + 1 to a `u16` value.
/// # Example
///
/// ```
/// use pkhexcore::util::bigendian::to_uint16;
/// let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127 ];
/// assert_eq!(255, to_uint16(&buffer, 2));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn to_uint16(data: &[u8], start_index: usize) -> u16 {
    u16::from_be_bytes(
        data[start_index..][..mem::size_of::<u16>()]
            .try_into()
            .expect("Failed to read u16. Invalid buffer provided."),
    )
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified
/// position in a byte array.
///
/// The `to_uint32` function converts the bytes from index start_index to
/// start_index + 3 to a `u32` value.
/// # Example
///
/// ```
/// use pkhexcore::util::bigendian::to_uint32;
/// let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19,];
/// assert_eq!(16712448, to_uint32(&buffer, 6));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn to_uint32(data: &[u8], start_index: usize) -> u32 {
    u32::from_be_bytes(
        data[start_index..][..mem::size_of::<u32>()]
            .try_into()
            .expect("Failed to read u32. Invalid buffer provided."),
    )
}

/// Returns a 64-bit unsigned integer converted from eight bytes at a specified
/// position in a byte array.
///
/// The `to_uint64` function converts the bytes from index start_index to
/// start_index + 7 to a `u64` value.
/// # Example
///
/// ```
/// use pkhexcore::util::bigendian::to_uint64;
/// let buffer = [255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170,];
/// assert_eq!(18374686479671623680, to_uint64(&buffer, 2));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn to_uint64(data: &[u8], start_index: usize) -> u64 {
    u64::from_be_bytes(
        data[start_index..][..mem::size_of::<u64>()]
            .try_into()
            .expect("Failed to read u64. Invalid buffer provided."),
    )
}

/// Returns a 32-bit signed integer converted from bytes in a Binary Coded
/// Decimal format byte array.
///
/// # Example
///
/// ```
/// use pkhexcore::util::bigendian::bcd_to_int32;
/// let buffer = [32, 16];
/// assert_eq!(2010, bcd_to_int32(&buffer, 0, 2));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn bcd_to_int32(input: &[u8], offset: usize, length: u32) -> i32 {
    input
        .iter()
        .take(offset + length as usize)
        .skip(offset)
        .fold(0i32, |mut result, p| {
            result *= 100;
            result += (10 * (p >> 4)) as i32;
            result += (p & 0xf) as i32;
            result
        })
}

/// Returns the specified 32-bit signed integer value as an array of Binary
/// Coded Decimal format bytes.
///
/// # Example
///
/// ```
/// use pkhexcore::util::bigendian::int32_to_bcd;
/// assert_eq!([32, 16], int32_to_bcd::<2>(2010));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn int32_to_bcd<const N: usize>(mut input: i32) -> [u8; N] {
    let mut result: [u8; N] = [0; N];
    for i in 0..N {
        let p = input % 100;
        input /= 100;
        result[N - i - 1] = ((p / 10) << 4 | (p % 10)) as u8;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_int16_test_be() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        assert_eq!(3840, to_int16(&buffer, 0));
        assert_eq!(0, to_int16(&buffer, 1));
        assert_eq!(128, to_int16(&buffer, 2));
        assert_eq!(4135, to_int16(&buffer, 4));
        assert_eq!(-3880, to_int16(&buffer, 6));
        assert_eq!(-3585, to_int16(&buffer, 8));
        assert_eq!(-129, to_int16(&buffer, 9));
    }

    #[test]
    #[should_panic]
    fn to_int16_panic_test() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        to_uint16(&buffer, 11);
    }

    #[test]
    fn to_int32_test_be() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        assert_eq!(251658240, to_int32(&buffer, 0));
        assert_eq!(16, to_int32(&buffer, 2));
        assert_eq!(1048831, to_int32(&buffer, 4));
        assert_eq!(268500739, to_int32(&buffer, 5));
        assert_eq!(50331850, to_int32(&buffer, 8));
        assert_eq!(-895861761, to_int32(&buffer, 11));
        assert_eq!(-1707343873, to_int32(&buffer, 12));
    }

    #[test]
    #[should_panic]
    fn to_int32_panic_test() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        to_int32(&buffer, 16);
    }

    #[test]
    fn to_int64_test_be() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        assert_eq!(15311545525338111, to_int64(&buffer, 0));
        assert_eq!(-1099511627776, to_int64(&buffer, 5));
        assert_eq!(0, to_int64(&buffer, 8));
        assert_eq!(128, to_int64(&buffer, 9));
        assert_eq!(57027523489300480, to_int64(&buffer, 17));
        assert_eq!(16777216, to_int64(&buffer, 21));
        assert_eq!(4294967295, to_int64(&buffer, 26));
        assert_eq!(72058693549555711, to_int64(&buffer, 34));
        assert_eq!(6220972285274488831, to_int64(&buffer, 45));
        assert_eq!(-6148914691236560896, to_int64(&buffer, 53));
        assert_eq!(110671437422605, to_int64(&buffer, 59));
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
        to_int64(&buffer, 68);
    }

    #[test]
    fn to_uint16_test_be() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(3840, to_uint16(&buffer, 0));
        assert_eq!(0, to_uint16(&buffer, 1));
        assert_eq!(65283, to_uint16(&buffer, 3));
        assert_eq!(4135, to_uint16(&buffer, 5));
        assert_eq!(65407, to_uint16(&buffer, 8));
        assert_eq!(65535, to_uint16(&buffer, 7));
    }

    #[test]
    #[should_panic]
    fn to_uint16_panic_test() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        to_uint16(&buffer, 9);
    }

    #[test]
    fn to_uint32_test_be() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        assert_eq!(251658240, to_uint32(&buffer, 0));
        assert_eq!(0, to_uint32(&buffer, 1));
        assert_eq!(4096, to_uint32(&buffer, 3));
        assert_eq!(4278386688, to_uint32(&buffer, 7));
        assert_eq!(13277755, to_uint32(&buffer, 10));
        assert_eq!(4294967295, to_uint32(&buffer, 14));
        assert_eq!(4294967167, to_uint32(&buffer, 15));
    }

    #[test]
    #[should_panic]
    fn to_uint32_panic_test() {
        let buffer = [
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        to_uint32(&buffer, 16);
    }

    #[test]
    fn to_uint64_test_be() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        assert_eq!(18446742974197923840, to_uint64(&buffer, 0));
        assert_eq!(0, to_uint64(&buffer, 3));
        assert_eq!(16777216, to_uint64(&buffer, 7));
        assert_eq!(110671437422605, to_uint64(&buffer, 13));
        assert_eq!(57027523489300480, to_uint64(&buffer, 21));
        assert_eq!(12297829382472990720, to_uint64(&buffer, 29));
        assert_eq!(255675177617290, to_uint64(&buffer, 35));
        assert_eq!(18446744073709551615, to_uint64(&buffer, 43));
        assert_eq!(18446744073709551487, to_uint64(&buffer, 44));
    }

    #[test]
    #[should_panic]
    fn to_uint64_panic_test() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        to_uint64(&buffer, 45);
    }
}
