#![allow(non_snake_case)]
use core::fmt;
use deku::prelude::*;

use crate::game::enums::{ability::Ability, gender::Gender, nature::Nature, species::Species};
use crate::pkm::util::pokecrypto::{decrypt_if_encrypted8, SIZE_8PARTY, SIZE_8STORED};
use crate::util::{bitconverter, flagutil::Flag};

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

#[derive(PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PK8Config {
    encryption_constant: u32,
    sanity: u16,
    checksum: u16,

    // Block A
    species: Species,
    held_item: u16,
    tid: u16,
    sid: u16,
    exp: u32,
    ability: Ability,
    #[deku(bits = 3)]
    _bit_padding0: u8,
    #[deku(bits = 1)]
    can_gigantamax: u8,
    #[deku(bits = 1)]
    favourite: u8,
    #[deku(bits = 3)]
    ability_number: u8,
    // 0x17 alignment unused
    _alignment_17: u8,
    mark_value: u16,
    // 0x1A alignment unused
    // 0x1B alignment unused
    _alignment_1A_1B: u16,
    pid: u32,
    nature: Nature,
    stat_nature: Nature,
    #[deku(bits = 4)]
    _bit_padding1: u8,
    gender: Gender,
    #[deku(bits = 1)]
    flag2: u8,
    #[deku(bits = 1)]
    fateful_encounter: u8,
    // 0x23 alignment unused
    _alignment_23: u8,
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
        update = "(((self.pkrs & !0xF) | self.pkrs_days) | ((self.pkrs & 0xF) | self.pkrs_strain << 4))"
    )]
    pkrs: u8,
    #[deku(skip, default = "*pkrs & 0xF")]
    pkrs_days: u8,
    #[deku(skip, default = "*pkrs >> 4")]
    pkrs_strain: u8,
    // 0x33 unused padding
    _padding33: u8,
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
    ribbon_count_memory_battle: u8,
    // 0x3E padding
    // 0x3F padding
    _padding3E_3F: u16,
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
    u48: u32,
    // 0x4C-0x4F unused
    _unused4C_4F: [u8; 4],
    height_scalar: u8,
    weight_scalar: u8,
    // 0x52-0x57 unused
    _unused52_57: [u8; 6],
    // Block B
    raw_nickname: [u16; NICK_LENGTH],
    _raw_nickname_terminator: u16,
    #[deku(
        skip,
        default = "String::from_utf16(raw_nickname).unwrap().trim_end_matches(char::from(0)).to_string()"
    )]
    nickname: String, // TODO use string_converter when implemented
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
    #[deku(skip, default = "((*iv32 >> 00) & 0x1F) as u8")]
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
    dynamax_level: u8,
    _unused90_93: [u8; 4],
    status_condition: i32,
    _unk98: i32,
}

impl From<&[u8; 344]> for PK8Config {
    fn from(data: &[u8; SIZE_8PARTY]) -> Self {
        let mut array = *data;
        decrypt_if_encrypted8(&mut array);
        let (_rest, test_file) = PK8Config::from_bytes((array.as_ref(), 0)).unwrap();
        test_file
    }
}

