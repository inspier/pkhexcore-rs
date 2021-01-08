#![allow(non_snake_case)]
use alloc::{format, string::String, vec::Vec};
use core::convert::TryFrom;
use deku::prelude::*;

use crate::game::enums::{
    ability::Ability, flag::Flag, gender::Gender, nature::Nature, species::Species,
};
use crate::pkm::strings::string_converter::get_string7;
use crate::pkm::util::pokecrypto::{decrypt_if_encrypted8, get_chk, SIZE_8PARTY, SIZE_8STORED};

#[allow(dead_code)]
static FORMAT: u32 = 8;

pub const MAX_IV: i32 = 31;
pub const MAX_EV: i32 = 252;
pub const OT_LENGTH: usize = 12;
pub const NICK_LENGTH: usize = 12;

// TODO: PersonalInfo

pub struct PK8 {
    data: [u8; SIZE_8PARTY],
    affixed_ribbon: i8, // 00 would make it show Kalos Champion
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PK8Config {
    encryption_constant: u32,
    sanity: u16,
    #[deku(
        update = "get_chk::<SIZE_8PARTY>(&<[u8; SIZE_8PARTY]>::try_from(self.to_bytes().unwrap()).unwrap(), SIZE_8STORED)"
    )]
    checksum: u16,

    // Block A
    species: Species,
    held_item: u16,
    tid: u16,
    sid: u16,
    exp: u32,
    #[deku(pad_bits_after = "3")]
    ability: Ability,
    #[deku(bits = 1)]
    can_gigantamax: u8,
    #[deku(bits = 1)]
    favourite: u8,
    #[deku(bits = 3)]
    ability_number: u8,
    // 0x17 alignment unused
    #[deku(pad_bytes_before = "1")]
    mark_value: u16,
    // 0x1A alignment unused
    // 0x1B alignment unused
    #[deku(pad_bytes_before = "2")]
    pid: u32,
    nature: Nature,
    #[deku(pad_bits_after = "4")]
    stat_nature: Nature,
    gender: Gender,
    #[deku(bits = 1)]
    flag2: u8,
    #[deku(bits = 1)]
    fateful_encounter: u8,
    // 0x23 alignment unused
    #[deku(pad_bytes_before = "1")]
    alt_form: u16,
    ev_hp: u8,
    ev_atk: u8,
    ev_def: u8,
    ev_spe: u8,
    ev_spa: u8,
    ev_spd: u8,
    cnt_cool: u8,
    cnt_beauty: u8,
    cnt_cute: u8,
    cnt_smart: u8,
    cnt_tough: u8,
    cnt_sheen: u8,
    #[deku(
        pad_bytes_after = "1",
        update = "(((self.pkrs & !0xF) | self.pkrs_days) | ((self.pkrs & 0xF) | self.pkrs_strain << 4))"
    )]
    pkrs: u8,
    #[deku(skip, default = "*pkrs & 0xF")]
    pkrs_days: u8,
    #[deku(skip, default = "*pkrs >> 4")]
    pkrs_strain: u8,
    // 0x33 unused padding
    ribbon_champion_kalos: Flag,
    ribbon_champion_g3_hoenn: Flag,
    ribbon_champion_sinnoh: Flag,
    ribbon_best_friends: Flag,
    ribbon_training: Flag,
    ribbon_battler_skillful: Flag,
    ribbon_battler_expert: Flag,
    ribbon_effort: Flag,

    ribbon_alert: Flag,
    ribbon_shock: Flag,
    ribbon_downcast: Flag,
    ribbon_careless: Flag,
    ribbon_relax: Flag,
    ribbon_snooze: Flag,
    ribbon_smile: Flag,
    ribbon_gorgeous: Flag,

    ribbon_royal: Flag,
    ribbon_gorgeous_royal: Flag,
    ribbon_artist: Flag,
    ribbon_footprint: Flag,
    ribbon_record: Flag,
    ribbon_legend: Flag,
    ribbon_country: Flag,
    ribbon_national: Flag,

    ribbon_earth: Flag,
    ribbon_world: Flag,
    ribbon_classic: Flag,
    ribbon_premier: Flag,
    ribbon_event: Flag,
    ribbon_birthday: Flag,
    ribbon_special: Flag,
    ribbon_souvenir: Flag,

    ribbon_wishing: Flag,
    ribbon_champion_battle: Flag,
    ribbon_champion_regional: Flag,
    ribbon_champion_national: Flag,
    ribbon_champion_world: Flag,
    has_contest_memory_ribbon: Flag,
    has_battle_memory_ribbon: Flag,
    ribbon_champion_g6_hoenn: Flag,

    ribbon_contest_star: Flag,
    ribbon_master_coolness: Flag,
    ribbon_master_beauty: Flag,
    ribbon_master_cuteness: Flag,
    ribbon_master_cleverness: Flag,
    ribbon_master_toughness: Flag,
    ribbon_champion_alola: Flag,
    ribbon_battle_royale: Flag,

    ribbon_battle_tree_great: Flag,
    ribbon_battle_tree_master: Flag,
    ribbon_champion_galar: Flag,
    ribbon_tower_master: Flag,
    ribbon_master_rank: Flag,
    ribbon_mark_lunchtime: Flag,
    ribbon_mark_sleepy_time: Flag,
    ribbon_mark_dusk: Flag,

    ribbon_mark_dawn: Flag,
    ribbon_mark_cloudy: Flag,
    ribbon_mark_rainy: Flag,
    ribbon_mark_stormy: Flag,
    ribbon_mark_snowy: Flag,
    ribbon_mark_blizzard: Flag,
    ribbon_mark_dry: Flag,
    ribbon_mark_sandstorm: Flag,
    ribbon_count_memory_contest: u8,
    #[deku(pad_bytes_after = "2")]
    ribbon_count_memory_battle: u8,
    // 0x3E padding
    // 0x3F padding

    // 0x40 Ribbon 1
    ribbon_mark_misty: Flag,
    ribbon_mark_destiny: Flag,
    ribbon_mark_fishing: Flag,
    ribbon_mark_curry: Flag,
    ribbon_mark_uncommon: Flag,
    ribbon_mark_rare: Flag,
    ribbon_mark_rowdy: Flag,
    ribbon_mark_absent_minded: Flag,

    ribbon_mark_jittery: Flag,
    ribbon_mark_excited: Flag,
    ribbon_mark_charismatic: Flag,
    ribbon_mark_calmness: Flag,
    ribbon_mark_intense: Flag,
    ribbon_mark_zoned_out: Flag,
    ribbon_mark_joyful: Flag,
    ribbon_mark_angry: Flag,

    ribbon_mark_smiley: Flag,
    ribbon_mark_teary: Flag,
    ribbon_mark_upbeat: Flag,
    ribbon_mark_peeved: Flag,
    ribbon_mark_intellectual: Flag,
    ribbon_mark_ferocious: Flag,
    ribbon_mark_crafty: Flag,
    ribbon_mark_scowling: Flag,

    ribbon_mark_kindly: Flag,
    ribbon_mark_flustered: Flag,
    ribbon_mark_pumped_up: Flag,
    ribbon_mark_zero_energy: Flag,
    ribbon_mark_prideful: Flag,
    ribbon_mark_unsure: Flag,
    ribbon_mark_humble: Flag,
    ribbon_mark_thorny: Flag,
    // 0x44 Ribbon 2
    ribbon_mark_vigor: Flag,
    ribbon_mark_slump: Flag,
    rib44_2: Flag,
    rib44_3: Flag,
    rib44_4: Flag,
    rib44_5: Flag,
    rib44_6: Flag,
    rib44_7: Flag,

    rib45_0: Flag,
    rib45_1: Flag,
    rib45_2: Flag,
    rib45_3: Flag,
    rib45_4: Flag,
    rib45_5: Flag,
    rib45_6: Flag,
    rib45_7: Flag,

    rib46_0: Flag,
    rib46_1: Flag,
    rib46_2: Flag,
    rib46_3: Flag,
    rib46_4: Flag,
    rib46_5: Flag,
    rib46_6: Flag,
    rib46_7: Flag,

    rib47_0: Flag,
    rib47_1: Flag,
    rib47_2: Flag,
    rib47_3: Flag,
    rib47_4: Flag,
    rib47_5: Flag,
    rib47_6: Flag,
    rib47_7: Flag,
    #[deku(pad_bytes_after = "4")]
    u48: u32,
    // 0x4C-0x4F unused
    height_scalar: u8,
    #[deku(pad_bytes_after = "6")]
    weight_scalar: u8,
    // 0x52-0x57 unused
    // Block B
    raw_nickname: [u16; NICK_LENGTH],
    raw_nickname_terminator: u16,
    #[deku(skip, default = "get_string7(raw_nickname)")]
    nickname: String,
    move1: u16,
    move2: u16,
    move3: u16,
    move4: u16,
    move1_pp: u8,
    move2_pp: u8,
    move3_pp: u8,
    move4_pp: u8,
    move_1_pp_ups: u8,
    move_2_pp_ups: u8,
    move_3_pp_ups: u8,
    move_4_pp_ups: u8,
    relearn_move1: u16,
    relearn_move2: u16,
    relearn_move3: u16,
    relearn_move4: u16,
    stat_hp_current: u16,
    iv32: u32,
    #[deku(skip, default = "((*iv32) & 0x1F) as u8")]
    iv_hp: u8,
    #[deku(skip, default = "((*iv32 >> 05) & 0x1F) as u8")]
    iv_atk: u8,
    #[deku(skip, default = "((*iv32 >> 10) & 0x1F) as u8")]
    iv_def: u8,
    #[deku(skip, default = "((*iv32 >> 15) & 0x1F) as u8")]
    iv_spe: u8,
    #[deku(skip, default = "((*iv32 >> 20) & 0x1F) as u8")]
    iv_spa: u8,
    #[deku(skip, default = "((*iv32 >> 25) & 0x1F) as u8")]
    iv_spd: u8,
    #[deku(skip, default = "(((*iv32 >> 30) & 1) == 1) as u8")]
    is_egg: u8,
    #[deku(skip, default = "(((*iv32 >> 31) & 1) == 1) as u8")]
    is_nicknamed: u8,
    #[deku(pad_bytes_after = "4")]
    dynamax_level: u8,
    status_condition: i32,
    #[deku(pad_bytes_after = "12")]
    unk98: i32,
    raw_ht_name: [u16; NICK_LENGTH],
    raw_ht_name_terminator: u16,
    #[deku(skip, default = "get_string7(raw_ht_name)")]
    ht_name: String,
    ht_gender: u8,
    ht_language: u8,
    current_handler: u8,
}

