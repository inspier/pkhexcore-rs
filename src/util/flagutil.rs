use crate::game::enums::flag::Flag;

/// Gets a specified bitflag in a byte array.
///
/// # Arguments
///
/// * `array` - A byte array
/// * `offset` - Index into byte array.
/// * `bit_index` - Bit index in offset element.
///
/// # Example
///
/// ```
/// use pkhexcore::{game::enums::flag::Flag, util::flagutil::*};
///
/// let buffer = [15, 0, 16, 255];
/// assert_eq!(Flag::Set, get_flag(&buffer, 0, 0));
/// ```
pub fn get_flag(array: &[u8], offset: usize, mut bit_index: u32) -> Flag {
    bit_index &= 7; // ensure bit access is 0-7
    match (array[offset] >> bit_index & 1) != 0 {
        true => Flag::Set,
        false => Flag::Unset,
    }
}

/// Sets a specified bitflag in a byte array.
///
/// # Arguments
///
/// * `array` - A byte array
/// * `offset` - Index into byte array.
/// * `bit_index` - Bit index in offset element.
/// * `value` - Whether the flag should be set or not.
///
/// # Example
///
/// ```
/// use pkhexcore::{game::enums::flag::Flag, util::flagutil::*};
///
/// let mut buffer = [15, 0, 16, 255];
/// set_flag(&mut buffer, 0, 0, Flag::Unset);
/// assert_eq!(Flag::Unset, get_flag(&buffer, 0, 0));
/// ```
pub fn set_flag(array: &mut [u8], offset: usize, mut bit_index: u32, value: Flag) {
    bit_index &= 7; // ensure bit access is 0-7
    array[offset] &= !(1 << bit_index) as u8;
    array[offset] |= match value {
        Flag::Set => 1,
        Flag::Unset => 0,
    } << bit_index as u8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_flag_test() {
        let buffer = [15, 0, 16, 255];
        for i in 0..4 {
            assert_eq!(Flag::Set, get_flag(&buffer, 0, i));
            assert_eq!(Flag::Unset, get_flag(&buffer, 1, i));
            assert_eq!(Flag::Unset, get_flag(&buffer, 2, i));
        }
        assert_eq!(Flag::Set, get_flag(&buffer, 2, 4));
        for i in 0..8 {
            assert_eq!(Flag::Set, get_flag(&buffer, 3, i));
        }
    }

    #[test]
    fn set_flag_test() {
        let mut buffer = [15, 0, 16, 255];
        set_flag(&mut buffer, 0, 0, Flag::Unset);
        assert_eq!(Flag::Unset, get_flag(&buffer, 0, 0));
        for i in 0..8 {
            set_flag(&mut buffer, 3, i, Flag::Unset);
            assert_eq!(Flag::Unset, get_flag(&buffer, 3, i));
        }
    }
}
