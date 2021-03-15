pub const LINKTRADE_4: i32 = 2002;
pub const LINKTRADE_5: i32 = 30003;
pub const LINKTRADE_6: i32 = 30002;

pub const DAYCARE_4: i32 = 2000;
pub const DAYCARE_5: i32 = 60002;

pub const LINKTRADE_2_NPC: i32 = 126;
pub const LINKTRADE_3_NPC: i32 = 254;
pub const LINKTRADE_4_NPC: i32 = 2001;
pub const LINKTRADE_5_NPC: i32 = 30002;
pub const LINKTRADE_6_NPC: i32 = 30001;

pub const POKEWALKER_4: i32 = 233;
pub const RANGER_4: i32 = 3001;
pub const FARAWAY_4: i32 = 3002;

/// Goldenrod City in
/// [GameVersion::C][`crate::game::enums::game_version::GameVersion::C`]
pub const HATCH_LOCATION_C: i32 = 16;

/// Route 117 in
/// [GameVersion::RSE][`crate::game::enums::game_version::GameVersion::RSE`]
pub const HATCH_LOCATION_RSE: i32 = 32;

/// Route 17 in
/// [GameVersion::FRLG][`crate::game::enums::game_version::GameVersion::FRLG`]
pub const HATCH_LOCATION_FRLG: i32 = 117;

/// Solaceon Town in
/// [GameVersion::DPPT][`crate::game::enums::game_version::GameVersion::DPPT`]
pub const HATCH_LOCATION_DPPT: i32 = 4;

/// Route 34 in
/// [GameVersion::HGSS][`crate::game::enums::game_version::GameVersion::HGSS`]
pub const HATCH_LOCATION_HGSS: i32 = 182;

/// Skyarrow Bridge in
/// [GameVersion::Gen5][`crate::game::enums::game_version::GameVersion::Gen5`]
pub const HATCH_LOCATION_5: i32 = 64;

/// Route 7 in
/// [GameVersion::XY][`crate::game::enums::game_version::GameVersion::XY`]
pub const HATCH_LOCATION_6XY: i32 = 38;

/// Battle Resort in
/// [GameVersion::ORAS][`crate::game::enums::game_version::GameVersion::ORAS`]
pub const HATCH_LOCATION_6AO: i32 = 318;

/// Paniola Ranch in
/// [GameVersion::Gen7][`crate::game::enums::game_version::GameVersion::Gen7`]
pub const HATCH_LOCATION_7: i32 = 78;

/// Route 5 in
/// [GameVersion::SWSH][`crate::game::enums::game_version::GameVersion::SWSH`]
pub const HATCH_LOCATION_8: i32 = 40;

/// Generation 1 -> Generation 7 Transfer Location (Kanto)
pub const TRANSFER_1: i32 = 30013;

/// Generation 2 -> Generation 7 Transfer Location (Johto)
pub const TRANSFER_2: i32 = 30017;

/// Generation 3 -> Generation 4 Transfer Location (Pal Park)
pub const TRANSFER_3: i32 = 0x37;

/// Generation 4 -> Generation 5 Transfer Location (Poké Transporter)
pub const TRANSFER_4: i32 = 30001;

/// Generation 4 -> Generation 5 Transfer Location (Crown Celebi - Event not
/// activated in Gen 5)
pub const TRANSFER_4_CELEBI_UNUSED: i32 = 30010;

/// Generation 4 -> Generation 5 Transfer Location (Crown Celebi - Event
/// activated in Gen 5)
pub const TRANSFER_4_CELEBI_USED: i32 = 30011;

/// Generation 4 -> Generation 5 Transfer Location (Crown Beast - Event not
/// activated in Gen 5)
pub const TRANSFER_4_CROWN_UNUSED: i32 = 30012;

/// Generation 4 -> Generation 5 Transfer Location (Crown Beast - Event
/// activated in Gen 5)
pub const TRANSFER_4_CROWN_USED: i32 = 30013;

/// Generation 6 Gift from Pokémon Link
pub const LINKGIFT_6: i32 = 30011;

/// Generation 7 Transfer from GO to Pokémon LGP/E's GO Park
pub const GO_7: i32 = 50;

/// Generation 8 Transfer from GO to Pokémon HOME
pub const GO_8: i32 = 30012;

/// Generation 8 Gift from Pokémon HOME
pub const HOME_8: i32 = 30018;

pub const BUG_CATCHING_CONTEST_4: i32 = 207;

pub fn traded_egg_location_npc(generation: u32) -> i32 {
    match generation {
        1 => LINKTRADE_2_NPC,
        2 => LINKTRADE_2_NPC,
        3 => LINKTRADE_3_NPC,
        4 => LINKTRADE_4,
        5 => LINKTRADE_5,
        _ => LINKTRADE_6_NPC,
    }
}

pub fn traded_egg_location(generation: u32) -> i32 {
    match generation {
        4 => LINKTRADE_4,
        5 => LINKTRADE_5,
        _ => LINKTRADE_6,
    }
}

pub fn is_pt_hgss_location(location: i32) -> bool { 111 < location && location < 2000 }
pub fn is_pt_hgss_location_egg(location: i32) -> bool { 2010 < location && location < 3000 }
pub fn is_event_location_5(location: i32) -> bool { 40000 < location && location < 50000 }

const SAFARI_LOCATION_RSE: i32 = 57;
const SAFARI_LOCATION_FRLG: i32 = 136;
const SAFARI_LOCATION_HGSS: i32 = 202;
const MARSH_LOCATION_DPPT: i32 = 52;

pub fn is_safari_zone_location_3(location: i32) -> bool {
    [SAFARI_LOCATION_RSE, SAFARI_LOCATION_FRLG].contains(&location)
}

pub fn is_safari_zone_location_4(location: i32) -> bool {
    [MARSH_LOCATION_DPPT, SAFARI_LOCATION_HGSS].contains(&location)
}
