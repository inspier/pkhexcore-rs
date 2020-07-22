#[allow(non_camel_case_types)]
#[repr(u8)]
/// Game Language IDs
pub enum LanguageGC {
    /// Undefined Language ID, usually indicative of a value not being set.
    // Gen5 Japanese In-game Trades happen to not have their Language value set, and express Language=0.
    Hacked = 0,

    /// Japanese (日本語)
    Japanese = 1,

    /// English (US/UK/AU)
    English = 2,

    /// German (Deutsch)
    German = 3,

    /// French (Français)
    French = 4,

    /// Italian (Italiano)
    Italian = 5,

    /// Spanish (Español)
    Spanish = 6,

    /// Unused Language ID
    // Was reserved for Korean in Gen3 but never utilized.
    UNUSED_6 = 7,
}
