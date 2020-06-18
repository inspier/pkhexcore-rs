use structure::*;

/// A module that converts base data types to an array of bytes, and an array of bytes to base data types.

/// Returns a 16-bit unsigned integer converted from two bytes at a specified position in a byte array.
/// # Example
///
/// ```
///# use pkhexcore::util::bitconverter::to_uint16;
/// let buffer: Vec<u8> = vec! [15, 0, 0, 255, 3, 16, 39, 255, 255, 127 ];
/// assert_eq!(65280, to_uint16(&buffer, 2));
/// ```
///
pub fn to_uint16(data: &[u8], start_index: usize) -> u16 {
    let unpacker = structure!("<H");
    let (result,) = unpacker
        .unpack(&data[start_index..start_index + 2])
        .expect("Failed to read uint16.");
    return result;
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified position in a byte array.
/// # Example
///
/// ```
///# use pkhexcore::util::bitconverter::to_uint32;
/// let buffer: Vec<u8> = vec![15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19,];
/// assert_eq!(261888, to_uint32(&buffer, 6));
/// ```
///
pub fn to_uint32(data: &[u8], start_index: usize) -> u32 {
    let unpacker = structure!("<I");
    let (result,) = unpacker
        .unpack(&data[start_index..start_index + 4])
        .expect("Failed to read uint32.");
    return result;
}

/// Returns a 64-bit unsigned integer converted from eight bytes at a specified position in a byte array.
/// # Example
///
/// ```
///# use pkhexcore::util::bitconverter::to_uint64;
/// let buffer: Vec<u8> = vec! [255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170,];
/// assert_eq!(255, to_uint64(&buffer, 2));
/// ```
///
pub fn to_uint64(data: &[u8], start_index: usize) -> u64 {
    let unpacker = structure!("<Q");
    let (result,) = unpacker
        .unpack(&data[start_index..start_index + 8])
        .expect("Failed to read uint64.");
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_uint16_test() {
        let buffer = vec![15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(15, to_uint16(&buffer, 0));
        assert_eq!(0, to_uint16(&buffer, 1));
        assert_eq!(1023, to_uint16(&buffer, 3));
        assert_eq!(10000, to_uint16(&buffer, 5));
        assert_eq!(32767, to_uint16(&buffer, 8));
        assert_eq!(65535, to_uint16(&buffer, 7));
    }

    #[test]
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
    }

    #[test]
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
    }

    #[test]
    fn rand_test() {
        let buffer: Vec<u8> = vec![15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(65280, to_uint16(&buffer, 2));
    }
}
