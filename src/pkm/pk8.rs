#![allow(non_snake_case)]
use alloc::{format, string::String, vec::Vec};
use core::convert::TryFrom;
use deku::prelude::*;

use crate::game::enums::{
    ability::Ability, ball::Ball, flag::Flag, gender::Gender, language_id::LanguageID, moves::Move,
    nature::Nature, species::Species,
};
use crate::pkm::strings::string_converter::get_string7;
use crate::pkm::util::pokecrypto::{decrypt_if_encrypted8, get_chk, SIZE_8PARTY, SIZE_8STORED};

pub const FORMAT: u32 = 8;

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
    can_gigantamax: bool,
    #[deku(bits = 1)]
    favourite: bool,
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
    #[deku(bits = 2)]
    gender: Gender,
    #[deku(bits = 1)]
    flag2: Flag,
    #[deku(bits = 1)]
    fateful_encounter: bool,
    // 0x23 alignment unused
    #[deku(pad_bytes_before = "1")]
    form: u16,
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
    #[deku(bits = 1)]
    ribbon_champion_kalos: Flag,
    #[deku(bits = 1)]
    ribbon_champion_g3_hoenn: Flag,
    #[deku(bits = 1)]
    ribbon_champion_sinnoh: Flag,
    #[deku(bits = 1)]
    ribbon_best_friends: Flag,
    #[deku(bits = 1)]
    ribbon_training: Flag,
    #[deku(bits = 1)]
    ribbon_battler_skillful: Flag,
    #[deku(bits = 1)]
    ribbon_battler_expert: Flag,
    #[deku(bits = 1)]
    ribbon_effort: Flag,

    #[deku(bits = 1)]
    ribbon_alert: Flag,
    #[deku(bits = 1)]
    ribbon_shock: Flag,
    #[deku(bits = 1)]
    ribbon_downcast: Flag,
    #[deku(bits = 1)]
    ribbon_careless: Flag,
    #[deku(bits = 1)]
    ribbon_relax: Flag,
    #[deku(bits = 1)]
    ribbon_snooze: Flag,
    #[deku(bits = 1)]
    ribbon_smile: Flag,
    #[deku(bits = 1)]
    ribbon_gorgeous: Flag,

    #[deku(bits = 1)]
    ribbon_royal: Flag,
    #[deku(bits = 1)]
    ribbon_gorgeous_royal: Flag,
    #[deku(bits = 1)]
    ribbon_artist: Flag,
    #[deku(bits = 1)]
    ribbon_footprint: Flag,
    #[deku(bits = 1)]
    ribbon_record: Flag,
    #[deku(bits = 1)]
    ribbon_legend: Flag,
    #[deku(bits = 1)]
    ribbon_country: Flag,
    #[deku(bits = 1)]
    ribbon_national: Flag,

    #[deku(bits = 1)]
    ribbon_earth: Flag,
    #[deku(bits = 1)]
    ribbon_world: Flag,
    #[deku(bits = 1)]
    ribbon_classic: Flag,
    #[deku(bits = 1)]
    ribbon_premier: Flag,
    #[deku(bits = 1)]
    ribbon_event: Flag,
    #[deku(bits = 1)]
    ribbon_birthday: Flag,
    #[deku(bits = 1)]
    ribbon_special: Flag,
    #[deku(bits = 1)]
    ribbon_souvenir: Flag,

    #[deku(bits = 1)]
    ribbon_wishing: Flag,
    #[deku(bits = 1)]
    ribbon_champion_battle: Flag,
    #[deku(bits = 1)]
    ribbon_champion_regional: Flag,
    #[deku(bits = 1)]
    ribbon_champion_national: Flag,
    #[deku(bits = 1)]
    ribbon_champion_world: Flag,
    #[deku(bits = 1)]
    has_contest_memory_ribbon: Flag,
    #[deku(bits = 1)]
    has_battle_memory_ribbon: Flag,
    #[deku(bits = 1)]
    ribbon_champion_g6_hoenn: Flag,

    #[deku(bits = 1)]
    ribbon_contest_star: Flag,
    #[deku(bits = 1)]
    ribbon_master_coolness: Flag,
    #[deku(bits = 1)]
    ribbon_master_beauty: Flag,
    #[deku(bits = 1)]
    ribbon_master_cuteness: Flag,
    #[deku(bits = 1)]
    ribbon_master_cleverness: Flag,
    #[deku(bits = 1)]
    ribbon_master_toughness: Flag,
    #[deku(bits = 1)]
    ribbon_champion_alola: Flag,
    #[deku(bits = 1)]
    ribbon_battle_royale: Flag,

    #[deku(bits = 1)]
    ribbon_battle_tree_great: Flag,
    #[deku(bits = 1)]
    ribbon_battle_tree_master: Flag,
    #[deku(bits = 1)]
    ribbon_champion_galar: Flag,
    #[deku(bits = 1)]
    ribbon_tower_master: Flag,
    #[deku(bits = 1)]
    ribbon_master_rank: Flag,
    #[deku(bits = 1)]
    ribbon_mark_lunchtime: Flag,
    #[deku(bits = 1)]
    ribbon_mark_sleepy_time: Flag,
    #[deku(bits = 1)]
    ribbon_mark_dusk: Flag,

    #[deku(bits = 1)]
    ribbon_mark_dawn: Flag,
    #[deku(bits = 1)]
    ribbon_mark_cloudy: Flag,
    #[deku(bits = 1)]
    ribbon_mark_rainy: Flag,
    #[deku(bits = 1)]
    ribbon_mark_stormy: Flag,
    #[deku(bits = 1)]
    ribbon_mark_snowy: Flag,
    #[deku(bits = 1)]
    ribbon_mark_blizzard: Flag,
    #[deku(bits = 1)]
    ribbon_mark_dry: Flag,
    #[deku(bits = 1)]
    ribbon_mark_sandstorm: Flag,
    ribbon_count_memory_contest: u8,
    #[deku(pad_bytes_after = "2")]
    ribbon_count_memory_battle: u8,
    // 0x3E padding
    // 0x3F padding

    // 0x40 Ribbon 1
    #[deku(bits = 1)]
    ribbon_mark_misty: Flag,
    #[deku(bits = 1)]
    ribbon_mark_destiny: Flag,
    #[deku(bits = 1)]
    ribbon_mark_fishing: Flag,
    #[deku(bits = 1)]
    ribbon_mark_curry: Flag,
    #[deku(bits = 1)]
    ribbon_mark_uncommon: Flag,
    #[deku(bits = 1)]
    ribbon_mark_rare: Flag,
    #[deku(bits = 1)]
    ribbon_mark_rowdy: Flag,
    #[deku(bits = 1)]
    ribbon_mark_absent_minded: Flag,

    #[deku(bits = 1)]
    ribbon_mark_jittery: Flag,
    #[deku(bits = 1)]
    ribbon_mark_excited: Flag,
    #[deku(bits = 1)]
    ribbon_mark_charismatic: Flag,
    #[deku(bits = 1)]
    ribbon_mark_calmness: Flag,
    #[deku(bits = 1)]
    ribbon_mark_intense: Flag,
    #[deku(bits = 1)]
    ribbon_mark_zoned_out: Flag,
    #[deku(bits = 1)]
    ribbon_mark_joyful: Flag,
    #[deku(bits = 1)]
    ribbon_mark_angry: Flag,

    #[deku(bits = 1)]
    ribbon_mark_smiley: Flag,
    #[deku(bits = 1)]
    ribbon_mark_teary: Flag,
    #[deku(bits = 1)]
    ribbon_mark_upbeat: Flag,
    #[deku(bits = 1)]
    ribbon_mark_peeved: Flag,
    #[deku(bits = 1)]
    ribbon_mark_intellectual: Flag,
    #[deku(bits = 1)]
    ribbon_mark_ferocious: Flag,
    #[deku(bits = 1)]
    ribbon_mark_crafty: Flag,
    #[deku(bits = 1)]
    ribbon_mark_scowling: Flag,

    #[deku(bits = 1)]
    ribbon_mark_kindly: Flag,
    #[deku(bits = 1)]
    ribbon_mark_flustered: Flag,
    #[deku(bits = 1)]
    ribbon_mark_pumped_up: Flag,
    #[deku(bits = 1)]
    ribbon_mark_zero_energy: Flag,
    #[deku(bits = 1)]
    ribbon_mark_prideful: Flag,
    #[deku(bits = 1)]
    ribbon_mark_unsure: Flag,
    #[deku(bits = 1)]
    ribbon_mark_humble: Flag,
    #[deku(bits = 1)]
    ribbon_mark_thorny: Flag,
    // 0x44 Ribbon 2
    #[deku(bits = 1)]
    ribbon_mark_vigor: Flag,
    #[deku(bits = 1)]
    ribbon_mark_slump: Flag,
    #[deku(bits = 1)]
    rib44_2: Flag,
    #[deku(bits = 1)]
    rib44_3: Flag,
    #[deku(bits = 1)]
    rib44_4: Flag,
    #[deku(bits = 1)]
    rib44_5: Flag,
    #[deku(bits = 1)]
    rib44_6: Flag,
    #[deku(bits = 1)]
    rib44_7: Flag,

    #[deku(bits = 1)]
    rib45_0: Flag,
    #[deku(bits = 1)]
    rib45_1: Flag,
    #[deku(bits = 1)]
    rib45_2: Flag,
    #[deku(bits = 1)]
    rib45_3: Flag,
    #[deku(bits = 1)]
    rib45_4: Flag,
    #[deku(bits = 1)]
    rib45_5: Flag,
    #[deku(bits = 1)]
    rib45_6: Flag,
    #[deku(bits = 1)]
    rib45_7: Flag,

    #[deku(bits = 1)]
    rib46_0: Flag,
    #[deku(bits = 1)]
    rib46_1: Flag,
    #[deku(bits = 1)]
    rib46_2: Flag,
    #[deku(bits = 1)]
    rib46_3: Flag,
    #[deku(bits = 1)]
    rib46_4: Flag,
    #[deku(bits = 1)]
    rib46_5: Flag,
    #[deku(bits = 1)]
    rib46_6: Flag,
    #[deku(bits = 1)]
    rib46_7: Flag,

    #[deku(bits = 1)]
    rib47_0: Flag,
    #[deku(bits = 1)]
    rib47_1: Flag,
    #[deku(bits = 1)]
    rib47_2: Flag,
    #[deku(bits = 1)]
    rib47_3: Flag,
    #[deku(bits = 1)]
    rib47_4: Flag,
    #[deku(bits = 1)]
    rib47_5: Flag,
    #[deku(bits = 1)]
    rib47_6: Flag,
    #[deku(bits = 1)]
    rib47_7: Flag,
    #[deku(pad_bytes_after = "4")]
    u48: u32,
    // 0x4C-0x4F unused
    height_scalar: u8,
    #[deku(pad_bytes_after = "6")]
    weight_scalar: u8,
    // 0x52-0x57 unused
    // Block B
    raw_nickname: [u16; NICK_LENGTH + 1],
    #[deku(skip, default = "get_string7(raw_nickname)")]
    nickname: String,
    move1: Move,
    move2: Move,
    move3: Move,
    move4: Move,
    move1_pp: u8,
    move2_pp: u8,
    move3_pp: u8,
    move4_pp: u8,
    move_1_pp_ups: u8,
    move_2_pp_ups: u8,
    move_3_pp_ups: u8,
    move_4_pp_ups: u8,
    relearn_move1: Move,
    relearn_move2: Move,
    relearn_move3: Move,
    relearn_move4: Move,
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
    #[deku(skip, default = "((*iv32 >> 30) & 1) == 1")]
    is_egg: bool,
    #[deku(skip, default = "((*iv32 >> 31) & 1) == 1")]
    is_nicknamed: bool,
    #[deku(pad_bytes_after = "3")]
    dynamax_level: u8,
    status_condition: i32,
    #[deku(pad_bytes_after = "12")]
    unk98: i32,
    // Block C
    raw_ht_name: [u16; NICK_LENGTH + 1],
    #[deku(skip, default = "get_string7(raw_ht_name)")]
    ht_name: String,
    ht_gender: Gender,
    ht_language: LanguageID,
    #[deku(pad_bytes_after = "1")]
    current_handler: u8,
    // 0xC5 unused (alignment)
    ht_trainer_id: u16,
    ht_friendship: u8,
    ht_intensity: u8,
    ht_memory: u8,
    ht_feeling: u8,
    #[deku(pad_bytes_after = "14")]
    ht_text_var: u16,
    // 0xCE-0xDB unused
    fullness: u8,
    enjoyment: u8,
    version: u8,
    #[deku(pad_bytes_after = "2")]
    battle_version: u8,
    // region: u8,
    // console_region: u8,
    language: LanguageID,
    unk_e3: u8,
    form_argument: u32,
    #[deku(pad_bytes_after = "15")]
    affixed_ribbon: i8,
    // 0xE9-0xF7 unused
    // Block D
    raw_ot_name: [u16; NICK_LENGTH + 1],
    #[deku(skip, default = "get_string7(raw_ot_name)")]
    ot_name: String,
    ot_friendship: u8,
    ot_intensity: u8,
    #[deku(pad_bytes_after = "1")]
    ot_memory: u8,
    // 0x115 unused align
    ot_text_var: u16,
    ot_feeling: u8,
    egg_year: u8,
    egg_month: u8,
    egg_day: u8,
    met_year: u8,
    met_month: u8,
    #[deku(pad_bytes_after = "1")]
    met_day: u8,
    // 0x11F unused align
    egg_location: u16,
    met_location: u16,
    ball: Ball,
    #[deku(bits = "1")]
    ot_gender: Gender,
    #[deku(bits = "7")]
    met_level: u8,
    #[deku(bits = "1", pad_bits_before = "2")]
    ht_spe: Flag,
    #[deku(bits = "1")]
    ht_spd: Flag,
    #[deku(bits = "1")]
    ht_spa: Flag,
    #[deku(bits = "1")]
    ht_def: Flag,
    #[deku(bits = "1")]
    ht_atk: Flag,
    #[deku(bits = "1")]
    ht_hp: Flag,
    raw_move_record: [u8; 14],
    #[deku(pad_bytes_after = "11")]
    tracker: u64,
    #[deku(pad_bytes_after = "1")]
    stat_level: u8,
    stat_hp_max: u16,
    stat_atk: u16,
    stat_def: u16,
    stat_spe: u16,
    stat_spa: u16,
    stat_spd: u16,
    dynamax_type: u16,
}

impl From<&[u8; 344]> for PK8Config {
    fn from(data: &[u8; SIZE_8PARTY]) -> Self {
        let mut array = *data;
        decrypt_if_encrypted8(&mut array);
        let (_rest, file) = PK8Config::from_bytes((array.as_ref(), 0)).unwrap();
        file
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
        assert_eq!(false, dracovish.favourite);
        assert_eq!(false, dracovish.can_gigantamax);
        assert_eq!(0, dracovish.mark_value);
        assert_eq!(0xC730F59, dracovish.pid);
        assert_eq!(16, dracovish.nature as u8);
        assert_eq!(Nature::Mild, dracovish.nature);
        assert_eq!(16, dracovish.stat_nature as u8);
        assert_eq!(Nature::Mild, dracovish.stat_nature);
        assert_eq!(false, dracovish.fateful_encounter);
        assert_eq!(Flag::Unset, dracovish.flag2);
        assert_eq!(Gender::Genderless, dracovish.gender);
        assert_eq!(0, dracovish.form);
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
