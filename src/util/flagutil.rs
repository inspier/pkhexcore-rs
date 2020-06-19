pub fn get_flag(array: &[u8], offset: usize, mut bit_index: u32) -> bool {
    bit_index &= 7; // ensure bit access is 0-7
    (array[offset] >> bit_index & 1) != 0
}

pub fn set_flag(array: &mut [u8], offset: usize, mut bit_index: u32, value: bool) {
    bit_index &= 7; // ensure bit access is 0-7
    array[offset] &= !(1 << bit_index) as u8;
    array[offset] |= ((if value { 1 } else { 0 }) << bit_index) as u8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_flag_test() {
        let buffer = vec![15, 0, 16, 255];
        for i in 0..4 {
            assert_eq!(true, get_flag(&buffer, 0, i));
            assert_eq!(false, get_flag(&buffer, 1, i));
            assert_eq!(false, get_flag(&buffer, 2, i));
        }
        assert_eq!(true, get_flag(&buffer, 2, 4));
        for i in 0..8 {
            assert_eq!(true, get_flag(&buffer, 3, i));
        }
    }
}
