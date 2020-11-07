const GEN7_ZH_OFS: u16 = 0xE800;
const SM_ZHCHARTABLE_SIZE: u16 = 0x30F;
const USUM_CHS_SIZE: u16 = 0x4;

pub fn get_string7(data: &[u8]) -> String {

}

/// Converts a Generation 7 in-game Chinese string to Unicode string.
///
/// # Arguments
///
/// * `input` - In-game Chinese string.
///
/// # Example
///
/// ```
/// use pkhexcore::pkm::strings::string_converter::convert_bin2string_g7_zh;
///
/// assert_eq!(75, get_level(582914, 2).unwrap());
/// ```
///
fn convert_bin2string_g7_zh(input: &str) {
    let mut result = String::with_capacity(24);
    for c in input.chars() { 
    result.push_str();
    }
}

/// Shifts a character from the Chinese character tables
///
/// # Arguments
///
/// * `val` - Input value to shift
///
/// # Example
///
/// ```
/// use pkhexcore::pkm::strings::string_converter::convert_bin2string_g7_zh;
///
/// assert_eq!(75, get_level(582914, 2).unwrap());
/// ```
///
fn get_g7_chinese_char(val: u16) -> u16 {

}
