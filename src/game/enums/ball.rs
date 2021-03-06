use alloc::format;
use deku::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, DekuRead, DekuWrite)]
#[deku(type = "u8", ctx = "_endian: deku::ctx::Endian")]
#[non_exhaustive]
#[repr(u8)]
/// Ball IDs for the corresponding English ball name.
pub enum Ball {
    None = 0,

    Master = 1,
    Ultra = 2,
    Great = 3,
    Poke = 4,

    Safari = 5,

    Net = 6,
    Dive = 7,
    Nest = 8,
    Repeat = 9,
    Timer = 10,
    Luxury = 11,
    Premier = 12,
    Dusk = 13,
    Heal = 14,
    Quick = 15,

    Cherish = 16,

    Fast = 17,
    Level = 18,
    Lure = 19,
    Heavy = 20,
    Love = 21,
    Friend = 22,
    Moon = 23,

    Sport = 24,
    Dream = 25,
    Beast = 26,
}

impl Default for Ball {
    fn default() -> Self { Ball::None }
}

/// Checks if the [`Ball`] is an Apricorn Ball (HG/SS)
/// # Arguments
///
/// * `ball` - Ball ID
pub fn is_apricorn_ball(ball: Ball) -> bool { Ball::Fast <= ball && ball <= Ball::Moon }
