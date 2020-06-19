use std::convert::TryInto;

/// Returns a 16-bit unsigned integer converted from two bytes at a specified position in a byte array.
///
/// The `to_uint16` function converts the bytes from index start_index to start_index + 1
/// to a `u16` value.
/// # Example
///
/// ```
///# use pkhexcore::util::bigendian::to_uint16;
/// let buffer: Vec<u8> = vec![15, 0, 0, 255, 3, 16, 39, 255, 255, 127 ];
/// assert_eq!(255, to_uint16(&buffer, 2));
/// ```
///
pub fn to_uint16(data: &[u8], start_index: usize) -> u16 {
    let result = u16::from_be_bytes(
        data[start_index..start_index + 2]
            .try_into()
            .expect("Failed to read u16. Invalid buffer provided."),
    );
    result
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified position in a byte array.
///
/// The `to_uint32` function converts the bytes from index start_index to start_index + 3
/// to a `u32` value.
/// # Example
///
/// ```
///# use pkhexcore::util::bigendian::to_uint32;
/// let buffer: Vec<u8> = vec![15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19,];
/// assert_eq!(16712448, to_uint32(&buffer, 6));
/// ```
///
pub fn to_uint32(data: &[u8], start_index: usize) -> u32 {
    let result = u32::from_be_bytes(
        data[start_index..start_index + 4]
            .try_into()
            .expect("Failed to read u32. Invalid buffer provided."),
    );
    result
}

/// Returns a 64-bit unsigned integer converted from eight bytes at a specified position in a byte array.
///
/// The `to_uint64` function converts the bytes from index start_index to start_index + 7
/// to a `u64` value.
/// # Example
///
/// ```
///# use pkhexcore::util::bigendian::to_uint64;
/// let buffer: Vec<u8> = vec![255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170,];
/// assert_eq!(18374686479671623680, to_uint64(&buffer, 2));
/// ```
///
pub fn to_uint64(data: &[u8], start_index: usize) -> u64 {
    let result = u64::from_be_bytes(
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
    fn to_uint16_test_be() {
        let buffer = vec![15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
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
        let buffer = vec![15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        to_uint16(&buffer, 9);
    }

    #[test]
    fn to_uint32_test_be() {
        let buffer = vec![
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
        let buffer = vec![
            15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127,
        ];
        to_uint32(&buffer, 16);
    }

    #[test]
    fn to_uint64_test_be() {
        let buffer = vec![
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
        let buffer = vec![
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        to_uint64(&buffer, 45);
    }
}
