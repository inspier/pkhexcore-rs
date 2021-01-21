use alloc::{string::String, vec::Vec};
use core::{
    char::{decode_utf16, REPLACEMENT_CHARACTER},
    iter,
};

use crate::{game::enums::language_id::LanguageID, pkm::strings::resources::char_zh::*};

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

fn unsanitize_glyph(c: char, generation: u32, full_width: bool) -> char {
    match c {
        '\'' if generation >= 6 => '’',
        '\u{2640}' if generation <= 5 => '\u{246E}',
        '\u{2642}' if generation <= 5 => '\u{246D}',
        '\u{2640}' if generation >= 6 && !full_width => '\u{E08F}',
        '\u{2642}' if generation >= 6 && !full_width => '\u{E08E}',
        _ => c,
    }
}

fn remap_chinese_glyphs(c: char) -> char {
    match c as u16 {
        v if v < GEN7_ZH_OFS || v >= GEN7_ZH_OFS + GEN7_ZH.len() as u16 => c,
        _ => GEN7_ZH[(c as u16 - GEN7_ZH_OFS) as usize],
    }
}

fn sanitize_string(data: &[u16]) -> String {
    decode_utf16(data.iter().take_while(|&&x| x != 0).copied())
        .map(|r| r.map_or(REPLACEMENT_CHARACTER, sanitize_glyph))
        .map(|c| remap_chinese_glyphs(c))
        .collect::<String>()
}

fn unsanitize_string<S: AsRef<str>>(s: S, generation: u32) -> String {
    let s = s.as_ref();
    let full_width = s
        .chars()
        .filter(|c| !['\u{2640}', '\u{2642}'].contains(c))
        .any(|x| ![0, 0xE].contains(&(x as u16 >> 12)));

    s.chars().map(|c| unsanitize_glyph(c, generation, full_width)).collect()
}

pub fn get_string7(data: &[u16]) -> String {
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
    // TODO Language unsanitizing.
    let data = unsanitize_string(data, 7);
    let mut result = data.encode_utf16().take(max_length).collect::<Vec<u16>>();
    // Pad to max_length if necessary
    result.extend([0].iter().cycle().take(max_length - result.len()));
    // Null terminator
    result.extend(iter::once(0));
    // Pad remaining if requested.
    let delta = pad_to.checked_sub(max_length + 1).unwrap_or(0);
    result.extend([pad_with].iter().cycle().take(delta));
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_string7_test() {
        assert_eq!(
            "Farfetch\'d",
            get_string7(&[0x46, 0x61, 0x72, 0x66, 0x65, 0x74, 0x63, 0x68, 0x2019, 0x64])
        );
        assert_eq!("Nidoran♀", get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0xe08f]));
        assert_eq!("Nidoran♂", get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0xe08e]));
        assert_eq!("Nidoran♀", get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0x246e]));
        assert_eq!("Nidoran♂", get_string7(&[0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0x246d]));
        assert_eq!(
            "Fletchinder",
            get_string7(&[0x46, 0x6c, 0x65, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x64, 0x65, 0x72])
        );
        assert_eq!("Ho-Oh", get_string7(&[0x48, 0x6f, 0x2d, 0x4f, 0x68]));
    }

    #[test]
    fn set_string7b_test() {
        let set_string = |s| {
            set_string7b(s, 12, LanguageID::English, 0, 0, false)
                .into_iter()
                .take_while(|&x| x != 0)
                .collect::<Vec<u16>>()
        };
        assert_eq!(
            set_string("Farfetch\'d"),
            [0x46, 0x61, 0x72, 0x66, 0x65, 0x74, 0x63, 0x68, 0x2019, 0x64]
        );
        assert_eq!(set_string("Nidoran♀"), [0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0xe08f]);
        assert_eq!(set_string("Nidoran♂"), [0x4e, 0x69, 0x64, 0x6f, 0x72, 0x61, 0x6e, 0xe08e]);
        assert_eq!(
            set_string("Fletchinder"),
            [0x46, 0x6c, 0x65, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x64, 0x65, 0x72]
        );
        assert_eq!(set_string("Ho-Oh"), [0x48, 0x6f, 0x2d, 0x4f, 0x68]);
    }

    #[test]
    fn unsanitize_string_test() {
        assert_eq!(unsanitize_string("Nidoran♀", 5), "Nidoran\u{246E}");
        assert_eq!(unsanitize_string("Nidoran♂", 5), "Nidoran\u{246D}");
        assert_eq!(unsanitize_string("Nidoran♀", 7), "Nidoran\u{E08F}");
        assert_eq!(unsanitize_string("Nidoran♂", 7), "Nidoran\u{E08E}");
    }
}
