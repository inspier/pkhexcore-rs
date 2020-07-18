use crate::pkm::util::pokecrypto::{
    decrypt_if_encrypted8, encrypt_array8, SIZE_8PARTY, SIZE_8STORED,
};
use crate::pkm::util::pokedex;
use crate::util::bitconverter;

// Alignment bytes
static UNUSED: [u16; 12] = [
    0x17, 0x1A, 0x1B, 0x23, 0x33, 0x3E, 0x3F, 0xE0, 0xE1, 0xC5, 0x115, 0x11F,
];

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
    fn default() -> Self {
        PK8 {
            data: [0; SIZE_8PARTY],
            affixed_ribbon: -1,
        }
    }
}

impl From<&[u8; 344]> for PK8 {
    fn from(data: &[u8; 344]) -> Self {
        let mut array = data.clone();
        decrypt_if_encrypted8(&mut array);
        PK8 {
            data: array,
            ..Default::default()
        }
    }
}

impl From<&[u8]> for PK8 {
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
    pub fn new() -> PK8 {
        return PK8 {
            ..Default::default()
        };
    }

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
    pub fn encryption_constant(mut self: Self, value: u32) -> Self {
        self.set_encryption_constant(value);
        self
    }

    pub fn get_encryption_constant(self: &Self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x00)
    }

    pub fn set_encryption_constant(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x0);
    }

    // Sanity
    pub fn sanity(mut self: Self, value: u16) -> Self {
        self.set_sanity(value);
        self
    }

    pub fn get_sanity(self: &Self) -> u16 {
        bitconverter::to_uint16(&self.data, 0x04)
    }

    pub fn set_sanity(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x4);
    }

    // Checksum
    pub fn checksum(mut self: Self, value: u16) -> Self {
        self.set_checksum(value);
        self
    }

    pub fn get_checksum(self: &Self) -> u16 {
        bitconverter::to_uint16(&self.data, 0x06)
    }

    pub fn set_checksum(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x6);
    }

    // Structure
    // Region A

    // Species
    pub fn species(mut self: Self, value: u32) -> Self {
        self.set_species(value as u16);
        self
    }

    pub fn get_species(self: &Self) -> u16 {
        bitconverter::to_uint16(&self.data, 0x08)
    }

    pub fn set_species(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x8);
    }
}

impl PartialEq for PK8 {
    fn eq(&self, other: &Self) -> bool {
        (self.data.iter().eq(other.data.iter())) && (self.affixed_ribbon == other.affixed_ribbon)
    }
}

impl Eq for PK8 {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pk8_from_array_test() {
        let orbeetle_d = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.pk8"));
        let orbeetle_e = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.ek8"));
        let dracovish = PK8::from(&*include_bytes!("util/tests/data/Dracovish.pk8"));

        assert_eq!(true, orbeetle_d == orbeetle_e);
        assert_eq!(false, dracovish == orbeetle_d);
    }

    #[test]
    fn pk8_from_vec_test() {
        let orbeetle_e = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.ek8"));
        let orbeetle_d = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.pk8").to_vec());

        assert_eq!(true, orbeetle_e == orbeetle_d);
    }

    #[test]
    fn pk8_calc_checksum_test() {
        let orbeetle = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.pk8"));
        let dracovish = PK8::from(&*include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0x4E8E, orbeetle.calculate_checksum());
        assert_eq!(0x3B4C, dracovish.calculate_checksum());
    }

    #[test]
    fn pk8_get_test() {
        let dracovish = PK8::from(&*include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0xAC731A09, dracovish.get_encryption_constant());
        assert_eq!(0x0, dracovish.get_sanity());
        assert_eq!(0x3B4C, dracovish.get_checksum());
        assert_eq!(882, dracovish.get_species());
        assert_eq!("Dracovish", pokedex::get_species(dracovish.get_species()));
    }
}