impl From<&[u8; 344]> for PK8Config {
    fn from(data: &[u8; SIZE_8PARTY]) -> Self {
        let mut array = *data;
        decrypt_if_encrypted8(&mut array);
        let (_rest, test_file) = PK8Config::from_bytes((array.as_ref(), 0)).unwrap();
        test_file
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::enums::{ability::Ability, gender::Gender, nature::Nature, species::Species};
    use core::convert::TryFrom;

    #[test]
    fn pk8_from_array_test() {
        let orbeetle_d = PK8Config::from(include_bytes!("util/tests/data/Orbeetle.pk8"));
        let orbeetle_e = PK8Config::from(include_bytes!("util/tests/data/Orbeetle.ek8"));
        let dracovish = PK8Config::from(include_bytes!("util/tests/data/Dracovish.pk8"));

        assert_eq!(true, orbeetle_d == orbeetle_e);
        assert_eq!(false, dracovish == orbeetle_d);
    }

    #[test]
    fn pk8_from_vec_test() {
        let orbeetle_e = PK8Config::from(include_bytes!("util/tests/data/Orbeetle.ek8"));
        let orbeetle_d =
            PK8Config::try_from(&*include_bytes!("util/tests/data/Orbeetle.pk8")).unwrap();

        assert_eq!(true, orbeetle_e == orbeetle_d);
    }

    #[test]
    fn pk8_calc_checksum_test() {
        let orbeetle = PK8Config::from(include_bytes!("util/tests/data/Orbeetle.pk8"));
        let dracovish = PK8Config::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0x4E8E, orbeetle.checksum);
        assert_eq!(0x5D57, dracovish.checksum);
    }

