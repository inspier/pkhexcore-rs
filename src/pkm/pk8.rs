use async_std::io;
use async_std::prelude::*;
use async_std::{fs, fs::File};
use log_derive::{logfn, logfn_inputs};
use std::{fmt, fmt::Debug};

use crate::pkm::util::pokecrypto::{decrypt_if_encrypted8, SIZE_8PARTY, SIZE_8STORED};
use crate::util::bitconverter;

/// ## Alignment bytes
/// ```
/// static UNUSED: [u16; 12] = [
///     0x17, 0x1A, 0x1B, 0x23, 0x33, 0x3E, 0x3F, 0xE0, 0xE1, 0xC5, 0x115, 0x11F,
/// ];
/// ```

#[allow(dead_code)]
static FORMAT: u32 = 8;

pub const MAX_IV: i32 = 31;
pub const MAX_EV: i32 = 252;
pub const OT_LENGTH: i32 = 12;
pub const NICK_LENGTH: i32 = 12;

// TODO: PersonalInfo

pub struct PK8 {
    data: [u8; SIZE_8PARTY],
    affixed_ribbon: i8, // 00 would make it show Kalos Champion
}

impl Default for PK8 {
    #[logfn(INFO)]
    fn default() -> Self {
        PK8 {
            data: [0; SIZE_8PARTY],
            affixed_ribbon: -1,
        }
    }
}

impl From<&[u8; 344]> for PK8 {
    fn from(data: &[u8; SIZE_8PARTY]) -> Self {
        let mut array = *data;
        decrypt_if_encrypted8(&mut array);
        PK8 {
            data: array,
            ..Default::default()
        }
    }
}

impl From<&[u8]> for PK8 {
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    fn from(data: &[u8]) -> Self {
        let mut array: [u8; SIZE_8PARTY] = [0; SIZE_8PARTY];
        array.copy_from_slice(&data[0..SIZE_8PARTY]);
        decrypt_if_encrypted8(&mut array);
        PK8 {
            data: array,
            ..Default::default()
        }
    }
}

impl PK8 {
    #[logfn(INFO)]
    pub fn new<T: Into<PK8>>(data: T) -> PK8 {
        data.into()
    }

