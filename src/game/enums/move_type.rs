use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

/// Elemental type a move has; additionally, types a PKM can have.
#[repr(i8)]
#[derive(PartialEq, PartialOrd, TryFromPrimitive)]
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
