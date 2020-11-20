use log_derive::{logfn, logfn_inputs};

/// The depth of the EXP_TABLE, useful to stay within bounds
const EXP_TABLE_DEPTH: u32 = 100;

/// The width of the EXP_TABLE, useful to stay within bounds
const EXP_TABLE_WIDTH: u32 = 6;

/// Experience required for next level [0,99]
#[allow(clippy::zero_prefixed_literal)]
const EXP_TABLE: [[u32; EXP_TABLE_WIDTH as usize]; EXP_TABLE_DEPTH as usize] = [
    [0000000, 0000000, 0000000, 0000000, 0000000, 0000000],
    [0000008, 0000015, 0000004, 0000009, 0000006, 0000010],
    [0000027, 0000052, 0000013, 0000057, 0000021, 0000033],
    [0000064, 0000122, 0000032, 0000096, 0000051, 0000080],
    [0000125, 0000237, 0000065, 0000135, 0000100, 0000156],
    [0000216, 0000406, 0000112, 0000179, 0000172, 0000270],
    [0000343, 0000637, 0000178, 0000236, 0000274, 0000428],
    [0000512, 0000942, 0000276, 0000314, 0000409, 0000640],
    [0000729, 0001326, 0000393, 0000419, 0000583, 0000911],
    [0001000, 0001800, 0000540, 0000560, 0000800, 0001250],
    [0001331, 0002369, 0000745, 0000742, 0001064, 0001663],
    [0001728, 0003041, 0000967, 0000973, 0001382, 0002160],
    [0002197, 0003822, 0001230, 0001261, 0001757, 0002746],
    [0002744, 0004719, 0001591, 0001612, 0002195, 0003430],
    [0003375, 0005737, 0001957, 0002035, 0002700, 0004218],
    [0004096, 0006881, 0002457, 0002535, 0003276, 0005120],
    [0004913, 0008155, 0003046, 0003120, 0003930, 0006141],
    [0005832, 0009564, 0003732, 0003798, 0004665, 0007290],
    [0006859, 0011111, 0004526, 0004575, 0005487, 0008573],
    [0008000, 0012800, 0005440, 0005460, 0006400, 0010000],
    [0009261, 0014632, 0006482, 0006458, 0007408, 0011576],
    [0010648, 0016610, 0007666, 0007577, 0008518, 0013310],
    [0012167, 0018737, 0009003, 0008825, 0009733, 0015208],
    [0013824, 0021012, 0010506, 0010208, 0011059, 0017280],
    [0015625, 0023437, 0012187, 0011735, 0012500, 0019531],
    [0017576, 0026012, 0014060, 0013411, 0014060, 0021970],
    [0019683, 0028737, 0016140, 0015244, 0015746, 0024603],
    [0021952, 0031610, 0018439, 0017242, 0017561, 0027440],
    [0024389, 0034632, 0020974, 0019411, 0019511, 0030486],
    [0027000, 0037800, 0023760, 0021760, 0021600, 0033750],
    [0029791, 0041111, 0026811, 0024294, 0023832, 0037238],
    [0032768, 0044564, 0030146, 0027021, 0026214, 0040960],
    [0035937, 0048155, 0033780, 0029949, 0028749, 0044921],
    [0039304, 0051881, 0037731, 0033084, 0031443, 0049130],
    [0042875, 0055737, 0042017, 0036435, 0034300, 0053593],
    [0046656, 0059719, 0046656, 0040007, 0037324, 0058320],
    [0050653, 0063822, 0050653, 0043808, 0040522, 0063316],
    [0054872, 0068041, 0055969, 0047846, 0043897, 0068590],
    [0059319, 0072369, 0060505, 0052127, 0047455, 0074148],
    [0064000, 0076800, 0066560, 0056660, 0051200, 0080000],
    [0068921, 0081326, 0071677, 0061450, 0055136, 0086151],
    [0074088, 0085942, 0078533, 0066505, 0059270, 0092610],
    [0079507, 0090637, 0084277, 0071833, 0063605, 0099383],
    [0085184, 0095406, 0091998, 0077440, 0068147, 0106480],
    [0091125, 0100237, 0098415, 0083335, 0072900, 0113906],
    [0097336, 0105122, 0107069, 0089523, 0077868, 0121670],
    [0103823, 0110052, 0114205, 0096012, 0083058, 0129778],
    [0110592, 0115015, 0123863, 0102810, 0088473, 0138240],
    [0117649, 0120001, 0131766, 0109923, 0094119, 0147061],
    [0125000, 0125000, 0142500, 0117360, 0100000, 0156250],
    [0132651, 0131324, 0151222, 0125126, 0106120, 0165813],
    [0140608, 0137795, 0163105, 0133229, 0112486, 0175760],
    [0148877, 0144410, 0172697, 0141677, 0119101, 0186096],
    [0157464, 0151165, 0185807, 0150476, 0125971, 0196830],
    [0166375, 0158056, 0196322, 0159635, 0133100, 0207968],
    [0175616, 0165079, 0210739, 0169159, 0140492, 0219520],
    [0185193, 0172229, 0222231, 0179056, 0148154, 0231491],
    [0195112, 0179503, 0238036, 0189334, 0156089, 0243890],
    [0205379, 0186894, 0250562, 0199999, 0164303, 0256723],
    [0216000, 0194400, 0267840, 0211060, 0172800, 0270000],
    [0226981, 0202013, 0281456, 0222522, 0181584, 0283726],
    [0238328, 0209728, 0300293, 0234393, 0190662, 0297910],
    [0250047, 0217540, 0315059, 0246681, 0200037, 0312558],
    [0262144, 0225443, 0335544, 0259392, 0209715, 0327680],
    [0274625, 0233431, 0351520, 0272535, 0219700, 0343281],
    [0287496, 0241496, 0373744, 0286115, 0229996, 0359370],
    [0300763, 0249633, 0390991, 0300140, 0240610, 0375953],
    [0314432, 0257834, 0415050, 0314618, 0251545, 0393040],
    [0328509, 0267406, 0433631, 0329555, 0262807, 0410636],
    [0343000, 0276458, 0459620, 0344960, 0274400, 0428750],
    [0357911, 0286328, 0479600, 0360838, 0286328, 0447388],
    [0373248, 0296358, 0507617, 0377197, 0298598, 0466560],
    [0389017, 0305767, 0529063, 0394045, 0311213, 0486271],
    [0405224, 0316074, 0559209, 0411388, 0324179, 0506530],
    [0421875, 0326531, 0582187, 0429235, 0337500, 0527343],
    [0438976, 0336255, 0614566, 0447591, 0351180, 0548720],
    [0456533, 0346965, 0639146, 0466464, 0365226, 0570666],
    [0474552, 0357812, 0673863, 0485862, 0379641, 0593190],
    [0493039, 0367807, 0700115, 0505791, 0394431, 0616298],
    [0512000, 0378880, 0737280, 0526260, 0409600, 0640000],
    [0531441, 0390077, 0765275, 0547274, 0425152, 0664301],
    [0551368, 0400293, 0804997, 0568841, 0441094, 0689210],
    [0571787, 0411686, 0834809, 0590969, 0457429, 0714733],
    [0592704, 0423190, 0877201, 0613664, 0474163, 0740880],
    [0614125, 0433572, 0908905, 0636935, 0491300, 0767656],
    [0636056, 0445239, 0954084, 0660787, 0508844, 0795070],
    [0658503, 0457001, 0987754, 0685228, 0526802, 0823128],
    [0681472, 0467489, 1035837, 0710266, 0545177, 0851840],
    [0704969, 0479378, 1071552, 0735907, 0563975, 0881211],
    [0729000, 0491346, 1122660, 0762160, 0583200, 0911250],
    [0753571, 0501878, 1160499, 0789030, 0602856, 0941963],
    [0778688, 0513934, 1214753, 0816525, 0622950, 0973360],
    [0804357, 0526049, 1254796, 0844653, 0643485, 1005446],
    [0830584, 0536557, 1312322, 0873420, 0664467, 1038230],
    [0857375, 0548720, 1354652, 0902835, 0685900, 1071718],
    [0884736, 0560922, 1415577, 0932903, 0707788, 1105920],
    [0912673, 0571333, 1460276, 0963632, 0730138, 1140841],
    [0941192, 0583539, 1524731, 0995030, 0752953, 1176490],
    [0970299, 0591882, 1571884, 1027103, 0776239, 1212873],
    [1000000, 0600000, 1640000, 1059860, 0800000, 1250000],
];

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
/// assert_eq!(Some(75), get_level(582914, 2));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
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
/// assert_eq!(Some(800000), get_exp(100, 4));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn get_exp(current_level: u32, growth_rate: u32) -> Option<u32> {
    if current_level <= 1 || growth_rate > EXP_TABLE_WIDTH || current_level > EXP_TABLE_DEPTH {
        return None;
    }
    Some(EXP_TABLE[(current_level - 1) as usize][growth_rate as usize])
}

