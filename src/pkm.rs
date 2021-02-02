use alloc::{string::String, vec::Vec};
use deku::DekuUpdate;
use crate::game::enums::game_version::GameVersion;

pub trait PKM {
    type RawVariant;

    fn get_string(data: &[u16]) -> String;

    fn set_string<S: AsRef<str>>(&self, data: S, max_length: usize) -> Vec<u16>;

    fn e(version: u8) -> bool { version == GameVersion::E as u8 }
    fn frlg(version: u8) -> bool { [GameVersion::FR as u8, GameVersion::LG as u8].contains(&version) }
    fn pt(version: u8) -> bool { version == GameVersion::Pt as u8 }
    fn hgss(version: u8) -> bool { [GameVersion::HG as u8, GameVersion::SS as u8].contains(&version) }
    fn bw(version: u8) -> bool { [GameVersion::B as u8, GameVersion::W as u8].contains(&version) }
    fn b2w2(version: u8) -> bool { [GameVersion::B2 as u8, GameVersion::W2 as u8].contains(&version) }
    fn xy(version: u8) -> bool { [GameVersion::X as u8, GameVersion::Y as u8].contains(&version) }
    fn ao(version: u8) -> bool { [GameVersion::AS as u8, GameVersion::OR as u8].contains(&version) }
    fn sm(version: u8) -> bool { [GameVersion::SN as u8, GameVersion::MN as u8].contains(&version) }
    fn usum(version: u8) -> bool { [GameVersion::US as u8, GameVersion::UM as u8].contains(&version) }
    fn go(version: u8) -> bool { version == GameVersion::GO as u8 }
    fn vc1(version: u8) -> bool { version >= GameVersion::RD as u8 && version <= GameVersion::YW as u8 }
    fn vc2(version: u8) -> bool { version >= GameVersion::GD as u8 && version <= GameVersion::C as u8 }
    fn lgpe(version: u8) -> bool { [GameVersion::GP as u8, GameVersion::GE as u8].contains(&version) }
    fn swsh(version: u8) -> bool { [GameVersion::SW as u8, GameVersion::SH as u8].contains(&version) }

//    fn is_gen8(version: u8, met_location: u16) -> bool { version >= 44 && version <= 45 || GO_HOME }

    /*
    fn get_generation(version: u8, met_location: u16) -> {

    }
    */

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
