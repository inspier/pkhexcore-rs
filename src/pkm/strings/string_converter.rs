use alloc::{string::String, vec, vec::Vec};
use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use core::iter;

use crate::game::enums::language_id::LanguageID;

const GEN7_ZH_OFS: u16 = 0xE800;
const SM_ZHCHARTABLE_SIZE: u16 = 0x30F;
const USUM_CHS_SIZE: u16 = 0x4;

fn sanitize_glyph(c: char) -> char {
    match c {
        '’' => '\'',       // Farfetch'd
        '\u{E08F}' => '♀', // ♀ (gen6+)
        '\u{E08E}' => '♂', // ♂ (gen6+)
        '\u{246E}' => '♀', // ♀ (gen5)
        '\u{246D}' => '♂', // ♂ (gen5)
        _ => c,
    }
}

fn sanitize_string(data: &[u16]) -> String {
    decode_utf16(data.iter().take_while(|&&x| x != 0).copied())
        .map(|r| r.map_or(REPLACEMENT_CHARACTER, sanitize_glyph))
        .collect::<String>()
}

pub fn get_string7(data: &[u16]) -> String {
    // TODO Language sanitizing.
    sanitize_string(&data)
}

pub fn set_string7b(
    data: &str,
    max_length: usize,
    language: LanguageID,
    pad_to: usize,
    pad_with: u16,
    is_chinese: bool,
) -> Vec<u16> {
    let mut result = data.encode_utf16().take(max_length).collect::<Vec<u16>>();
    result.extend(vec![0; max_length - result.len()]);
    result.extend(iter::once(0));
    let delta = pad_to.checked_sub(max_length + 1).unwrap_or(0);
    result.extend(vec![pad_with; delta]);
    result
}

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