    #[logfn(INFO)]
    pub fn empty() -> PK8 {
        PK8 {
            ..Default::default()
        }
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub async fn read_from(path: &str) -> io::Result<Self> {
        let mut contents = File::open(path).await?;
        let mut array: [u8; SIZE_8PARTY] = [0; SIZE_8PARTY];
        let _ = contents.read(&mut array).await?;
        decrypt_if_encrypted8(&mut array);
        Ok(PK8 {
            data: array,
            ..Default::default()
        })
    }

    // TODO: Fix when const-generics are stabilized
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub async fn write_to(self: &Self, path: &str) -> io::Result<()> {
        fs::write(path, &self.data[..]).await?;
        Ok(())
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn calculate_checksum(self: &Self) -> u16 {
        let mut chk: u16 = 0;
        for i in (8..SIZE_8STORED).step_by(2) {
            chk = chk.wrapping_add(bitconverter::to_uint16(&self.data, i));
        }
        chk
    }

    // pub fn encrypt(self: &mut Self) -> [u8; SIZE_8PARTY] {
    //     refresh_checksum();
    //     encrypt_array8(&mut self.data.clone());
    // }

    // Encryption Constant
    field!(self; EncryptionConstant; get: u32 => bitconverter::to_uint32(&self.data, 0x00); set: u32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_encryption_constant(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x00);
    }

    // Sanity
    field!(self; Sanity; get: u16 => bitconverter::to_uint16(&self.data, 0x04); set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_sanity(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x04);
    }

    // Checksum
    field!(self; Checksum; get: u16 => bitconverter::to_uint16(&self.data, 0x06); set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_checksum(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x06);
    }

    // Structure
    // Region A

    // Species
    field!(self; Species; get: i32 => bitconverter::to_uint16(&self.data, 0x08) as i32; set: T: Into<u16>);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_species<T: Into<u16> + Debug>(self: &mut Self, value: T) {
        bitconverter::get_bytes(value.into()).copy_to(&mut self.data, 0x08);
    }

    // Held Item
    field!(self; HeldItem; get: i32 => bitconverter::to_uint16(&self.data, 0x0A) as i32; set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_held_item(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0A);
    }

    // TID
    field!(self; tid; get: i32 => bitconverter::to_uint16(&self.data, 0x0C) as i32; set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_tid(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0C);
    }

    // SID
    field!(self; sid; get: i32 => bitconverter::to_uint16(&self.data, 0x0E) as i32; set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_sid(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0E);
    }

    // EXP
    field!(self; exp; get: u32 => bitconverter::to_uint32(&self.data, 0x10); set: u32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_exp(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x10);
    }

    // Ability
    field!(self; Ability; get: i32 => bitconverter::to_uint16(&self.data, 0x14) as i32; set: T: Into<u16>);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ability<T: Into<u16> + Debug>(self: &mut Self, value: T) {
        bitconverter::get_bytes(value.into()).copy_to(&mut self.data, 0x14);
    }

    // Ability Number
    field!(self; AbilityNumber; get: i32 => (self.data[0x16] & 7) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ability_number<T: Into<i32> + Debug>(self: &mut Self, value: T) {
        self.data[0x16] = ((self.data[0x16] & !7) as i32 | (value.into() & 7)) as u8;
    }

    // Favourite
    // unused, was in LGPE but not in SWSH
    field!(self; Favourite; get: bool => self.data[0x16] & 8 != 0; set: bool);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_favourite(self: &mut Self, value: bool) {
        self.data[0x16] =
            ((self.data[0x16] & !8) as i32 | ((if value { 1 } else { 0 }) << 3)) as u8;
    }

    // Can Gigantamax
    field!(self; CanGigantamax; get: bool => self.data[0x16] & 16 != 0; set: bool);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_can_gigantamax(self: &mut Self, value: bool) {
        self.data[0x16] = ((self.data[0x16] & !16) | (if value { 16 } else { 0 })) as u8;
    }

    // 0x17 alignment unused

    // Mark Value
    field!(self; MarkValue; get: i32 => bitconverter::to_uint16(&self.data, 0x18) as i32; set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_mark_value(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x18);
    }

    // 0x1A alignment unused
    // 0x1B alignment unused

    // PID
    field!(self; pid; get: u32 => bitconverter::to_uint32(&self.data, 0x1C); set: u32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_pid(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x1C);
    }

    // Nature
    field!(self; Nature; get: i32 => self.data[0x20] as i32; set: T: Into<i32>);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_nature<T: Into<i32> + Debug>(self: &mut Self, value: T) {
        self.data[0x20] = value.into() as u8;
    }

    // Stat Nature
    field!(self; StatNature; get: i32 => self.data[0x21] as i32; set: T: Into<i32>);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_stat_nature<T: Into<i32> + Debug>(self: &mut Self, value: T) {
        self.data[0x21] = value.into() as u8;
    }

    // Fateful Encounter
    field!(self; FatefulEncounter; get: bool => (self.data[0x22] & 1) == 1; set: bool);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_fateful_encounter(self: &mut Self, value: bool) {
        self.data[0x22] = ((self.data[0x22] & !0x01) | (if value { 1 } else { 0 })) as u8;
    }
}

impl PartialEq for PK8 {
    fn eq(&self, other: &Self) -> bool {
        (self.data.iter().eq(other.data.iter())) && (self.affixed_ribbon == other.affixed_ribbon)
    }
}

impl Eq for PK8 {}

impl fmt::Debug for PK8 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::enums::{ability::Ability, nature::Nature, species::Species};

    #[test]
    fn pk8_from_array_test() -> std::io::Result<()> {
        async_std::task::block_on(async {
            let orbeetle_d = PK8::read_from("src/pkm/util/tests/data/Orbeetle.pk8").await?;
            let orbeetle_e = PK8::read_from("src/pkm/util/tests/data/Orbeetle.ek8").await?;
            let dracovish = PK8::read_from("src/pkm/util/tests/data/Dracovish.pk8").await?;

            assert_eq!(true, orbeetle_d == orbeetle_e);
            assert_eq!(false, dracovish == orbeetle_d);
            Ok(())
        })
    }

    #[test]
    fn pk8_from_vec_test() -> io::Result<()> {
        async_std::task::block_on(async {
            let orbeetle_e = PK8::read_from("src/pkm/util/tests/data/Orbeetle.ek8").await?;
            let orbeetle_d = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.pk8").to_vec());

            assert_eq!(true, orbeetle_e == orbeetle_d);
            Ok(())
        })
    }

    #[test]
    fn pk8_calc_checksum_test() -> io::Result<()> {
        async_std::task::block_on(async {
            let orbeetle = PK8::read_from("src/pkm/util/tests/data/Orbeetle.pk8").await?;
            let dracovish = PK8::read_from("src/pkm/util/tests/data/Dracovish.pk8").await?;
            assert_eq!(0x4E8E, orbeetle.calculate_checksum());
            assert_eq!(0x3B4C, dracovish.calculate_checksum());
            Ok(())
        })
    }

    #[test]
    fn pk8_get_test() -> io::Result<()> {
        async_std::task::block_on(async {
            let dracovish = PK8::read_from("src/pkm/util/tests/data/Dracovish.pk8").await?;
            assert_eq!(0xAC731A09, get!(dracovish, EncryptionConstant));
            assert_eq!(0x0, get!(dracovish, Sanity));
            assert_eq!(0x3B4C, get!(dracovish, Checksum));
            assert_eq!(882, get!(dracovish, Species));
            assert_eq!(i32::from(Species::Dracovish), get!(dracovish, Species));
            assert_eq!(0, get!(dracovish, HeldItem));
            assert_eq!(30756, get!(dracovish, tid));
            assert_eq!(45312, get!(dracovish, sid));
            assert_eq!(1250, get!(dracovish, Exp));
            assert_eq!(11, get!(dracovish, Ability));
            assert_eq!(i32::from(Ability::WaterAbsorb), get!(dracovish, Ability));
            assert_eq!(1, get!(dracovish, AbilityNumber));
            assert_eq!(false, get!(dracovish, Favourite));
            assert_eq!(false, get!(dracovish, CanGigantamax));
            assert_eq!(0, get!(dracovish, MarkValue));
            assert_eq!(0xC730F59, get!(dracovish, pid));
            assert_eq!(16, get!(dracovish, Nature));
            assert_eq!(i32::from(Nature::Mild), get!(dracovish, Nature));
            assert_eq!(16, get!(dracovish, StatNature));
            assert_eq!(i32::from(Nature::Mild), get!(dracovish, StatNature));
            assert_eq!(false, get!(dracovish, FatefulEncounter));
            Ok(())
        })
    }

    #[test]
    fn pk8_set_test() -> io::Result<()> {
        async_std::task::block_on(async {
            let mut dracovish = PK8::read_from("src/pkm/util/tests/data/Dracovish.pk8").await?;
            assert_eq!(0xAC731A09, get!(dracovish, EncryptionConstant));
            set!(dracovish, EncryptionConstant, 0xDEADBEEF);
            assert_eq!(0xDEADBEEF, get!(dracovish, EncryptionConstant));
            Ok(())
        })
    }
}
