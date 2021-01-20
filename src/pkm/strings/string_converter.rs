use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{
    char::{decode_utf16, REPLACEMENT_CHARACTER},
    iter,
};

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

// TODO Refactor to be more idiomatic.
fn unsanitize_string(s: &str, generation: u32) -> String {
    let mut s = s.to_string();
    if generation >= 6 {
        s = s.replace("\'", "’"); // Farfetch'd
    }
    if generation <= 5 {
        s = s.replace("\u{2640}", "\u{246E}") // ♀
        .replace("\u{2642}", "\u{246D}"); // ♂
        return s.into();
    }
    let full_width = s
        .chars()
        .filter(|c| !['\u{2640}', '\u{2642}'].contains(c))
        .any(|x| ![0, 0xE].contains(&(x as u16 >> 12)));
    if full_width {
        return s.into();
    }
    s = s.replace("\u{2640}", "\u{E08F}") // ♀
       .replace("\u{2642}", "\u{E08E}"); // ♂
    s.into()
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