impl fmt::Debug for PK8Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PK8Config")
            .field("encryption_constant", &self.encryption_constant)
            .field("sanity", &self.sanity)
            .field("checksum", &self.checksum)
            .field("species", &self.species)
            .field("held_item", &self.held_item)
            .field("tid", &self.tid)
            .field("sid", &self.sid)
            .field("exp", &self.exp)
            .field("ability", &self.ability)
            .field("can_gigantamax", &self.can_gigantamax)
            .field("favourite", &self.favourite)
            .field("ability_number", &self.ability_number)
            .field("mark_value", &self.mark_value)
            .field("pid", &self.pid)
            .field("nature", &self.nature)
            .field("stat_nature", &self.stat_nature)
            .field("fateful_encounter", &self.fateful_encounter)
            .field("flag2", &self.flag2)
            .field("gender", &self.gender)
            .field("alt_form", &self.alt_form)
            .field("ev_hp", &self.ev_hp)
            .field("ev_atk", &self.ev_atk)
            .field("ev_def", &self.ev_def)
            .field("ev_spe", &self.ev_spe)
            .field("ev_spa", &self.ev_spa)
            .field("ev_spd", &self.ev_spd)
            .field("cnt_cool", &self.cnt_cool)
            .field("cnt_beauty", &self.cnt_beauty)
            .field("cnt_cute", &self.cnt_cute)
            .field("cnt_smart", &self.cnt_smart)
            .field("cnt_tough", &self.cnt_tough)
            .field("cnt_sheen", &self.cnt_sheen)
            .field("pkrs", &self.pkrs)
            .field("pkrs_days", &self.pkrs_days)
            .field("pkrs_strain", &self.pkrs_strain)
            .field("ribbon_champion_kalos", &self.ribbon_champion_kalos)
            .field("ribbon_champion_g3_hoenn", &self.ribbon_champion_g3_hoenn)
            .field("ribbon_champion_sinnoh", &self.ribbon_champion_sinnoh)
            .field("ribbon_best_friends", &self.ribbon_best_friends)
            .field("ribbon_training", &self.ribbon_training)
            .field("ribbon_battler_skillful", &self.ribbon_battler_skillful)
            .field("ribbon_battler_expert", &self.ribbon_battler_expert)
            .field("ribbon_effort", &self.ribbon_effort)
            .field("ribbon_alert", &self.ribbon_alert)
            .field("ribbon_shock", &self.ribbon_shock)
            .field("ribbon_downcast", &self.ribbon_downcast)
            .field("ribbon_careless", &self.ribbon_careless)
            .field("ribbon_relax", &self.ribbon_relax)
            .field("ribbon_snooze", &self.ribbon_snooze)
            .field("ribbon_smile", &self.ribbon_smile)
            .field("ribbon_gorgeous", &self.ribbon_gorgeous)
            .field("ribbon_royal", &self.ribbon_royal)
            .field("ribbon_gorgeous_royal", &self.ribbon_gorgeous_royal)
            .field("ribbon_artist", &self.ribbon_artist)
            .field("ribbon_footprint", &self.ribbon_footprint)
            .field("ribbon_record", &self.ribbon_record)
            .field("ribbon_legend", &self.ribbon_legend)
            .field("ribbon_country", &self.ribbon_country)
            .field("ribbon_national", &self.ribbon_national)
            .field("ribbon_earth", &self.ribbon_earth)
            .field("ribbon_world", &self.ribbon_world)
            .field("ribbon_classic", &self.ribbon_classic)
            .field("ribbon_premier", &self.ribbon_premier)
            .field("ribbon_event", &self.ribbon_event)
            .field("ribbon_birthday", &self.ribbon_birthday)
            .field("ribbon_special", &self.ribbon_special)
            .field("ribbon_souvenir", &self.ribbon_souvenir)
            .field("ribbon_wishing", &self.ribbon_wishing)
            .field("ribbon_champion_battle", &self.ribbon_champion_battle)
            .field("ribbon_champion_regional", &self.ribbon_champion_regional)
            .field("ribbon_champion_national", &self.ribbon_champion_national)
            .field("ribbon_champion_world", &self.ribbon_champion_world)
            .field("has_contest_memory_ribbon", &self.has_contest_memory_ribbon)
            .field("has_battle_memory_ribbon", &self.has_battle_memory_ribbon)
            .field("ribbon_champion_g6_hoenn", &self.ribbon_champion_g6_hoenn)
            .field("ribbon_contest_star", &self.ribbon_contest_star)
            .field("ribbon_master_coolness", &self.ribbon_master_coolness)
            .field("ribbon_master_beauty", &self.ribbon_master_beauty)
            .field("ribbon_master_cuteness", &self.ribbon_master_cuteness)
            .field("ribbon_master_cleverness", &self.ribbon_master_cleverness)
            .field("ribbon_master_toughness", &self.ribbon_master_toughness)
            .field("ribbon_champion_alola", &self.ribbon_champion_alola)
            .field("ribbon_battle_royale", &self.ribbon_battle_royale)
            .field("ribbon_battle_tree_great", &self.ribbon_battle_tree_great)
            .field("ribbon_battle_tree_master", &self.ribbon_battle_tree_master)
            .field("ribbon_champion_galar", &self.ribbon_champion_galar)
            .field("ribbon_tower_master", &self.ribbon_tower_master)
            .field("ribbon_master_rank", &self.ribbon_master_rank)
            .field("ribbon_mark_lunchtime", &self.ribbon_mark_lunchtime)
            .field("ribbon_mark_sleepy_time", &self.ribbon_mark_sleepy_time)
            .field("ribbon_mark_dusk", &self.ribbon_mark_dusk)
            .field("ribbon_mark_dawn", &self.ribbon_mark_dawn)
            .field("ribbon_mark_cloudy", &self.ribbon_mark_cloudy)
            .field("ribbon_mark_rainy", &self.ribbon_mark_rainy)
            .field("ribbon_mark_stormy", &self.ribbon_mark_stormy)
            .field("ribbon_mark_snowy", &self.ribbon_mark_snowy)
            .field("ribbon_mark_blizzard", &self.ribbon_mark_blizzard)
            .field("ribbon_mark_dry", &self.ribbon_mark_dry)
            .field("ribbon_mark_sandstorm", &self.ribbon_mark_sandstorm)
            .field(
                "ribbon_count_memory_contest",
                &self.ribbon_count_memory_contest,
            )
            .field(
                "ribbon_count_memory_battle",
                &self.ribbon_count_memory_battle,
            )
            .field("ribbon_mark_misty", &self.ribbon_mark_misty)
            .field("ribbon_mark_destiny", &self.ribbon_mark_destiny)
            .field("ribbon_mark_fishing", &self.ribbon_mark_fishing)
            .field("ribbon_mark_curry", &self.ribbon_mark_curry)
            .field("ribbon_mark_uncommon", &self.ribbon_mark_uncommon)
            .field("ribbon_mark_rare", &self.ribbon_mark_rare)
            .field("ribbon_mark_rowdy", &self.ribbon_mark_rowdy)
            .field("ribbon_mark_absent_minded", &self.ribbon_mark_absent_minded)
            .field("ribbon_mark_jittery", &self.ribbon_mark_jittery)
            .field("ribbon_mark_excited", &self.ribbon_mark_excited)
            .field("ribbon_mark_charismatic", &self.ribbon_mark_charismatic)
            .field("ribbon_mark_calmness", &self.ribbon_mark_calmness)
            .field("ribbon_mark_intense", &self.ribbon_mark_intense)
            .field("ribbon_mark_zoned_out", &self.ribbon_mark_zoned_out)
            .field("ribbon_mark_joyful", &self.ribbon_mark_joyful)
            .field("ribbon_mark_angry", &self.ribbon_mark_angry)
            .field("ribbon_mark_smiley", &self.ribbon_mark_smiley)
            .field("ribbon_mark_teary", &self.ribbon_mark_teary)
            .field("ribbon_mark_upbeat", &self.ribbon_mark_upbeat)
            .field("ribbon_mark_peeved", &self.ribbon_mark_peeved)
            .field("ribbon_mark_intellectual", &self.ribbon_mark_intellectual)
            .field("ribbon_mark_ferocious", &self.ribbon_mark_ferocious)
            .field("ribbon_mark_crafty", &self.ribbon_mark_crafty)
            .field("ribbon_mark_scowling", &self.ribbon_mark_scowling)
            .field("ribbon_mark_kindly", &self.ribbon_mark_kindly)
            .field("ribbon_mark_flustered", &self.ribbon_mark_flustered)
            .field("ribbon_mark_pumped_up", &self.ribbon_mark_pumped_up)
            .field("ribbon_mark_zero_energy", &self.ribbon_mark_zero_energy)
            .field("ribbon_mark_prideful", &self.ribbon_mark_prideful)
            .field("ribbon_mark_unsure", &self.ribbon_mark_unsure)
            .field("ribbon_mark_humble", &self.ribbon_mark_humble)
            .field("ribbon_mark_thorny", &self.ribbon_mark_thorny)
            .field("ribbon_mark_vigor", &self.ribbon_mark_vigor)
            .field("ribbon_mark_slump", &self.ribbon_mark_slump)
            .field("rib44_2", &self.rib44_2)
            .field("rib44_3", &self.rib44_3)
            .field("rib44_4", &self.rib44_4)
            .field("rib44_5", &self.rib44_5)
            .field("rib44_6", &self.rib44_6)
            .field("rib44_7", &self.rib44_7)
            .field("rib45_0", &self.rib45_0)
            .field("rib45_1", &self.rib45_1)
            .field("rib45_2", &self.rib45_2)
            .field("rib45_3", &self.rib45_3)
            .field("rib45_4", &self.rib45_4)
            .field("rib45_5", &self.rib45_5)
            .field("rib45_6", &self.rib45_6)
            .field("rib45_7", &self.rib45_7)
            .field("rib46_0", &self.rib46_0)
            .field("rib46_1", &self.rib46_1)
            .field("rib46_2", &self.rib46_2)
            .field("rib46_3", &self.rib46_3)
            .field("rib46_4", &self.rib46_4)
            .field("rib46_5", &self.rib46_5)
            .field("rib46_6", &self.rib46_6)
            .field("rib46_7", &self.rib46_7)
            .field("rib47_0", &self.rib47_0)
            .field("rib47_1", &self.rib47_1)
            .field("rib47_2", &self.rib47_2)
            .field("rib47_3", &self.rib47_3)
            .field("rib47_4", &self.rib47_4)
            .field("rib47_5", &self.rib47_5)
            .field("rib47_6", &self.rib47_6)
            .field("rib47_7", &self.rib47_7)
            .field("u48", &self.u48)
            .field("height_scalar", &self.height_scalar)
            .field("weight_scalar", &self.weight_scalar)
            .field("nickname", &self.nickname)
            .field("move1", &self.move1)
            .field("move2", &self.move2)
            .field("move3", &self.move3)
            .field("move4", &self.move4)
            .field("move1_pp", &self.move1_pp)
            .field("move2_pp", &self.move2_pp)
            .field("move3_pp", &self.move3_pp)
            .field("move4_pp", &self.move4_pp)
            .field("move_1_pp_ups", &self.move_1_pp_ups)
            .field("move_2_pp_ups", &self.move_2_pp_ups)
            .field("move_3_pp_ups", &self.move_3_pp_ups)
            .field("move_4_pp_ups", &self.move_4_pp_ups)
            .field("relearn_move1", &self.relearn_move1)
            .field("relearn_move2", &self.relearn_move2)
            .field("relearn_move3", &self.relearn_move3)
            .field("relearn_move4", &self.relearn_move4)
            .field("stat_hp_current", &self.stat_hp_current)
            .field("iv32", &self.iv32)
            .field("iv_hp", &self.iv_hp)
            .field("iv_atk", &self.iv_atk)
            .field("iv_def", &self.iv_def)
            .field("iv_spe", &self.iv_spe)
            .field("iv_spa", &self.iv_spa)
            .field("iv_spd", &self.iv_spd)
            .field("is_egg", &self.is_egg)
            .field("is_nicknamed", &self.is_nicknamed)
            .field("dynamax_level", &self.dynamax_level)
            .finish()
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
