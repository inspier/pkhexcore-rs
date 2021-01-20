use crate::game::enums::game_version::GameVersion;

/// GameVersion analogues used by Colosseum/XD instead of the main-series
/// values.
#[non_exhaustive]
#[repr(u8)]
pub enum GCVersion {
    None = 0,
    FR = 1,
    LG = 2,
    S = 8,
    R = 9,
    E = 10,
    CXD = 11,
}

/// Translates a main-series GameVersion to the corresponding GCVersion value.
/// # Arguments
///
/// * `gba_version` - Version ID while present in the main-series games
pub fn get_cxd_version_id(gba_version: GameVersion) -> GCVersion {
    match gba_version {
        GameVersion::S => GCVersion::S,
        GameVersion::R => GCVersion::R,
        GameVersion::E => GCVersion::E,
        GameVersion::FR => GCVersion::FR,
        GameVersion::LG => GCVersion::LG,
        GameVersion::CXD => GCVersion::CXD,
        _ => GCVersion::None,
    }
}

/// Translates a main-series GCVersion to the corresponding GameVersion value.
/// # Arguments
///
/// * `gc_version` - Version ID while present in the GameCube gamesc
pub fn get_g3_version_id(gc_version: GCVersion) -> GameVersion {
    match gc_version {
        GCVersion::S => GameVersion::S,
        GCVersion::R => GameVersion::R,
        GCVersion::E => GameVersion::E,
        GCVersion::FR => GameVersion::FR,
        GCVersion::LG => GameVersion::LG,
        GCVersion::CXD => GameVersion::CXD,
        _ => GameVersion::Unknown,
    }
}
