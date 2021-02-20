use alloc::{borrow::ToOwned, string::String, vec::Vec};
use conquer_once::spin::Lazy;
use core::{
    char::{decode_utf16, REPLACEMENT_CHARACTER}, iter
};
use hashbrown::HashMap;

use crate::{game::enums::language_id::LanguageID, pkm::strings::resources::char_zh::*};

static G7_CHS: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    GEN7_ZH
        .iter()
        .enumerate()
        .filter(|(index, _)| is_g7chs_char(*index))
        .map(|(index, &value)| (value, index))
        .collect()
});

static G7_CHT: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    GEN7_ZH
        .iter()
        .enumerate()
        .filter(|(index, _)| !is_g7chs_char(*index))
        .map(|(index, &value)| (value, index))
        .collect()
});

const SM_ZH_CHARTABLE_SIZE: u16 = 0x30F;
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
        '\'' if generation >= 6 => '’',            // Farfetch'd
        '\u{2640}' if generation <= 5 => '\u{246E}', // ♀ (gen5)
        '\u{2642}' if generation <= 5 => '\u{246D}', // ♂ (gen5)
        '\u{2640}' if generation >= 6 && !full_width => '\u{E08F}', // ♀ (gen6+)
        '\u{2642}' if generation >= 6 && !full_width => '\u{E08E}', // ♂ (gen6+)
        _ => c,
    }
}

fn remap_chinese_glyphs(c: char) -> char {
    match c as u16 {
        v if v < GEN7_ZH_OFS || v >= GEN7_ZH_OFS + GEN7_ZH.len() as u16 => c,
        _ => GEN7_ZH[(c as u16 - GEN7_ZH_OFS) as usize],
    }
}

fn convert_string_g7_zh<S: AsRef<str>>(input: S, language: LanguageID) -> String {
    let mut input_chars = input.as_ref().chars();
    // CHS and CHT have the same display name.
    let is_traditional = input_chars.any(|c| G7_CHT.contains_key(&c) && !G7_CHS.contains_key(&c))
        || (language == LanguageID::ChineseT
            && !input_chars.any(|c| G7_CHT.contains_key(&c) ^ G7_CHS.contains_key(&c)));
    let table = [&G7_CHS, &G7_CHT][is_traditional as usize];

    input_chars
        .map(|c| table.get(&c).map_or(c, |&index| char::from(index as u8 + GEN7_ZH_OFS as u8)))
        .collect::<String>()
}

fn is_g7chs_char(index: usize) -> bool {
    index < SM_ZH_CHARTABLE_SIZE as usize
        || (SM_ZH_CHARTABLE_SIZE as usize * 2 <= index
            && index < (SM_ZH_CHARTABLE_SIZE as usize * 2) + USUM_CHS_SIZE as usize)
}

fn sanitize_string(data: &[u16]) -> String {
    decode_utf16(data.iter().take_while(|&&x| x != 0).copied())
        .map(|r| r.map_or(REPLACEMENT_CHARACTER, sanitize_glyph))
        .map(remap_chinese_glyphs)
        .collect::<String>()
}

fn unsanitize_string<S: AsRef<str>>(input: S, generation: u32) -> String {
    let input = input.as_ref();
    let full_width = input
        .chars()
        .filter(|c| !['\u{2640}', '\u{2642}'].contains(c))
        .any(|x| ![0, 0xE].contains(&(x as u16 >> 12)));

    input.chars().map(|c| unsanitize_glyph(c, generation, full_width)).collect()
}

pub fn get_string7(data: &[u16]) -> String { sanitize_string(&data) }

pub fn set_string7b(
    data: &str,
    max_length: usize,
    language: LanguageID,
    pad_to: usize,
    pad_with: u16,
    is_chinese: bool,
) -> Vec<u16> {
    let mut unsanitized = data.to_owned();
    if is_chinese {
        unsanitized = convert_string_g7_zh(data, language);
    }
    unsanitized = unsanitize_string(unsanitized, 7);
    let mut result = unsanitized.encode_utf16().take(max_length).collect::<Vec<u16>>();
    // Pad to max_length if necessary
    result.extend([0].iter().cycle().take(max_length - result.len()));
    // Null terminator
    result.extend(iter::once(0));
    // Pad remaining if requested.
    let delta = pad_to.saturating_sub(max_length + 1);
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
        assert_eq!("多龍梅西亞", get_string7(&[0x591A, 0x9F8D, 0x6885, 0x897F, 0x4E9E])) // Dreepy
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
        assert_eq!(set_string("多龍梅西亞"), [0x591A, 0x9F8D, 0x6885, 0x897F, 0x4E9E]) // Dreepy
    }

    #[test]
    fn unsanitize_string_test() {
        assert_eq!(unsanitize_string("Nidoran♀", 5), "Nidoran\u{246E}");
        assert_eq!(unsanitize_string("Nidoran♂", 5), "Nidoran\u{246D}");
        assert_eq!(unsanitize_string("Nidoran♀", 7), "Nidoran\u{E08F}");
        assert_eq!(unsanitize_string("Nidoran♂", 7), "Nidoran\u{E08E}");
    }
}
