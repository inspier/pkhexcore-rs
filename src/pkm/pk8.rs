#![allow(non_snake_case)]
use alloc::{format, string::String, vec::Vec};
use core::convert::TryFrom;
use deku::prelude::*;

use crate::{
    game::enums::{
        ability::Ability, ball::Ball, flag::Flag, gender::Gender, language_id::LanguageID,
        moves::Move, nature::Nature, species::Species,
    },
    pkm::{
        strings::string_converter::{get_string7, set_string7b},
        util::pokecrypto::{decrypt_if_encrypted8, get_chk, SIZE_8PARTY, SIZE_8STORED},
    },
    util::{
        custom_read_write::{read, write},
        packutil::pack_u32,
    },
};

pub const FORMAT: u32 = 8;

pub const MAX_IV: i32 = 31;
pub const MAX_EV: i32 = 252;
pub const OT_LENGTH: usize = 12;
pub const NICK_LENGTH: usize = 12;

// TODO: PersonalInfo

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
#[repr(C)]
pub struct PK8 {
    pub encryption_constant: u32,
    pub sanity: u16,
    #[deku(
        update = "get_chk::<SIZE_8PARTY>(&<[u8; SIZE_8PARTY]>::try_from(self.to_bytes().unwrap()).unwrap(), SIZE_8STORED)"
    )]
    pub checksum: u16,

    // Block A
    pub species: Species,
    pub held_item: u16,
    pub tid: u16,
    pub sid: u16,
    pub exp: u32,
    #[deku(pad_bits_after = "3")]
    pub ability: Ability,
    #[deku(bits = 1)]
    pub can_gigantamax: bool,
    #[deku(bits = 1)]
    pub favourite: bool,
    #[deku(bits = 3)]
    pub ability_number: u8,
    // 0x17 alignment unused
    #[deku(pad_bytes_before = "1")]
    pub mark_value: u16,
    // 0x1A alignment unused
    // 0x1B alignment unused
    #[deku(pad_bytes_before = "2")]
    pub pid: u32,
    pub nature: Nature,
    #[deku(pad_bits_after = "4")]
    pub stat_nature: Nature,
    #[deku(bits = 2)]
    pub gender: Gender,
    #[deku(bits = 1)]
    pub flag2: Flag,
    #[deku(bits = 1)]
    pub fateful_encounter: bool,
    // 0x23 alignment unused
    #[deku(pad_bytes_before = "1")]
    pub form: u16,
    pub ev_hp: u8,
    pub ev_atk: u8,
    pub ev_def: u8,
    pub ev_spe: u8,
    pub ev_spa: u8,
    pub ev_spd: u8,
    pub cnt_cool: u8,
    pub cnt_beauty: u8,
    pub cnt_cute: u8,
    pub cnt_smart: u8,
    pub cnt_tough: u8,
    pub cnt_sheen: u8,
    #[deku(bits = 4)]
    pub pkrs_strain: u8,
    #[deku(bits = 4, pad_bytes_after = "1")]
    pub pkrs_days: u8,
    // 0x33 unused padding
    #[deku(bits = 1)]
    pub ribbon_champion_kalos: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_g3_hoenn: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_sinnoh: Flag,
    #[deku(bits = 1)]
    pub ribbon_best_friends: Flag,
    #[deku(bits = 1)]
    pub ribbon_training: Flag,
    #[deku(bits = 1)]
    pub ribbon_battler_skillful: Flag,
    #[deku(bits = 1)]
    pub ribbon_battler_expert: Flag,
    #[deku(bits = 1)]
    pub ribbon_effort: Flag,

    #[deku(bits = 1)]
    pub ribbon_alert: Flag,
    #[deku(bits = 1)]
    pub ribbon_shock: Flag,
    #[deku(bits = 1)]
    pub ribbon_downcast: Flag,
    #[deku(bits = 1)]
    pub ribbon_careless: Flag,
    #[deku(bits = 1)]
    pub ribbon_relax: Flag,
    #[deku(bits = 1)]
    pub ribbon_snooze: Flag,
    #[deku(bits = 1)]
    pub ribbon_smile: Flag,
    #[deku(bits = 1)]
    pub ribbon_gorgeous: Flag,

    #[deku(bits = 1)]
    pub ribbon_royal: Flag,
    #[deku(bits = 1)]
    pub ribbon_gorgeous_royal: Flag,
    #[deku(bits = 1)]
    pub ribbon_artist: Flag,
    #[deku(bits = 1)]
    pub ribbon_footprint: Flag,
    #[deku(bits = 1)]
    pub ribbon_record: Flag,
    #[deku(bits = 1)]
    pub ribbon_legend: Flag,
    #[deku(bits = 1)]
    pub ribbon_country: Flag,
    #[deku(bits = 1)]
    pub ribbon_national: Flag,

    #[deku(bits = 1)]
    pub ribbon_earth: Flag,
    #[deku(bits = 1)]
    pub ribbon_world: Flag,
    #[deku(bits = 1)]
    pub ribbon_classic: Flag,
    #[deku(bits = 1)]
    pub ribbon_premier: Flag,
    #[deku(bits = 1)]
    pub ribbon_event: Flag,
    #[deku(bits = 1)]
    pub ribbon_birthday: Flag,
    #[deku(bits = 1)]
    pub ribbon_special: Flag,
    #[deku(bits = 1)]
    pub ribbon_souvenir: Flag,

    #[deku(bits = 1)]
    pub ribbon_wishing: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_battle: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_regional: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_national: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_world: Flag,
    #[deku(bits = 1)]
    pub has_contest_memory_ribbon: Flag,
    #[deku(bits = 1)]
    pub has_battle_memory_ribbon: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_g6_hoenn: Flag,

    #[deku(bits = 1)]
    pub ribbon_contest_star: Flag,
    #[deku(bits = 1)]
    pub ribbon_master_coolness: Flag,
    #[deku(bits = 1)]
    pub ribbon_master_beauty: Flag,
    #[deku(bits = 1)]
    pub ribbon_master_cuteness: Flag,
    #[deku(bits = 1)]
    pub ribbon_master_cleverness: Flag,
    #[deku(bits = 1)]
    pub ribbon_master_toughness: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_alola: Flag,
    #[deku(bits = 1)]
    pub ribbon_battle_royale: Flag,

    #[deku(bits = 1)]
    pub ribbon_battle_tree_great: Flag,
    #[deku(bits = 1)]
    pub ribbon_battle_tree_master: Flag,
    #[deku(bits = 1)]
    pub ribbon_champion_galar: Flag,
    #[deku(bits = 1)]
    pub ribbon_tower_master: Flag,
    #[deku(bits = 1)]
    pub ribbon_master_rank: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_lunchtime: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_sleepy_time: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_dusk: Flag,

    #[deku(bits = 1)]
    pub ribbon_mark_dawn: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_cloudy: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_rainy: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_stormy: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_snowy: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_blizzard: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_dry: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_sandstorm: Flag,
    pub ribbon_count_memory_contest: u8,
    #[deku(pad_bytes_after = "2")]
    pub ribbon_count_memory_battle: u8,
    // 0x3E padding
    // 0x3F padding

    // 0x40 Ribbon 1
    #[deku(bits = 1)]
    pub ribbon_mark_misty: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_destiny: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_fishing: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_curry: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_uncommon: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_rare: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_rowdy: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_absent_minded: Flag,

    #[deku(bits = 1)]
    pub ribbon_mark_jittery: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_excited: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_charismatic: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_calmness: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_intense: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_zoned_out: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_joyful: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_angry: Flag,

    #[deku(bits = 1)]
    pub ribbon_mark_smiley: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_teary: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_upbeat: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_peeved: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_intellectual: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_ferocious: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_crafty: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_scowling: Flag,

    #[deku(bits = 1)]
    pub ribbon_mark_kindly: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_flustered: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_pumped_up: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_zero_energy: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_prideful: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_unsure: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_humble: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_thorny: Flag,
    // 0x44 Ribbon 2
    #[deku(bits = 1)]
    pub ribbon_mark_vigor: Flag,
    #[deku(bits = 1)]
    pub ribbon_mark_slump: Flag,
    #[deku(bits = 1)]
    pub rib44_2: Flag,
    #[deku(bits = 1)]
    pub rib44_3: Flag,
    #[deku(bits = 1)]
    pub rib44_4: Flag,
    #[deku(bits = 1)]
    pub rib44_5: Flag,
    #[deku(bits = 1)]
    pub rib44_6: Flag,
    #[deku(bits = 1)]
    pub rib44_7: Flag,

    #[deku(bits = 1)]
    pub rib45_0: Flag,
    #[deku(bits = 1)]
    pub rib45_1: Flag,
    #[deku(bits = 1)]
    pub rib45_2: Flag,
    #[deku(bits = 1)]
    pub rib45_3: Flag,
    #[deku(bits = 1)]
    pub rib45_4: Flag,
    #[deku(bits = 1)]
    pub rib45_5: Flag,
    #[deku(bits = 1)]
    pub rib45_6: Flag,
    #[deku(bits = 1)]
    pub rib45_7: Flag,

    #[deku(bits = 1)]
    pub rib46_0: Flag,
    #[deku(bits = 1)]
    pub rib46_1: Flag,
    #[deku(bits = 1)]
    pub rib46_2: Flag,
    #[deku(bits = 1)]
    pub rib46_3: Flag,
    #[deku(bits = 1)]
    pub rib46_4: Flag,
    #[deku(bits = 1)]
    pub rib46_5: Flag,
    #[deku(bits = 1)]
    pub rib46_6: Flag,
    #[deku(bits = 1)]
    pub rib46_7: Flag,

    #[deku(bits = 1)]
    pub rib47_0: Flag,
    #[deku(bits = 1)]
    pub rib47_1: Flag,
    #[deku(bits = 1)]
    pub rib47_2: Flag,
    #[deku(bits = 1)]
    pub rib47_3: Flag,
    #[deku(bits = 1)]
    pub rib47_4: Flag,
    #[deku(bits = 1)]
    pub rib47_5: Flag,
    #[deku(bits = 1)]
    pub rib47_6: Flag,
    #[deku(bits = 1)]
    pub rib47_7: Flag,
    #[deku(pad_bytes_after = "4")]
    pub u48: u32,
    // 0x4C-0x4F unused
    pub height_scalar: u8,
    #[deku(pad_bytes_after = "6")]
    pub weight_scalar: u8,
    // 0x52-0x57 unused
    // Block B
    #[deku(
        reader = "read::read_string_custom(deku::rest, NICK_LENGTH + 1, Self::get_string)",
        writer = "write::write_string_custom(deku::output, self.set_string(&self.nickname, NICK_LENGTH))"
    )]
    pub nickname: String,
    pub move1: Move,
    pub move2: Move,
    pub move3: Move,
    pub move4: Move,
    pub move1_pp: u8,
    pub move2_pp: u8,
    pub move3_pp: u8,
    pub move4_pp: u8,
    pub move_1_pp_ups: u8,
    pub move_2_pp_ups: u8,
    pub move_3_pp_ups: u8,
    pub move_4_pp_ups: u8,
    pub relearn_move1: Move,
    pub relearn_move2: Move,
    pub relearn_move3: Move,
    pub relearn_move4: Move,
    pub stat_hp_current: u16,
    #[deku(
        update = "pack_u32(&[(self.iv_hp, 5), (self.iv_atk, 5), (self.iv_def, 5), (self.iv_spe, 5), (self.iv_spa, 5), (self.iv_spd, 5), (self.is_egg as u8, 1), (self.is_nicknamed as u8, 1)])"
    )]
    pub iv32: u32,
    #[deku(skip, default = "((*iv32) & 0x1F) as u8")]
    pub iv_hp: u8,
    #[deku(skip, default = "((*iv32 >> 05) & 0x1F) as u8")]
    pub iv_atk: u8,
    #[deku(skip, default = "((*iv32 >> 10) & 0x1F) as u8")]
    pub iv_def: u8,
    #[deku(skip, default = "((*iv32 >> 15) & 0x1F) as u8")]
    pub iv_spe: u8,
    #[deku(skip, default = "((*iv32 >> 20) & 0x1F) as u8")]
    pub iv_spa: u8,
    #[deku(skip, default = "((*iv32 >> 25) & 0x1F) as u8")]
    pub iv_spd: u8,
    #[deku(skip, default = "((*iv32 >> 30) & 1) == 1")]
    pub is_egg: bool,
    #[deku(skip, default = "((*iv32 >> 31) & 1) == 1")]
    pub is_nicknamed: bool,
    #[deku(pad_bytes_after = "3")]
    pub dynamax_level: u8,
    pub status_condition: i32,
    #[deku(pad_bytes_after = "12")]
    pub unk98: i32,
    // Block C
    #[deku(
        reader = "read::read_string_custom(deku::rest, OT_LENGTH + 1, Self::get_string)",
        writer = "write::write_string_custom(deku::output, self.set_string(&self.ht_name, OT_LENGTH))"
    )]
    pub ht_name: String,
    pub ht_gender: Gender,
    pub ht_language: LanguageID,
    #[deku(pad_bytes_after = "1")]
    pub current_handler: u8,
    // 0xC5 unused (alignment)
    pub ht_trainer_id: u16,
    pub ht_friendship: u8,
    pub ht_intensity: u8,
    pub ht_memory: u8,
    pub ht_feeling: u8,
    #[deku(pad_bytes_after = "14")]
    pub ht_text_var: u16,
    // 0xCE-0xDB unused
    pub fullness: u8,
    pub enjoyment: u8,
    pub version: u8,
    #[deku(pad_bytes_after = "2")]
    pub battle_version: u8,
    // region: u8,
    // console_region: u8,
    pub language: LanguageID,
    pub unk_e3: u8,
    pub form_argument: u32,
    #[deku(pad_bytes_after = "15")]
    pub affixed_ribbon: i8,
    // 0xE9-0xF7 unused
    // Block D
    #[deku(
        reader = "read::read_string_custom(deku::rest, OT_LENGTH + 1, Self::get_string)",
        writer = "write::write_string_custom(deku::output, self.set_string(&self.ot_name, OT_LENGTH))"
    )]
    pub ot_name: String,
    pub ot_friendship: u8,
    pub ot_intensity: u8,
    #[deku(pad_bytes_after = "1")]
    pub ot_memory: u8,
    // 0x115 unused align
    pub ot_text_var: u16,
    pub ot_feeling: u8,
    pub egg_year: u8,
    pub egg_month: u8,
    pub egg_day: u8,
    pub met_year: u8,
    pub met_month: u8,
    #[deku(pad_bytes_after = "1")]
    pub met_day: u8,
    // 0x11F unused align
    pub egg_location: u16,
    pub met_location: u16,
    pub ball: Ball,
    #[deku(bits = "1")]
    pub ot_gender: Gender,
    #[deku(bits = "7")]
    pub met_level: u8,
    #[deku(bits = "1", pad_bits_before = "2")]
    pub ht_spe: Flag,
    #[deku(bits = "1")]
    pub ht_spd: Flag,
    #[deku(bits = "1")]
    pub ht_spa: Flag,
    #[deku(bits = "1")]
    pub ht_def: Flag,
    #[deku(bits = "1")]
    pub ht_atk: Flag,
    #[deku(bits = "1")]
    pub ht_hp: Flag,
    pub raw_move_record: [u8; 14],
    #[deku(pad_bytes_after = "11")]
    pub tracker: u64,
    #[deku(pad_bytes_after = "1")]
    pub stat_level: u8,
    pub stat_hp_max: u16,
    pub stat_atk: u16,
    pub stat_def: u16,
    pub stat_spe: u16,
    pub stat_spa: u16,
    pub stat_spd: u16,
    pub dynamax_type: u16,
}

