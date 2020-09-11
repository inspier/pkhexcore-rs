use core::{convert::TryFrom, fmt, fmt::Debug};
use log_derive::{logfn, logfn_inputs};

use crate::pkm::util::pokecrypto::{decrypt_if_encrypted8, SIZE_8PARTY, SIZE_8STORED};
use crate::util::{bitconverter, flagutil, flagutil::Flag};

// Alignment bytes
// ```
// static UNUSED: [u16; 12] = [
// 0x17, 0x1A, 0x1B, 0x23, 0x33, 0x3E, 0x3F,
// 0x4C, 0x4D, 0x4E, 0x4F,
// 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,

// 0x90, 0x91, 0x92, 0x93,
// 0x9C, 0x9D, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7,

// 0xC5,
// 0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xDC, 0xDD,
// 0xE0, 0xE1, // Old Console Region / Region
// 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7,
// 0x115, 0x11F, // Alignment

// 0x13D, 0x13E, 0x13F,
// 0x140, 0x141, 0x142, 0x143, 0x144, 0x145, 0x146, 0x147,
// ];
// ```

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

impl TryFrom<&[u8]> for PK8 {
    type Error = &'static str;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() >= SIZE_8PARTY {
            let mut array: [u8; SIZE_8PARTY] = [0; SIZE_8PARTY];
            array.copy_from_slice(&data[0..SIZE_8PARTY]);
            decrypt_if_encrypted8(&mut array);
            Ok(PK8 {
                data: array,
                ..Default::default()
            })
        } else {
            Err("Invalid size container provided.")
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

    // PSV
    pub fn psv(self: &Self) -> i32 {
        ((get!(self, pid) >> 16 ^ (get!(self, pid) & 0xFFFF)) >> 4) as i32
    }

    // TSV
    pub fn tsv(self: &Self) -> i32 {
        (get!(self, tid) ^ get!(self, sid)) >> 4
    }

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

    // Flag2
    field!(self; Flag2; get: bool => (self.data[0x22] & 2) == 2; set: bool);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_flag2(self: &mut Self, value: bool) {
        self.data[0x22] = ((self.data[0x22] & !0x02) | (if value { 2 } else { 0 })) as u8
    }

    // Gender
    field!(self; Gender; get: i32 => ((self.data[0x22] >> 2) & 0x3) as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_gender(self: &mut Self, value: u8) {
        self.data[0x22] = ((self.data[0x22] & 0xF3) | (value << 2)) as u8
    }

    // 0x23 alignment unused

    // AltForm
    field!(self; AltForm; get: i32 => bitconverter::to_uint16(&self.data, 0x24) as i32; set: u16);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_alt_form(self: &mut Self, value: u16) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x24)
    }

    // EV_HP
    field!(self; ev_hp; get: i32 => self.data[0x26] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ev_hp(self: &mut Self, value: u8) {
        self.data[0x26] = value
    }

    // EV_ATK
    field!(self; ev_atk; get: i32 => self.data[0x27] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ev_atk(self: &mut Self, value: u8) {
        self.data[0x27] = value
    }

    // EV_DEF
    field!(self; ev_def; get: i32 => self.data[0x28] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ev_def(self: &mut Self, value: u8) {
        self.data[0x28] = value
    }

    // EV_SPE
    field!(self; ev_spe; get: i32 => self.data[0x29] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ev_spe(self: &mut Self, value: u8) {
        self.data[0x29] = value
    }

    // EV_SPA
    field!(self; ev_spa; get: i32 => self.data[0x2A] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ev_spa(self: &mut Self, value: u8) {
        self.data[0x2A] = value
    }

    // EV_SPD
    field!(self; ev_spd; get: i32 => self.data[0x2B] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ev_spd(self: &mut Self, value: u8) {
        self.data[0x2B] = value
    }

    // CNT_Cool
    field!(self; cnt_Cool; get: i32 => self.data[0x2C] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_cnt_cool(self: &mut Self, value: u8) {
        self.data[0x2C] = value;
    }

    // CNT_Beauty
    field!(self; cnt_Beauty; get: i32 => self.data[0x2D] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_cnt_beauty(self: &mut Self, value: u8) {
        self.data[0x2D] = value;
    }

    // CNT_Cute
    field!(self; cnt_Cute; get: i32 => self.data[0x2E] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_cnt_cute(self: &mut Self, value: u8) {
        self.data[0x2E] = value as u8;
    }

    // CNT_Smart
    field!(self; cnt_Smart; get: i32 => self.data[0x2F] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_cnt_smart(self: &mut Self, value: u8) {
        self.data[0x2F] = value as u8;
    }

    // CNT_Tough
    field!(self; cnt_Tough; get: i32 => self.data[0x30] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_cnt_tough(self: &mut Self, value: u8) {
        self.data[0x30] = value as u8;
    }

    // CNT_Sheen
    field!(self; cnt_Sheen; get: i32 => self.data[0x31] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_cnt_sheen(self: &mut Self, value: u8) {
        self.data[0x31] = value as u8;
    }

    // pkrs
    fn get_pkrs(self: &Self) -> u8 {
        self.data[0x32]
    }

    fn set_pkrs(self: &mut Self, value: u8) {
        self.data[0x32] = value;
    }

    // pkrs_Days
    field!(self; pkrs_Days; get: i32 => (get!(self, pkrs) & 0xF) as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_pkrs_days(self: &mut Self, value: u8) {
        set!(self, pkrs, ((get!(self, pkrs) & !0xF) | value) as u8);
    }

    // pkrs_Strain
    field!(self; pkrs_Strain; get: i32 => (get!(self, pkrs) >> 4) as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_pkrs_strain(self: &mut Self, value: u8) {
        set!(self, pkrs, ((get!(self, pkrs) & 0xF) | value << 4) as u8);
    }

    // 0x33 unused padding

    // ribbon u32
    // RibbonChampionKalos
    field!(self; ribbonChampionKalos; get: Flag => flagutil::get_flag(&self.data, 0x34, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_kalos(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 0, value);
    }

    // RibbonChampionG3Hoenn
    field!(self; ribbonChampionG3Hoenn; get: Flag => flagutil::get_flag(&self.data, 0x34, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_g3_hoenn(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 1, value);
    }

    // RibbonChampionSinnoh
    field!(self; ribbonChampionSinnoh; get: Flag => flagutil::get_flag(&self.data, 0x34, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_sinnoh(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 2, value);
    }

    // RibbonBestFriends
    field!(self; ribbonBestFriends; get: Flag => flagutil::get_flag(&self.data, 0x34, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_best_friends(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 3, value);
    }

    // RibbonTraining
    field!(self; ribbonTraining; get: Flag => flagutil::get_flag(&self.data, 0x34, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_training(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 4, value);
    }

    // RibbonBattlerSkillful
    field!(self; ribbonBattlerSkillful; get: Flag => flagutil::get_flag(&self.data, 0x34, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_battler_skillful(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 5, value);
    }

    // RibbonBattlerExpert
    field!(self; ribbonBattlerExpert; get: Flag => flagutil::get_flag(&self.data, 0x34, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_battler_expert(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 6, value);
    }

    // RibbonEffort
    field!(self; ribbonEffort; get: Flag => flagutil::get_flag(&self.data, 0x34, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_effort(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x34, 7, value);
    }

    // RibbonAlert
    field!(self; ribbonAlert; get: Flag => flagutil::get_flag(&self.data, 0x35, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_alert(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 0, value);
    }

    // RibbonShock
    field!(self; ribbonShock; get: Flag => flagutil::get_flag(&self.data, 0x35, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_shock(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 1, value);
    }

    // RibbonDowncast
    field!(self; ribbonDowncast; get: Flag => flagutil::get_flag(&self.data, 0x35, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_downcast(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 2, value);
    }

    // RibbonCareless
    field!(self; ribbonCareless; get: Flag => flagutil::get_flag(&self.data, 0x35, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_careless(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 3, value);
    }

    // RibbonRelax
    field!(self; ribbonRelax; get: Flag => flagutil::get_flag(&self.data, 0x35, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_relax(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 4, value);
    }

    // RibbonSnooze
    field!(self; ribbonSnooze; get: Flag => flagutil::get_flag(&self.data, 0x35, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_snooze(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 5, value);
    }

    // RibbonSmile
    field!(self; ribbonSmile; get: Flag => flagutil::get_flag(&self.data, 0x35, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_smile(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 6, value);
    }

    // RibbonGorgeous
    field!(self; ribbonGorgeous; get: Flag => flagutil::get_flag(&self.data, 0x35, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_gorgeous(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x35, 7, value);
    }

    // RibbonRoyal
    field!(self; ribbonRoyal; get: Flag => flagutil::get_flag(&self.data, 0x36, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_royal(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 0, value);
    }

    // RibbonGorgeousRoyal
    field!(self; ribbonGorgeousRoyal; get: Flag => flagutil::get_flag(&self.data, 0x36, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_gorgeous_royal(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 1, value);
    }

    // RibbonArtist
    field!(self; ribbonArtist; get: Flag => flagutil::get_flag(&self.data, 0x36, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_artist(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 2, value);
    }

    // RibbonFootprint
    field!(self; ribbonFootprint; get: Flag => flagutil::get_flag(&self.data, 0x36, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_footprint(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 3, value);
    }

    // RibbonRecord
    field!(self; ribbonRecord; get: Flag => flagutil::get_flag(&self.data, 0x36, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_record(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 4, value);
    }

    // RibbonLegend
    field!(self; ribbonLegend; get: Flag => flagutil::get_flag(&self.data, 0x36, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_legend(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 5, value);
    }

    // RibbonCountry
    field!(self; ribbonCountry; get: Flag => flagutil::get_flag(&self.data, 0x36, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_country(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 6, value);
    }

    // RibbonNational
    field!(self; ribbonNational; get: Flag => flagutil::get_flag(&self.data, 0x36, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_national(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x36, 7, value);
    }

    // RibbonEarth
    field!(self; ribbonEarth; get: Flag => flagutil::get_flag(&self.data, 0x37, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_earth(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 0, value);
    }

    // RibbonWorld
    field!(self; ribbonWorld; get: Flag => flagutil::get_flag(&self.data, 0x37, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_world(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 1, value);
    }

    // RibbonClassic
    field!(self; ribbonClassic; get: Flag => flagutil::get_flag(&self.data, 0x37, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_classic(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 2, value);
    }

    // RibbonPremier
    field!(self; ribbonPremier; get: Flag => flagutil::get_flag(&self.data, 0x37, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_premier(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 3, value);
    }

    // RibbonEvent
    field!(self; ribbonEvent; get: Flag => flagutil::get_flag(&self.data, 0x37, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_event(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 4, value);
    }

    // RibbonBirthday
    field!(self; ribbonBirthday; get: Flag => flagutil::get_flag(&self.data, 0x37, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_birthday(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 5, value);
    }

    // RibbonSpecial
    field!(self; ribbonSpecial; get: Flag => flagutil::get_flag(&self.data, 0x37, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_special(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 6, value);
    }

    // RibbonSouvenir
    field!(self; ribbonSouvenir; get: Flag => flagutil::get_flag(&self.data, 0x37, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_souvenir(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x37, 7, value);
    }

    // ribbon u32
    // RibbonWishing
    field!(self; ribbonWishing; get: Flag => flagutil::get_flag(&self.data, 0x38, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_wishing(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 0, value);
    }

    // RibbonChampionBattle
    field!(self; ribbonChampionBattle; get: Flag => flagutil::get_flag(&self.data, 0x38, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_battle(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 1, value);
    }

    // RibbonChampionRegional
    field!(self; ribbonChampionRegional; get: Flag => flagutil::get_flag(&self.data, 0x38, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_regional(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 2, value);
    }

    // RibbonChampionNational
    field!(self; ribbonChampionNational; get: Flag => flagutil::get_flag(&self.data, 0x38, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_national(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 3, value);
    }

    // RibbonChampionWorld
    field!(self; ribbonChampionWorld; get: Flag => flagutil::get_flag(&self.data, 0x38, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_world(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 4, value);
    }

    // HasContestMemoryRibbon
    field!(self; HasContestMemoryRibbon; get: Flag => flagutil::get_flag(&self.data, 0x38, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_has_contest_memory_ribbon(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 5, value);
    }

    // HasBattleMemoryRibbon
    field!(self; HasBattleMemoryRibbon; get: Flag => flagutil::get_flag(&self.data, 0x38, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_has_battle_memory_ribbon(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 6, value);
    }

    // RibbonChampionG6Hoenn
    field!(self; ribbonChampionG6Hoenn; get: Flag => flagutil::get_flag(&self.data, 0x38, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_g6_hoenn(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x38, 7, value);
    }

    // RibbonContestStar
    field!(self; ribbonContestStar; get: Flag => flagutil::get_flag(&self.data, 0x39, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_contest_star(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 0, value);
    }

    // RibbonMasterCoolness
    field!(self; ribbonMasterCoolness; get: Flag => flagutil::get_flag(&self.data, 0x39, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_master_coolness(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 1, value);
    }

    // RibbonMasterBeauty
    field!(self; ribbonMasterBeauty; get: Flag => flagutil::get_flag(&self.data, 0x39, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_master_beauty(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 2, value);
    }

    // RibbonMasterCuteness
    field!(self; ribbonMasterCuteness; get: Flag => flagutil::get_flag(&self.data, 0x39, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_master_cuteness(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 3, value);
    }

    // RibbonMasterCleverness
    field!(self; ribbonMasterCleverness; get: Flag => flagutil::get_flag(&self.data, 0x39, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_master_cleverness(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 4, value);
    }

    // RibbonMasterToughness
    field!(self; ribbonMasterToughness; get: Flag => flagutil::get_flag(&self.data, 0x39, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_master_toughness(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 5, value);
    }

    // RibbonChampionAlola
    field!(self; ribbonChampionAlola; get: Flag => flagutil::get_flag(&self.data, 0x39, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_alola(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 6, value);
    }

    // RibbonBattleRoyale
    field!(self; ribbonBattleRoyale; get: Flag => flagutil::get_flag(&self.data, 0x39, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_battle_royale(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x39, 7, value);
    }

    // RibbonBattleTreeGreat
    field!(self; ribbonBattleTreeGreat; get: Flag => flagutil::get_flag(&self.data, 0x3A, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_battle_tree_great(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 0, value);
    }

    // RibbonBattleTreeMaster
    field!(self; ribbonBattleTreeMaster; get: Flag => flagutil::get_flag(&self.data, 0x3A, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_battle_tree_master(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 1, value);
    }

    // RibbonChampionGalar
    field!(self; ribbonChampionGalar; get: Flag => flagutil::get_flag(&self.data, 0x3A, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_champion_galar(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 2, value);
    }

    // RibbonTowerMaster
    field!(self; ribbonTowerMaster; get: Flag => flagutil::get_flag(&self.data, 0x3A, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_tower_master(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 3, value);
    }

    // RibbonMasterRank
    field!(self; ribbonMasterRank; get: Flag => flagutil::get_flag(&self.data, 0x3A, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_master_rank(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 4, value);
    }

    // RibbonMarkLunchtime
    field!(self; ribbonMarkLunchtime; get: Flag => flagutil::get_flag(&self.data, 0x3A, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_lunchtime(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 5, value);
    }

    // RibbonMarkSleepyTime
    field!(self; ribbonMarkSleepyTime; get: Flag => flagutil::get_flag(&self.data, 0x3A, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_sleepy_time(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 6, value);
    }

    // RibbonMarkDusk
    field!(self; ribbonMarkDusk; get: Flag => flagutil::get_flag(&self.data, 0x3A, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_dusk(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3A, 7, value);
    }

    // RibbonMarkDawn
    field!(self; ribbonMarkDawn; get: Flag => flagutil::get_flag(&self.data, 0x3B, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_dawn(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 0, value);
    }

    // RibbonMarkCloudy
    field!(self; ribbonMarkCloudy; get: Flag => flagutil::get_flag(&self.data, 0x3B, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_cloudy(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 1, value);
    }

    // RibbonMarkRainy
    field!(self; ribbonMarkRainy; get: Flag => flagutil::get_flag(&self.data, 0x3B, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_rainy(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 2, value);
    }

    // RibbonMarkStormy
    field!(self; ribbonMarkStormy; get: Flag => flagutil::get_flag(&self.data, 0x3B, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_stormy(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 3, value);
    }

    // RibbonMarkSnowy
    field!(self; ribbonMarkSnowy; get: Flag => flagutil::get_flag(&self.data, 0x3B, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_snowy(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 4, value);
    }

    // RibbonMarkBlizzard
    field!(self; ribbonMarkBlizzard; get: Flag => flagutil::get_flag(&self.data, 0x3B, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_blizzard(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 5, value);
    }

    // RibbonMarkDry
    field!(self; ribbonMarkDry; get: Flag => flagutil::get_flag(&self.data, 0x3B, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_dry(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 6, value);
    }

    // RibbonMarkSandstorm
    field!(self; ribbonMarkSandstorm; get: Flag => flagutil::get_flag(&self.data, 0x3B, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_sandstorm(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x3B, 7, value);
    }

    // RibbonCountMemoryContest
    field!(self; ribbonCountMemoryContest; get: i32 => self.data[0x3C] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_count_memory_contest(self: &mut Self, value: u8) {
        self.data[0x3C] = value;
        set!(self, HasContestMemoryRibbon, Flag::from(value != 0));
    }

    // RibbonCountMemoryBattle
    field!(self; ribbonCountMemoryBattle; get: i32 => self.data[0x3D] as i32; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_count_memory_battle(self: &mut Self, value: u8) {
        self.data[0x3D] = value;
        set!(self, HasBattleMemoryRibbon, Flag::from(value != 0));
    }

    // 0x3E padding
    // 0x3F padding

    // 0x40 Ribbon 1
    // RibbonMarkMisty
    field!(self; ribbonMarkMisty; get: Flag => flagutil::get_flag(&self.data, 0x40, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_misty(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 0, value);
    }

    // RibbonMarkDestiny
    field!(self; ribbonMarkDestiny; get: Flag => flagutil::get_flag(&self.data, 0x40, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_destiny(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 1, value);
    }

    // RibbonMarkFishing
    field!(self; ribbonMarkFishing; get: Flag => flagutil::get_flag(&self.data, 0x40, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_fishing(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 2, value);
    }

    // RibbonMarkCurry
    field!(self; ribbonMarkCurry; get: Flag => flagutil::get_flag(&self.data, 0x40, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_curry(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 3, value);
    }

    // RibbonMarkUncommon
    field!(self; ribbonMarkUncommon; get: Flag => flagutil::get_flag(&self.data, 0x40, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_uncommon(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 4, value);
    }

    // RibbonMarkRare
    field!(self; ribbonMarkRare; get: Flag => flagutil::get_flag(&self.data, 0x40, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_rare(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 5, value);
    }

    // RibbonMarkRowdy
    field!(self; ribbonMarkRowdy; get: Flag => flagutil::get_flag(&self.data, 0x40, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_rowdy(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 6, value);
    }

    // RibbonMarkAbsentMinded
    field!(self; ribbonMarkAbsentMinded; get: Flag => flagutil::get_flag(&self.data, 0x40, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_absent_minded(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x40, 7, value);
    }

    // RibbonMarkJittery
    field!(self; ribbonMarkJittery; get: Flag => flagutil::get_flag(&self.data, 0x41, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_jittery(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 0, value);
    }

    // RibbonMarkExcited
    field!(self; ribbonMarkExcited; get: Flag => flagutil::get_flag(&self.data, 0x41, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_excited(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 1, value);
    }

    // RibbonMarkCharismatic
    field!(self; ribbonMarkCharismatic; get: Flag => flagutil::get_flag(&self.data, 0x41, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_charismatic(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 2, value);
    }

    // RibbonMarkCalmness
    field!(self; ribbonMarkCalmness; get: Flag => flagutil::get_flag(&self.data, 0x41, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_calmness(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 3, value);
    }

    // RibbonMarkIntense
    field!(self; ribbonMarkIntense; get: Flag => flagutil::get_flag(&self.data, 0x41, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_intense(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 4, value);
    }

    // RibbonMarkZonedOut
    field!(self; ribbonMarkZonedOut; get: Flag => flagutil::get_flag(&self.data, 0x41, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_zoned_out(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 5, value);
    }

    // RibbonMarkJoyful
    field!(self; ribbonMarkJoyful; get: Flag => flagutil::get_flag(&self.data, 0x41, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_joyful(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 6, value);
    }

    // RibbonMarkAngry
    field!(self; ribbonMarkAngry; get: Flag => flagutil::get_flag(&self.data, 0x41, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_angry(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 7, value);
    }

    // RibbonMarkSmiley
    field!(self; ribbonMarkSmiley; get: Flag => flagutil::get_flag(&self.data, 0x42, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_smiley(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 0, value);
    }

    // RibbonMarkTeary
    field!(self; ribbonMarkTeary; get: Flag => flagutil::get_flag(&self.data, 0x42, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_teary(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 1, value);
    }

    // RibbonMarkUpbeat
    field!(self; ribbonMarkUpbeat; get: Flag => flagutil::get_flag(&self.data, 0x42, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_upbeat(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 2, value);
    }

    // RibbonMarkPeeved
    field!(self; ribbonMarkPeeved; get: Flag => flagutil::get_flag(&self.data, 0x42, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_peeved(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 3, value);
    }

    // RibbonMarkIntellectual
    field!(self; ribbonMarkIntellectual; get: Flag => flagutil::get_flag(&self.data, 0x42, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_intellectual(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 4, value);
    }

    // RibbonMarkFerocious
    field!(self; ribbonMarkFerocious; get: Flag => flagutil::get_flag(&self.data, 0x42, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_ferocious(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 5, value);
    }

    // RibbonMarkCrafty
    field!(self; ribbonMarkCrafty; get: Flag => flagutil::get_flag(&self.data, 0x42, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_crafty(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 6, value);
    }

    // RibbonMarkScowling
    field!(self; ribbonMarkScowling; get: Flag => flagutil::get_flag(&self.data, 0x42, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_scowling(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x42, 7, value);
    }

    // RibbonMarkKindly
    field!(self; ribbonMarkKindly; get: Flag => flagutil::get_flag(&self.data, 0x43, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_kindly(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 0, value);
    }

    // RibbonMarkFlustered
    field!(self; ribbonMarkFlustered; get: Flag => flagutil::get_flag(&self.data, 0x43, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_flustered(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 1, value);
    }

    // RibbonMarkPumpedUp
    field!(self; ribbonMarkPumpedUp; get: Flag => flagutil::get_flag(&self.data, 0x43, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_pumped_up(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 2, value);
    }

    // RibbonMarkZeroEnergy
    field!(self; ribbonMarkZeroEnergy; get: Flag => flagutil::get_flag(&self.data, 0x43, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_zero_energy(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 3, value);
    }

    // RibbonMarkPrideful
    field!(self; ribbonMarkPrideful; get: Flag => flagutil::get_flag(&self.data, 0x43, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_prideful(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 4, value);
    }

    // RibbonMarkUnsure
    field!(self; ribbonMarkUnsure; get: Flag => flagutil::get_flag(&self.data, 0x43, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_unsure(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 5, value);
    }

    // RibbonMarkHumble
    field!(self; ribbonMarkHumble; get: Flag => flagutil::get_flag(&self.data, 0x43, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_humble(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 6, value);
    }

    // RibbonMarkThorny
    field!(self; ribbonMarkThorny; get: Flag => flagutil::get_flag(&self.data, 0x43, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_thorny(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x43, 7, value);
    }

    // 0x44 Ribbon 2

    // RibbonMarkVigor
    field!(self; ribbonMarkVigor; get: Flag => flagutil::get_flag(&self.data, 0x44, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_vigor(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 0, value);
    }

    // RibbonMarkSlump
    field!(self; ribbonMarkSlump; get: Flag => flagutil::get_flag(&self.data, 0x44, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_ribbon_mark_slump(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 1, value);
    }

    // RIB44_2
    field!(self; rib44_2; get: Flag => flagutil::get_flag(&self.data, 0x44, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib44_2(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 2, value);
    }

    // RIB44_3
    field!(self; rib44_3; get: Flag => flagutil::get_flag(&self.data, 0x44, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib44_3(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 3, value);
    }

    // RIB44_4
    field!(self; rib44_4; get: Flag => flagutil::get_flag(&self.data, 0x44, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib44_4(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 4, value);
    }

    // RIB44_5
    field!(self; rib44_5; get: Flag => flagutil::get_flag(&self.data, 0x44, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib44_5(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 5, value);
    }

    // RIB44_6
    field!(self; rib44_6; get: Flag => flagutil::get_flag(&self.data, 0x44, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib44_6(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 6, value);
    }

    // RIB44_7
    field!(self; rib44_7; get: Flag => flagutil::get_flag(&self.data, 0x44, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib44_7(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x44, 7, value);
    }

    // RIB45_0
    field!(self; rib45_0; get: Flag => flagutil::get_flag(&self.data, 0x45, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_0(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 0, value);
    }

    // RIB45_1
    field!(self; rib45_1; get: Flag => flagutil::get_flag(&self.data, 0x45, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_1(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 1, value);
    }

    // RIB45_2
    field!(self; rib45_2; get: Flag => flagutil::get_flag(&self.data, 0x45, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_2(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 2, value);
    }

    // RIB45_3
    field!(self; rib45_3; get: Flag => flagutil::get_flag(&self.data, 0x45, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_3(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 3, value);
    }

    // RIB45_4
    field!(self; rib45_4; get: Flag => flagutil::get_flag(&self.data, 0x45, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_4(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 4, value);
    }

    // RIB45_5
    field!(self; rib45_5; get: Flag => flagutil::get_flag(&self.data, 0x45, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_5(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 5, value);
    }

    // RIB45_6
    field!(self; rib45_6; get: Flag => flagutil::get_flag(&self.data, 0x45, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_6(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 6, value);
    }

    // RIB45_7
    field!(self; rib45_7; get: Flag => flagutil::get_flag(&self.data, 0x45, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib45_7(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x45, 7, value);
    }

    // RIB46_0
    field!(self; rib46_0; get: Flag => flagutil::get_flag(&self.data, 0x41, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_0(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x41, 0, value);
    }

    // RIB46_1
    field!(self; rib46_1; get: Flag => flagutil::get_flag(&self.data, 0x46, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_1(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 1, value);
    }

    // RIB46_2
    field!(self; rib46_2; get: Flag => flagutil::get_flag(&self.data, 0x46, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_2(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 2, value);
    }

    // RIB46_3
    field!(self; rib46_3; get: Flag => flagutil::get_flag(&self.data, 0x46, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_3(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 3, value);
    }

    // RIB46_4
    field!(self; rib46_4; get: Flag => flagutil::get_flag(&self.data, 0x46, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_4(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 4, value);
    }

    // RIB46_5
    field!(self; rib46_5; get: Flag => flagutil::get_flag(&self.data, 0x46, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_5(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 5, value);
    }

    // RIB46_6
    field!(self; rib46_6; get: Flag => flagutil::get_flag(&self.data, 0x46, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_6(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 6, value);
    }

    // RIB46_7
    field!(self; rib46_7; get: Flag => flagutil::get_flag(&self.data, 0x46, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib46_7(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x46, 7, value);
    }

    // RIB47_0
    field!(self; rib47_0; get: Flag => flagutil::get_flag(&self.data, 0x47, 0); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_0(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 0, value);
    }

    // RIB47_1
    field!(self; rib47_1; get: Flag => flagutil::get_flag(&self.data, 0x47, 1); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_1(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 1, value);
    }

    // RIB47_2
    field!(self; rib47_2; get: Flag => flagutil::get_flag(&self.data, 0x47, 2); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_2(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 2, value);
    }

    // RIB47_3
    field!(self; rib47_3; get: Flag => flagutil::get_flag(&self.data, 0x47, 3); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_3(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 3, value);
    }

    // RIB47_4
    field!(self; rib47_4; get: Flag => flagutil::get_flag(&self.data, 0x47, 4); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_4(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 4, value);
    }

    // RIB47_5
    field!(self; rib47_5; get: Flag => flagutil::get_flag(&self.data, 0x47, 5); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_5(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 5, value);
    }

    // RIB47_6
    field!(self; rib47_6; get: Flag => flagutil::get_flag(&self.data, 0x47, 6); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_6(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 6, value);
    }

    // RIB47_7
    field!(self; rib47_7; get: Flag => flagutil::get_flag(&self.data, 0x47, 7); set: Flag);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_rib47_7(self: &mut Self, value: Flag) {
        flagutil::set_flag(&mut self.data, 0x47, 7, value);
    }

    // U48
    field!(self; U48; get: u32 => bitconverter::to_uint32(&self.data, 0x48); set: u32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_u48(self: &mut Self, value: u32) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x48);
    }

    // 0x4C-0x4F unused

    // HeightScalar
    field!(self; HeightScalar; get: i32 => self.data[0x50] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_height_scalar(self: &mut Self, value: i32) {
        self.data[0x50] = value as u8;
    }

    // WeightScalar
    field!(self; WeightScalar; get: i32 => self.data[0x51] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_weight_scalar(self: &mut Self, value: i32) {
        self.data[0x51] = value as u8;
    }

    // 0x52-0x57 unused

    // Nickname
    // field!(self; Nickname; get: string => GetString(0x58, 24); set: NULL);

    // #[logfn(INFO)]
    // #[logfn_inputs(Debug)]
    // pub fn set_nickname(self: &mut Self, value: NULL) { SetString(value, 12).copy_to(&mut self.data, 0x58); }

    // 2 bytes for \0, automatically handled above

    // Move1
    field!(self; Move1; get: i32 => bitconverter::to_uint16(&self.data, 0x72) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move1(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x72);
    }

    // Move2
    field!(self; Move2; get: i32 => bitconverter::to_uint16(&self.data, 0x74) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move2(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x74);
    }

    // Move3
    field!(self; Move3; get: i32 => bitconverter::to_uint16(&self.data, 0x76) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move3(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x76);
    }

    // Move4
    field!(self; Move4; get: i32 => bitconverter::to_uint16(&self.data, 0x78) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move4(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x78);
    }

    // Move1_PP
    field!(self; Move1_pp; get: i32 => self.data[0x7A] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move1_pp(self: &mut Self, value: i32) {
        self.data[0x7A] = value as u8;
    }

    // Move2_PP
    field!(self; Move2_pp; get: i32 => self.data[0x7B] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move2_pp(self: &mut Self, value: i32) {
        self.data[0x7B] = value as u8;
    }

    // Move3_PP
    field!(self; Move3_pp; get: i32 => self.data[0x7C] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move3_pp(self: &mut Self, value: i32) {
        self.data[0x7C] = value as u8;
    }

    // Move4_PP
    field!(self; Move4_pp; get: i32 => self.data[0x7D] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move4_pp(self: &mut Self, value: i32) {
        self.data[0x7D] = value as u8;
    }

    // Move1_PPUps
    field!(self; Move1_ppUps; get: i32 => self.data[0x7E] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move1_pp_ups(self: &mut Self, value: i32) {
        self.data[0x7E] = value as u8;
    }

    // Move2_PPUps
    field!(self; Move2_ppUps; get: i32 => self.data[0x7F] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move2_pp_ups(self: &mut Self, value: i32) {
        self.data[0x7F] = value as u8;
    }

    // Move3_PPUps
    field!(self; Move3_ppUps; get: i32 => self.data[0x80] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move3_pp_ups(self: &mut Self, value: i32) {
        self.data[0x80] = value as u8;
    }

    // Move4_PPUps
    field!(self; Move4_ppUps; get: i32 => self.data[0x81] as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_move4_pp_ups(self: &mut Self, value: i32) {
        self.data[0x81] = value as u8;
    }

    // RelearnMove1
    field!(self; RelearnMove1; get: i32 => bitconverter::to_uint16(&self.data, 0x82) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_relearn_move1(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x82);
    }

    // RelearnMove2
    field!(self; RelearnMove2; get: i32 => bitconverter::to_uint16(&self.data, 0x84) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_relearn_move2(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x84);
    }

    // RelearnMove3
    field!(self; RelearnMove3; get: i32 => bitconverter::to_uint16(&self.data, 0x86) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_relearn_move3(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x86);
    }

    // RelearnMove4
    field!(self; RelearnMove4; get: i32 => bitconverter::to_uint16(&self.data, 0x88) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_relearn_move4(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x88);
    }

    // Stat_HPCurrent
    field!(self; Stat_hpCurrent; get: i32 => bitconverter::to_uint16(&self.data, 0x8A) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_stat_hp_current(self: &mut Self, value: i32) {
        bitconverter::get_bytes(value as u16).copy_to(&mut self.data, 0x8A);
    }

    // IV32
    fn get_iv32(self: &Self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x8C)
    }

    fn set_iv32(self: &mut Self, value: u8) {
        bitconverter::get_bytes(value).copy_to(&mut self.data, 0x8C);
    }

    // IV_HP
    field!(self; iv_hp; get: i32 => ((get!(self, iv32) >> 00) & 0x1F) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_iv_hp(self: &mut Self, value: i32) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !(0x1Fu32 << 00))
                | ((if value > 31 { 31 } else { value as u32 }) << 00)) as u8
        );
    }

    // IV_ATK
    field!(self; iv_atk; get: i32 => ((get!(self, iv32) >> 05) & 0x1F) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_iv_atk(self: &mut Self, value: i32) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !(0x1Fu32 << 05))
                | ((if value > 31 { 31 } else { value as u32 }) << 05)) as u8
        );
    }

    // IV_DEF
    field!(self; iv_def; get: i32 => ((get!(self, iv32) >> 10) & 0x1F) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_iv_def(self: &mut Self, value: i32) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !(0x1Fu32 << 10))
                | ((if value > 31 { 31 } else { value as u32 }) << 10)) as u8
        );
    }

    // IV_SPE
    field!(self; iv_spe; get: i32 => ((get!(self, iv32) >> 15) & 0x1F) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_iv_spe(self: &mut Self, value: i32) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !(0x1Fu32 << 15))
                | ((if value > 31 { 31 } else { value as u32 }) << 15)) as u8
        );
    }

    // IV_SPA
    field!(self; iv_spa; get: i32 => ((get!(self, iv32) >> 20) & 0x1F) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_iv_spa(self: &mut Self, value: i32) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !(0x1Fu32 << 20))
                | ((if value > 31 { 31 } else { value as u32 }) << 20)) as u8
        );
    }

    // IV_SPD
    field!(self; iv_spd; get: i32 => ((get!(self, iv32) >> 25) & 0x1F) as i32; set: i32);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_iv_spd(self: &mut Self, value: i32) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !(0x1Fu32 << 25))
                | ((if value > 31 { 31 } else { value as u32 }) << 25)) as u8
        );
    }

    // IsEgg
    field!(self; IsEgg; get: bool => ((get!(self, iv32) >> 30) & 1) == 1; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_is_egg(self: &mut Self, value: u8) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & !0x40000000u32) | (if value != 0 { 0x40000000 } else { 0 })) as u8
        );
    }

    // IsNicknamed
    field!(self; IsNicknamed; get: bool => ((get!(self, iv32) >> 31) & 1) == 1; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_is_nicknamed(self: &mut Self, value: u8) {
        set!(
            self,
            iv32,
            ((get!(self, iv32) & 0x7FFFFFFFu32) | (if value != 0 { 0x80000000 } else { 0 })) as u8
        );
    }

    // DynamaxLevel
    field!(self; DynamaxLevel; get: u8 => self.data[0x90]; set: u8);

    #[logfn(INFO)]
    #[logfn_inputs(Debug)]
    pub fn set_dynamax_level(self: &mut Self, value: u8) {
        self.data[0x90] = value;
    }

    // 0x90-0x93 unused
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
    use crate::game::enums::{ability::Ability, gender::Gender, nature::Nature, species::Species};

    #[test]
    fn pk8_from_array_test() {
        let orbeetle_d = PK8::from(include_bytes!("util/tests/data/Orbeetle.pk8"));
        let orbeetle_e = PK8::from(include_bytes!("util/tests/data/Orbeetle.ek8"));
        let dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));

        assert_eq!(true, orbeetle_d == orbeetle_e);
        assert_eq!(false, dracovish == orbeetle_d);
    }

    #[test]
    fn pk8_from_vec_test() {
        let orbeetle_e = PK8::from(include_bytes!("util/tests/data/Orbeetle.ek8"));
        let orbeetle_d = PK8::try_from(&*include_bytes!("util/tests/data/Orbeetle.pk8")).unwrap();

        assert_eq!(true, orbeetle_e == orbeetle_d);
    }

    #[test]
    fn pk8_calc_checksum_test() {
        let orbeetle = PK8::from(include_bytes!("util/tests/data/Orbeetle.pk8"));
        let dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0x4E8E, orbeetle.calculate_checksum());
        assert_eq!(0x5D57, dracovish.calculate_checksum());
    }

    #[test]
    fn pk8_get_test_1() {
        let dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0xAC731A09, get!(dracovish, EncryptionConstant));
        assert_eq!(0x0, get!(dracovish, Sanity));
        assert_eq!(0x5D57, get!(dracovish, Checksum));
        assert_eq!(882, get!(dracovish, Species));
        assert_eq!(i32::from(Species::Dracovish), get!(dracovish, Species));
        assert_eq!(268, get!(dracovish, HeldItem));
        assert_eq!(30756, get!(dracovish, tid));
        assert_eq!(45312, get!(dracovish, sid));
        assert_eq!(1250000, get!(dracovish, Exp));
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
        assert_eq!(false, get!(dracovish, Flag2));
        assert_eq!(i32::from(Gender::Genderless), get!(dracovish, Gender));
        assert_eq!(0, get!(dracovish, AltForm));
        assert_eq!(4, get!(dracovish, ev_hp));
        assert_eq!(252, get!(dracovish, ev_atk));
        assert_eq!(0, get!(dracovish, ev_def));
        assert_eq!(252, get!(dracovish, ev_spe));
        assert_eq!(0, get!(dracovish, ev_spa));
        assert_eq!(0, get!(dracovish, ev_spd));
    }

    #[test]
    fn pk8_get_test_2() {
        let dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(88, get!(dracovish, HeightScalar));
        assert_eq!(33, get!(dracovish, WeightScalar));
        assert_eq!(35, get!(dracovish, Move1_pp));
        assert_eq!(25, get!(dracovish, Move2_pp));
        assert_eq!(10, get!(dracovish, Move3_pp));
        assert_eq!(16, get!(dracovish, Move4_pp));
        assert_eq!(0, get!(dracovish, Move1_ppUps));
        assert_eq!(0, get!(dracovish, Move2_ppUps));
        assert_eq!(0, get!(dracovish, Move3_ppUps));
        assert_eq!(3, get!(dracovish, Move4_ppUps));
        assert_eq!(31, get!(dracovish, iv_hp));
        assert_eq!(31, get!(dracovish, iv_atk));
        assert_eq!(31, get!(dracovish, iv_def));
        assert_eq!(23, get!(dracovish, iv_spa));
        assert_eq!(2, get!(dracovish, iv_spd));
        assert_eq!(4, get!(dracovish, iv_spe));
        assert_eq!(10, get!(dracovish, DynamaxLevel));
    }

    #[test]
    fn pk8_set_test() {
        let mut dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0xAC731A09, get!(dracovish, EncryptionConstant));
        set!(dracovish, EncryptionConstant, 0xDEADBEEF);
        assert_eq!(0xDEADBEEF, get!(dracovish, EncryptionConstant));
    }
}
