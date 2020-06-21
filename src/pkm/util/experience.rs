use crate::pkm::util::exptable::{EXP_TABLE, EXP_TABLE_WIDTH};

/// Gets the current level of a species.
///
/// The `get_level` function takes in the species' current EXP and the EXP
/// growth rate, and returns the level for the given boundaries. This function
/// will return the invalid level `0` if an out of bounds growth is provided (a
/// value larger than `pkhexcore::pkm::util::exptable::EXP_TABLE_WIDTH`).
///
/// # Arguments
///
/// * `exp` - Experience points
/// * `growth` - Experience growth rate
/// # Example
///
/// ```
/// use pkhexcore::pkm::util::experience::get_level;
///
/// assert_eq!(75, get_level(582914, 2));
/// ```
pub fn get_level(exp: usize, growth: usize) -> usize {
    if growth > EXP_TABLE_WIDTH {
        return 0;
    } else if exp >= EXP_TABLE[99][growth] {
        return 100;
    }

    let mut lvl = 1;
    while exp >= EXP_TABLE[lvl][growth] {
        lvl += 1;
    }
    return lvl;
}

/// Gets the minimum Experience points for the specified level.
///
/// The `get_exp` function takes in the species' current level and growth rate,
/// and returns the base EXP for the given boundaries. This function will also
/// return `0` if an out of bounds growth is provided (a value larger than
/// `pkhexcore::pkm::util::exptable::EXP_TABLE_WIDTH`).
///
/// # Arguments
///
/// * `level` - The Pokemon's current level
/// * `growth` - Experience growth rate
/// # Example
///
/// ```
/// use pkhexcore::pkm::util::experience::get_exp;
///
/// assert_eq!(800000, get_exp(100, 4));
/// ```
pub fn get_exp(level: usize, growth: usize) -> usize {
    if level <= 1 || growth > EXP_TABLE_WIDTH {
        return 0;
    } else if level > 100 {
        return 100;
    }
    return EXP_TABLE[level - 1][growth];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_level() {
        assert_eq!(100, get_level(900_000, 4));
        assert_eq!(31, get_level(24_000, 4));
        assert_eq!(75, get_level(582914, 2));
        assert_eq!(10, get_level(1000, 0));
        assert_eq!(1, get_level(0, 0));
        assert_eq!(98, get_level(590298, 1));
        assert_eq!(100, get_level(589289198, 3));
        assert_eq!(3, get_level(100, 1));
        assert_eq!(34, get_level(50000, 5));
    }

    #[test]
    fn test_get_level_out_of_bounds_growth() {
        assert_eq!(0, get_level(100, 10));
    }

    #[test]
    fn test_get_exp() {
        assert_eq!(800000, get_exp(100, 4));
        assert_eq!(21012, get_exp(24, 1));
        assert_eq!(250047, get_exp(63, 0));
        assert_eq!(0, get_exp(1, 4));
        assert_eq!(1122660, get_exp(90, 2));
        assert_eq!(30486, get_exp(29, 5));
        assert_eq!(600000, get_exp(100, 1));
        assert_eq!(419, get_exp(9, 3));
    }

    #[test]
    fn test_get_exp_out_of_bounds() {
        assert_eq!(100, get_exp(101, 1));
        assert_eq!(0, get_exp(100, 10));
    }
}
