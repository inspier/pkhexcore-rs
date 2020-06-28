use crate::pkm::util::exptable::*;

/// Gets the current level of a species.
///
/// The `get_level` function takes in the species' current EXP and the EXP
/// growth rate, and returns the level for the given boundaries. This function
/// will return the invalid level `0` if an out of bounds growth is provided (a
/// value larger than `pkhexcore::pkm::util::exptable::EXP_TABLE_WIDTH`).
///
/// # Arguments
///
/// * `experience` - Experience points
/// * `growth_rate` - Experience growth rate
///
/// # Example
///
/// ```
/// use pkhexcore::pkm::util::experience::get_level;
///
/// assert_eq!(75, get_level(582914, 2).unwrap());
/// ```
pub fn get_level(experience: u32, growth_rate: u32) -> Option<u32> {
    if growth_rate > EXP_TABLE_WIDTH {
        return None;
    } else if experience >= EXP_TABLE[99][growth_rate as usize] {
        return Some(100);
    }

    let mut lvl = 1;
    while experience >= EXP_TABLE[lvl as usize][growth_rate as usize] {
        lvl += 1;
    }
    Some(lvl)
}

/// Gets the minimum Experience points for the specified level.
///
/// The `get_exp` function takes in the species' current level and growth rate,
/// and returns the base EXP for the given boundaries. This function will also
/// return `0` if an out of bounds growth is provided (a value larger than
/// `exptable::EXP_TABLE_WIDTH`).
///
/// # Arguments
///
/// * `current_level` - The Pokemon's current level
/// * `growth_rate` - Experience growth rate
///
/// # Example
///
/// ```
/// use pkhexcore::pkm::util::experience::get_exp;
///
/// assert_eq!(800000, get_exp(100, 4).unwrap());
/// ```
pub fn get_exp(mut current_level: u32, growth_rate: u32) -> Option<u32> {
    if current_level <= 1 || growth_rate > EXP_TABLE_WIDTH {
        return None;
    } else if current_level > 100 {
        current_level = 100;
    }
    Some(EXP_TABLE[(current_level - 1) as usize][growth_rate as usize])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_level() {
        assert_eq!(100, get_level(900_000, 4).unwrap());
        assert_eq!(31, get_level(24_000, 4).unwrap());
        assert_eq!(75, get_level(582914, 2).unwrap());
        assert_eq!(10, get_level(1000, 0).unwrap());
        assert_eq!(1, get_level(0, 0).unwrap());
        assert_eq!(98, get_level(590298, 1).unwrap());
        assert_eq!(100, get_level(589289198, 3).unwrap());
        assert_eq!(3, get_level(100, 1).unwrap());
        assert_eq!(34, get_level(50000, 5).unwrap());
    }

    #[test]
    fn test_get_level_out_of_bounds_growth() {
        assert_eq!(true, get_level(100, 10).is_none());
    }

    #[test]
    fn test_get_exp() {
        assert_eq!(800000, get_exp(100, 4).unwrap());
        assert_eq!(21012, get_exp(24, 1).unwrap());
        assert_eq!(250047, get_exp(63, 0).unwrap());
        assert_eq!(true, get_exp(1, 4).is_none());
        assert_eq!(1122660, get_exp(90, 2).unwrap());
        assert_eq!(30486, get_exp(29, 5).unwrap());
        assert_eq!(600000, get_exp(100, 1).unwrap());
        assert_eq!(419, get_exp(9, 3).unwrap());
        assert_eq!(1059860, get_exp(100, 3).unwrap());
    }

    #[test]
    fn test_get_exp_out_of_bounds() {
        assert_eq!(true, get_exp(100, 10).is_none());
        assert_eq!(true, get_exp(0, 11).is_none());
    }
}