    #[test]
    fn pk8_get_test_1() {
        let dracovish = PK8Config::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(0xAC731A09, dracovish.encryption_constant);
        assert_eq!(0x0, dracovish.sanity);
        assert_eq!(0x5D57, dracovish.checksum);
        assert_eq!(882, dracovish.species as i32);
        assert_eq!(Species::Dracovish, dracovish.species);
        assert_eq!(268, dracovish.held_item);
        assert_eq!(30756, dracovish.tid);
        assert_eq!(45312, dracovish.sid);
        assert_eq!(1250000, dracovish.exp);
        assert_eq!(11, dracovish.ability as u16);
        assert_eq!(Ability::WaterAbsorb, dracovish.ability);
        assert_eq!(1, dracovish.ability_number);
        assert_eq!(0, dracovish.favourite);
        assert_eq!(0, dracovish.can_gigantamax);
        assert_eq!(0, dracovish.mark_value);
        assert_eq!(0xC730F59, dracovish.pid);
        assert_eq!(16, dracovish.nature as u8);
        assert_eq!(Nature::Mild, dracovish.nature);
        assert_eq!(16, dracovish.stat_nature as u8);
        assert_eq!(Nature::Mild, dracovish.stat_nature);
        assert_eq!(0, dracovish.fateful_encounter);
        assert_eq!(0, dracovish.flag2);
        assert_eq!(Gender::Genderless, dracovish.gender);
        assert_eq!(0, dracovish.alt_form);
        assert_eq!(4, dracovish.ev_hp);
        assert_eq!(252, dracovish.ev_atk);
        assert_eq!(0, dracovish.ev_def);
        assert_eq!(252, dracovish.ev_spe);
        assert_eq!(0, dracovish.ev_spa);
        assert_eq!(0, dracovish.ev_spd);
    }

