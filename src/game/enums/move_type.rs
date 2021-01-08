#![allow(non_snake_case)]
use alloc::format;
use core::convert::TryFrom;
use deku::prelude::*;
use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
/// Elemental type a move has; additionally, types a PKM can have.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite, TryFromPrimitive)]
#[deku(type = "i8", ctx = "_endian: deku::ctx::Endian")]
#[repr(i8)]
pub enum MoveType {
    Any = -1,
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

impl_from! (MoveType for i8);

pub fn get_move_type_generation(move_type: MoveType, generation: i32) -> MoveType {
    if generation <= 2 {
        get_move_type_from_g12(move_type)
    } else {
        move_type
    }
}

fn get_move_type_from_g12(mut move_type: MoveType) -> MoveType {
    if move_type <= MoveType::Rock {
        return move_type;
    }
    move_type = MoveType::try_from(move_type as i8 - 1).unwrap(); // Skip unused Bird type
    if move_type <= MoveType::Steel {
        return move_type;
    }
    move_type = MoveType::try_from(move_type as i8 - 10).unwrap(); // 10 Normal duplicates
    move_type
}
