use crate::pkm::util::pokecrypto::{decrypt_if_encrypted8, SIZE_8PARTY, SIZE_8STORED};
use crate::util::bitconverter;
use async_std::io;
use async_std::prelude::*;
use async_std::{fs, fs::File};
use log_derive::{logfn, logfn_inputs};
use std::fmt;

/// ## Alignment bytes
/// ```
/// static UNUSED: [u16; 12] = [
///     0x17, 0x1A, 0x1B, 0x23, 0x33, 0x3E, 0x3F, 0xE0, 0xE1, 0xC5, 0x115, 0x11F,
/// ];
/// ```

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
        let mut array = data.clone();
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
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn encryption_constant(mut self: Self, value: u32) -> Self {
        self.set_encryption_constant(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_encryption_constant(self: &Self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x00)
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_encryption_constant(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x00);
    }

    // Sanity
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn sanity(mut self: Self, value: u16) -> Self {
        self.set_sanity(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_sanity(self: &Self) -> u16 {
        bitconverter::to_uint16(&self.data, 0x04)
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_sanity(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x04);
    }

    // Checksum
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn checksum(mut self: Self, value: u16) -> Self {
        self.set_checksum(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_checksum(self: &Self) -> u16 {
        bitconverter::to_uint16(&self.data, 0x06)
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_checksum(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x06);
    }

    // Structure
    // Region A

    // Species
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn species(mut self: Self, value: u32) -> Self {
        self.set_species(value as u16);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_species(self: &Self) -> i32 {
        bitconverter::to_uint16(&self.data, 0x08) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_species(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x08);
    }

    // Held Item
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn held_item(mut self: Self, value: u32) -> Self {
        self.set_held_item(value as u16);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_held_item(self: &Self) -> i32 {
        bitconverter::to_uint16(&self.data, 0x0A) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_held_item(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0A);
    }

    // TID
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn tid(mut self: Self, value: u32) -> Self {
        self.set_tid(value as u16);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_tid(self: &Self) -> i32 {
        bitconverter::to_uint16(&self.data, 0x0C) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_tid(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0C);
    }

    // SID
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn sid(mut self: Self, value: u32) -> Self {
        self.set_sid(value as u16);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_sid(self: &Self) -> i32 {
        bitconverter::to_uint16(&self.data, 0x0E) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_sid(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0E);
    }

    // EXP
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn exp(mut self: Self, value: u32) -> Self {
        self.set_exp(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_exp(self: &Self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x10) as u32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_exp(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x10);
    }

    // Ability
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn ability(mut self: Self, value: u32) -> Self {
        self.set_ability(value as u16);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_ability(self: &Self) -> i32 {
        bitconverter::to_uint16(&self.data, 0x14) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ability(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x14);
    }

    // Ability Number
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn ability_number(mut self: Self, value: i32) -> Self {
        self.set_ability_number(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_ability_number(self: &Self) -> i32 {
        (self.data[0x16] & 7) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ability_number(self: &mut Self, value: i32) {
        self.data[0x16] = ((self.data[0x16] & !7) as i32 | (value & 7)) as u8;
    }

    // Favourite
    // unused, was in LGPE but not in SWSH
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn favourite(mut self: Self, value: bool) -> Self {
        self.set_favourite(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_favourite(self: &Self) -> bool {
        self.data[0x16] & 8 != 0
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_favourite(self: &mut Self, value: bool) {
        self.data[0x16] =
            ((self.data[0x16] & !8) as i32 | ((if value { 1 } else { 0 }) << 3)) as u8;
    }

    // Can Gigantamax
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn can_gigantamax(mut self: Self, value: bool) -> Self {
        self.set_can_gigantamax(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_can_gigantamax(self: &Self) -> bool {
        self.data[0x16] & 16 != 0
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_can_gigantamax(self: &mut Self, value: bool) {
        self.data[0x16] = ((self.data[0x16] & !16) | (if value { 16 } else { 0 })) as u8;
    }

    // 0x17 alignment unused

    // Mark Value
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn mark_value(mut self: Self, value: u16) -> Self {
        self.set_mark_value(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_mark_value(self: &Self) -> i32 {
        bitconverter::to_uint16(&self.data, 0x18) as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_mark_value(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x18);
    }

    // 0x1A alignment unused
    // 0x1B alignment unused

    // PID
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn pid(mut self: Self, value: u32) -> Self {
        self.set_pid(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_pid(self: &Self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x1C)
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_pid(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x1C);
    }

    // Nature
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn nature(mut self: Self, value: i32) -> Self {
        self.set_nature(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_nature(self: &Self) -> i32 {
        self.data[0x20] as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_nature(self: &mut Self, value: i32) {
        self.data[0x20] = value as u8;
    }

    // Stat Nature
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn stat_nature(mut self: Self, value: i32) -> Self {
        self.set_stat_nature(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_stat_nature(self: &Self) -> i32 {
        self.data[0x21] as i32
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_stat_nature(self: &mut Self, value: i32) {
        self.data[0x21] = value as u8;
    }

    // Fateful Encounter
    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn fateful_encounter(mut self: Self, value: bool) -> Self {
        self.set_fateful_encounter(value);
        self
    }

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn get_fateful_encounter(self: &Self) -> bool {
        (self.data[0x22] & 1) == 1
    }

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
    use crate::game::enums::species::Species;

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
            assert_eq!(0xAC731A09, dracovish.get_encryption_constant());
            assert_eq!(0x0, dracovish.get_sanity());
            assert_eq!(0x3B4C, dracovish.get_checksum());
            assert_eq!(882, dracovish.get_species());
            assert_eq!(Species::Dracovish as i32, dracovish.get_species());
            assert_eq!(0, dracovish.get_held_item());
            assert_eq!(30756, dracovish.get_tid());
            assert_eq!(45312, dracovish.get_sid());
            assert_eq!(1250, dracovish.get_exp());
            assert_eq!(11, dracovish.get_ability());
            assert_eq!(1, dracovish.get_ability_number());
            assert_eq!(false, dracovish.get_favourite());
            assert_eq!(false, dracovish.get_can_gigantamax());
            assert_eq!(0, dracovish.get_mark_value());
            assert_eq!(0xC730F59, dracovish.get_pid());
            assert_eq!(16, dracovish.get_nature());
            assert_eq!(16, dracovish.get_stat_nature());
            assert_eq!(false, dracovish.get_fateful_encounter());
            Ok(())
        })
    }
}