    #[test]
    fn pk8_get_test_2() {
        let dracovish = PK8Config::from(include_bytes!("util/tests/data/Dracovish.pk8"));
        assert_eq!(88, dracovish.height_scalar);
        assert_eq!(33, dracovish.weight_scalar);
        assert_eq!(35, dracovish.move1_pp);
        assert_eq!(25, dracovish.move2_pp);
        assert_eq!(10, dracovish.move3_pp);
        assert_eq!(16, dracovish.move4_pp);
        assert_eq!(0, dracovish.move_1_pp_ups);
        assert_eq!(0, dracovish.move_2_pp_ups);
        assert_eq!(0, dracovish.move_3_pp_ups);
        assert_eq!(3, dracovish.move_4_pp_ups);
        assert_eq!(31, dracovish.iv_hp);
        assert_eq!(31, dracovish.iv_atk);
        assert_eq!(31, dracovish.iv_def);
        assert_eq!(23, dracovish.iv_spa);
        assert_eq!(2, dracovish.iv_spd);
        assert_eq!(4, dracovish.iv_spe);
        assert_eq!(10, dracovish.dynamax_level);
    }

    // #[test]
    // fn pk8_set_test() {
    //     let mut dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
    //     assert_eq!(0xAC731A09, dracovish.EncryptionConstant));
    //     set!(dracovish.EncryptionConstant, 0xDEADBEEF);
    //     assert_eq!(0xDEADBEEF, dracovish.EncryptionConstant));
    // }
}
