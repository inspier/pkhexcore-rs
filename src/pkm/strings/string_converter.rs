use beef::Cow;
use core::char::{decode_utf16, REPLACEMENT_CHARACTER};

const GEN7_ZH_OFS: u16 = 0xE800;
const SM_ZHCHARTABLE_SIZE: u16 = 0x30F;
const USUM_CHS_SIZE: u16 = 0x4;

fn sanitize_string(data: &[u16]) -> String {
    decode_utf16(data.iter().take_while(|&&x| x != 0).copied())
        .map(|c| match c {
            Ok('’') => '\'',       // Farfetch'd
            Ok('\u{E08F}') => '♀', // ♀ (gen6+)
            Ok('\u{E08E}') => '♂', // ♂ (gen6+)
            Ok('\u{246E}') => '♀', // ♀ (gen5)
            Ok('\u{246D}') => '♂', // ♂ (gen5)
            Ok(c) => c,
            Err(_) => REPLACEMENT_CHARACTER,
        })
        .collect::<String>()
}

pub fn get_string7(data: &[u16]) -> String {
    // TODO Language sanitizing.
    sanitize_string(&data)
}

/*
fn convert_bin2string_g7_zh(input: &str) {
    let mut result = String::with_capacity(24);
    for c in input.chars() {
        result.push_str();
    }
}

fn get_g7_chinese_char(val: u16) -> u16 {}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_string7_test() {
        assert_eq!(
            "Farfetch\'d",
            get_string7(&[0x46, 0x61, 0x72, 0x66, 0x65, 0x74, 0x63, 0x68, 0x27, 0x64])
        );
        assert_eq!(
            "Nidoran♀",
            get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0xe08f])
        );
        assert_eq!(
            "Nidoran♂",
            get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0xe08e])
        );
        assert_eq!(
            "Nidoran♀",
            get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0x246e])
        );
        assert_eq!(
            "Nidoran♂",
            get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0x246d])
        );
        assert_eq!(
            "Fletchinder",
            get_string7(&[0x46, 0x6c, 0x65, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x64, 0x65, 0x72])
        );
        assert_eq!("Ho-Oh", get_string7(&[0x48, 0x6f, 0x2d, 0x4f, 0x68]));
    }
}