/// Gets the nature value for "PK1" / "PK2" entries based on the experience
///
/// # Arguments
///
/// * `experience` - The Pokemon's current experience
///
/// # Example
///
/// ```
/// use pkhexcore::pkm::util::experience::get_nature_vc;
///
/// assert_eq!(0, get_nature_vc(25));
/// assert_eq!(10, get_nature_vc(10));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn get_nature_vc(exp: u32) -> u32 {
    exp % 25
}

/// Gets the amount of EXP to be earned until the next level-up occurs.
///
/// # Arguments
///
/// * `current_level` - The Pokemon's current level
/// * `growth` - Experience growth rate
///
/// # Example
///
/// ```
/// use pkhexcore::pkm::util::experience::get_nature_vc;
///
/// assert_eq!(0, get_nature_vc(25));
/// assert_eq!(10, get_nature_vc(10));
/// ```
///
#[logfn(INFO)]
#[logfn_inputs(Debug)]
pub fn get_exp_to_level_up(current_level: u32, growth_rate: u32) -> Option<u32> {
    if current_level == 0 || current_level > EXP_TABLE_DEPTH || growth_rate > EXP_TABLE_WIDTH {
        return None;
    }

    if current_level == 100 {
        return Some(0);
    }

    let current_exp = EXP_TABLE[(current_level - 1) as usize][growth_rate as usize];
    let next_exp = EXP_TABLE[current_level as usize][growth_rate as usize];
    Some(next_exp - current_exp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_level() {
        assert_eq!(Some(100), get_level(900_000, 4));
        assert_eq!(Some(31), get_level(24_000, 4));
        assert_eq!(Some(75), get_level(582914, 2));
        assert_eq!(Some(10), get_level(1000, 0));
        assert_eq!(Some(1), get_level(0, 0));
        assert_eq!(Some(98), get_level(590298, 1));
        assert_eq!(Some(100), get_level(589289198, 3));
        assert_eq!(Some(3), get_level(100, 1));
        assert_eq!(Some(34), get_level(50000, 5));
    }

    #[test]
    fn test_get_level_out_of_bounds_growth() {
        assert_eq!(true, get_level(100, 10).is_none());
    }

    #[test]
    fn test_get_exp() {
        assert_eq!(Some(800000), get_exp(100, 4));
        assert_eq!(Some(21012), get_exp(24, 1));
        assert_eq!(Some(250047), get_exp(63, 0));
        assert_eq!(true, get_exp(1, 4).is_none());
        assert_eq!(Some(1122660), get_exp(90, 2));
        assert_eq!(Some(30486), get_exp(29, 5));
        assert_eq!(Some(600000), get_exp(100, 1));
        assert_eq!(Some(419), get_exp(9, 3));
        assert_eq!(Some(1059860), get_exp(100, 3));
    }

    #[test]
    fn test_get_exp_out_of_bounds() {
        assert_eq!(true, get_exp(100, 10).is_none());
        assert_eq!(true, get_exp(0, 11).is_none());
        assert_eq!(true, get_exp(110, 11).is_none());
    }

    #[test]
    fn test_get_nature_vc() {
        assert_eq!(0, get_nature_vc(25));
        assert_eq!(0, get_nature_vc(0));
        assert_eq!(1, get_nature_vc(1));
        assert_eq!(10, get_nature_vc(10));
        assert_eq!(5, get_nature_vc(5));
        assert_eq!(2, get_nature_vc(2));
    }

    #[test]
    fn test_get_exp_to_level_up() {
        assert_eq!(Some(0), get_exp_to_level_up(100, 4));
        assert_eq!(Some(2425), get_exp_to_level_up(24, 1));
        assert_eq!(Some(12097), get_exp_to_level_up(63, 0));
        assert_eq!(Some(6), get_exp_to_level_up(1, 4));
        assert_eq!(Some(37839), get_exp_to_level_up(90, 2));
        assert_eq!(Some(3264), get_exp_to_level_up(29, 5));
        assert_eq!(Some(0), get_exp_to_level_up(100, 1));
        assert_eq!(Some(141), get_exp_to_level_up(9, 3));
    }

    #[test]
    fn test_get_exp_to_level_up_out_of_bounds() {
        assert_eq!(true, get_exp_to_level_up(100, 10).is_none());
        assert_eq!(true, get_exp_to_level_up(0, 11).is_none());
        assert_eq!(true, get_exp_to_level_up(110, 11).is_none());
    }
}
