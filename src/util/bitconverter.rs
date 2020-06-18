use std::convert::TryInto;

/// Returns a 16-bit unsigned integer converted from two bytes at a specified position in a byte array.
///
/// The `to_uint16` function converts the bytes from index start_index to start_index + 1
/// to a `u16` value. The order of bytes in the array must reflect the endianness of the
/// computer system's architecture.
/// # Example
///
/// ```
///# use pkhexcore::util::bitconverter::to_uint16;
/// let buffer: Vec<u8> = vec! [15, 0, 0, 255, 3, 16, 39, 255, 255, 127 ];
/// assert_eq!(65280, to_uint16(&buffer, 2));
/// ```
///
pub fn to_uint16(data: &[u8], start_index: usize) -> u16 {
    let result = u16::from_le_bytes(
        data[start_index..start_index + 2]
            .try_into()
            .expect("Failed to read u16. Invalid buffer provided."),
    );
    result
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified position in a byte array.
///
/// The `to_uint32` function converts the bytes from index start_index to start_index + 3
/// to a `u32` value. The order of bytes in the array must reflect the endianness of the
/// computer system's architecture.
/// # Example
///
/// ```
///# use pkhexcore::util::bitconverter::to_uint32;
/// let buffer: Vec<u8> = vec![15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19,];
/// assert_eq!(261888, to_uint32(&buffer, 6));
/// ```
///
pub fn to_uint32(data: &[u8], start_index: usize) -> u32 {
    let result = u32::from_le_bytes(
        data[start_index..start_index + 4]
            .try_into()
            .expect("Failed to read u32. Invalid buffer provided."),
    );
    result
}

/// Returns a 64-bit unsigned integer converted from eight bytes at a specified position in a byte array.
///
/// The `to_uint64` function converts the bytes from index start_index to start_index + 7
/// to a `u64` value. The order of bytes in the array must reflect the endianness of the
/// computer system's architecture.
/// # Example
///
/// ```
///# use pkhexcore::util::bitconverter::to_uint64;
/// let buffer: Vec<u8> = vec! [255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170,];
/// assert_eq!(255, to_uint64(&buffer, 2));
/// ```
///
pub fn to_uint64(data: &[u8], start_index: usize) -> u64 {
    let result = u64::from_le_bytes(
        data[start_index..start_index + 8]
            .try_into()
            .expect("Failed to read u64. Invalid buffer provided."),
    );
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn to_uint16_test() {
        let buffer = vec![15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(15, to_uint16(&buffer, 0));
        assert_eq!(0, to_uint16(&buffer, 1));
        assert_eq!(1023, to_uint16(&buffer, 3));
        assert_eq!(10000, to_uint16(&buffer, 5));
        assert_eq!(32767, to_uint16(&buffer, 8));
        assert_eq!(65535, to_uint16(&buffer, 7));
        to_uint16(&buffer, 9);
    }

    #[test]
    #[should_panic]
    fn to_uint32_test() {
        let buffer = vec![
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        assert_eq!(15, to_uint32(&buffer, 0));
        assert_eq!(0, to_uint32(&buffer, 1));
        assert_eq!(1048576, to_uint32(&buffer, 3));
        assert_eq!(1023, to_uint32(&buffer, 7));
        assert_eq!(1000000000, to_uint32(&buffer, 10));
        assert_eq!(4294967295, to_uint32(&buffer, 14));
        assert_eq!(2147483647, to_uint32(&buffer, 15));
        to_uint32(&buffer, 16);
    }

    #[test]
    #[should_panic]
    fn to_uint64_test() {
        let buffer = vec![
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        assert_eq!(16777215, to_uint64(&buffer, 0));
        assert_eq!(0, to_uint64(&buffer, 3));
        assert_eq!(4294967296, to_uint64(&buffer, 7));
        assert_eq!(1000000000000000000, to_uint64(&buffer, 13));
        assert_eq!(1000000000, to_uint64(&buffer, 21));
        assert_eq!(187649984473770, to_uint64(&buffer, 29));
        assert_eq!(10000000000000000000, to_uint64(&buffer, 35));
        assert_eq!(18446744073709551615, to_uint64(&buffer, 43));
        assert_eq!(9223372036854775807, to_uint64(&buffer, 44));
        to_uint64(&buffer, 45);
    }
}