impl PK8 {
    fn get_string(data: &[u16]) -> String {
        get_string7(data)
    }

    fn set_string<S: AsRef<str>>(&self, data: S, max_length: usize) -> Vec<u16> {
        set_string7b(data.as_ref(), max_length, self.language, 0, 0, false)
    }

    pub fn as_bytes(&mut self) -> [u8; SIZE_8PARTY] {
        // Note: Double updated needed to make sure changes to checksum propagate.
        let _ = self.update();
        let _ = self.update();
        <[u8; SIZE_8PARTY]>::try_from(self.to_bytes().unwrap()).unwrap()
    }
}

impl From<&[u8; SIZE_8PARTY]> for PK8 {
    fn from(data: &[u8; SIZE_8PARTY]) -> Self {
        let mut array = *data;
        decrypt_if_encrypted8(&mut array);
        let (_rest, file) = PK8::from_bytes((array.as_ref(), 0)).unwrap();
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
        assert_eq!(0x4E8E, orbeetle.checksum);
        assert_eq!(0x5D57, dracovish.checksum);
    }

    #[test]
    fn pk8_get_test_1() {
        let dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
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
        let dracovish = PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
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
    //     let mut dracovish =
    // PK8::from(include_bytes!("util/tests/data/Dracovish.pk8"));
    //     assert_eq!(0xAC731A09, dracovish.EncryptionConstant));
    //     set!(dracovish.EncryptionConstant, 0xDEADBEEF);
    //     assert_eq!(0xDEADBEEF, dracovish.EncryptionConstant));
    // }
}
