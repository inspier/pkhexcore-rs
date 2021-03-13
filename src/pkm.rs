use crate::{game::enums::game_version::GameVersion, legality::tables::location};
use alloc::{string::String, vec::Vec};
use deku::DekuUpdate;

pub trait PKM {
    type RawVariant;

    fn get_string(data: &[u16]) -> String;

    fn set_string<S: AsRef<str>>(&self, data: S, max_length: usize) -> Vec<u16>;

    fn is_e(version: GameVersion) -> bool { version == GameVersion::E }
    fn is_frlg(version: GameVersion) -> bool {
        [GameVersion::FR, GameVersion::LG].contains(&version)
    }
    fn is_pt(version: GameVersion) -> bool { version == GameVersion::PT }
    fn is_hgss(version: GameVersion) -> bool {
        [GameVersion::HG, GameVersion::SS].contains(&version)
    }
    fn is_bw(version: GameVersion) -> bool { [GameVersion::B, GameVersion::W].contains(&version) }
    fn is_b2w2(version: GameVersion) -> bool {
        [GameVersion::B2, GameVersion::W2].contains(&version)
    }
    fn is_xy(version: GameVersion) -> bool { [GameVersion::X, GameVersion::Y].contains(&version) }
    fn is_ao(version: GameVersion) -> bool { [GameVersion::AS, GameVersion::OR].contains(&version) }
    fn is_sm(version: GameVersion) -> bool { [GameVersion::SN, GameVersion::MN].contains(&version) }
    fn is_usum(version: GameVersion) -> bool {
        [GameVersion::US, GameVersion::UM].contains(&version)
    }
    fn is_go(version: GameVersion) -> bool { version == GameVersion::GO }
    fn is_vc1(version: GameVersion) -> bool {
        version >= GameVersion::RD && version <= GameVersion::YW
    }
    fn is_vc2(version: GameVersion) -> bool {
        version >= GameVersion::GD && version <= GameVersion::C
    }
    fn is_lgpe(version: GameVersion) -> bool {
        [GameVersion::GP, GameVersion::GE].contains(&version)
    }
    fn is_swsh(version: GameVersion) -> bool {
        [GameVersion::SW, GameVersion::SH].contains(&version)
    }

    fn is_pt_hgss(version: GameVersion) -> bool { Self::is_pt(version) || Self::is_hgss(version) }
    fn is_go_lgpe(version: GameVersion, met_location: u16) -> bool {
        Self::is_go(version) && met_location == location::GO7 as u16
    }
    fn is_go_home(version: GameVersion, met_location: u16) -> bool {
        Self::is_go(version) && met_location == location::GO8 as u16
    }
    fn is_vc(version: GameVersion) -> bool { Self::is_vc1(version) || Self::is_vc2(version) }
    fn is_gg(version: GameVersion, met_location: u16) -> bool {
        Self::is_lgpe(version) || Self::is_go_lgpe(version, met_location)
    }
    fn is_gen8(version: GameVersion, met_location: u16) -> bool {
        version >= GameVersion::SW && version <= GameVersion::SH
            || Self::is_go_home(version, met_location)
    }
    fn is_gen7(version: GameVersion, met_location: u16) -> bool {
        version >= GameVersion::SN && version <= GameVersion::UM
            || Self::is_gg(version, met_location)
    }
    fn is_gen6(version: GameVersion) -> bool {
        version >= GameVersion::X && version < GameVersion::SN
    }
    fn is_gen5(version: GameVersion) -> bool {
        version >= GameVersion::W && version <= GameVersion::B2
    }
    fn is_gen4(version: GameVersion) -> bool {
        version as u8 >= 7 && version as u8 <= 12 && version as u8 != 9
    }
    fn is_gen3(version: GameVersion) -> bool {
        version >= GameVersion::S && version <= GameVersion::LG || version == GameVersion::CXD
    }
    fn is_gen2(version: GameVersion) -> bool {
        version == GameVersion::GSC // Fixed value set by the Gen2 PKM classes
    }
    fn is_gen1(version: GameVersion) -> bool {
        version == GameVersion::RBY // Fixed value set by the Gen2 PKM classes
    }
    fn is_genu(generation: i32) -> bool {
        generation <= 0 // Fixed value set by the Gen2 PKM classes
    }

    fn get_generation(version: GameVersion, met_location: u16) -> i32 {
        match true {
            _ if Self::is_gen8(version, met_location) => 8,
            _ if Self::is_gen7(version, met_location) => 7,
            _ if Self::is_gen6(version) => 6,
            _ if Self::is_gen5(version) => 5,
            _ if Self::is_gen4(version) => 4,
            _ if Self::is_gen3(version) => 3,
            _ if Self::is_gen2(version) | Self::is_vc2(version) => 2,
            _ if Self::is_gen1(version) | Self::is_vc1(version) => 1,
            _ => -1,
        }
    }

    fn refresh_checksum(&mut self)
    where
        Self: DekuUpdate,
    {
        // Note: Double update needed to make sure changes to checksum propagate.
        let _ = self.update();
        let _ = self.update();
    }

    fn build(&mut self) -> Self::RawVariant;
}

/// Module containing utilities to manipulate Pokemon data files
#[macro_use]
pub mod util;

/// String utilities
pub mod strings;

/// Generation 8
pub mod pk8;
