#![allow(dead_code)]
/// Returns a 32-bit signed integer converted from bytes in a Binary Coded
/// Decimal format byte array.
pub(crate) fn bcd_to_int32(input: &[u8], offset: usize, length: u32) -> i32 {
    input.iter().take(offset + length as usize).skip(offset).fold(0i32, |mut result, p| {
        result *= 100;
        result += (10 * (p >> 4)) as i32;
        result += (p & 0xf) as i32;
        result
    })
}

/// Returns the specified 32-bit signed integer value as an array of Binary
/// Coded Decimal format bytes.
pub(crate) fn int32_to_bcd<const N: usize>(mut input: i32) -> [u8; N] {
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
    fn bcd_to_int32_test() {
        let buffer = [32, 16];
        assert_eq!(2010, bcd_to_int32(&buffer, 0, 2));
    }

    #[test]
    fn int32_to_bcd_test() {
        assert_eq!([32, 16], int32_to_bcd::<2>(2010));
    }
}
