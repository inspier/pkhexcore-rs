use alloc::format;
use deku::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u8", ctx = "_endian: deku::ctx::Endian")]
/// Contiguous series Game Language IDs
pub enum LanguageID {
    /// Undefined Language ID, usually indicative of a value not being set.
    // Gen5 Japanese In-game Trades happen to not have their Language value set, and express Language=0.
    Hacked = 0,

    /// Japanese (日本語)
    Japanese = 1,

    /// English (US/UK/AU)
    English = 2,

    /// French (Français)
    French = 3,

    /// Italian (Italiano)
    Italian = 4,

    /// German (Deutsch)
    German = 5,

    /// Unused Language ID
    // Was reserved for Korean in Gen3 but never utilized.
    UNUSED_6 = 6,

    /// Spanish (Español)
    Spanish = 7,

    /// Korean (한국어)
    Korean = 8,

    /// Chinese Simplified (简体中文)
    ChineseS = 9,

    /// Chinese Traditional (繁體中文)
    ChineseT = 10,
}
