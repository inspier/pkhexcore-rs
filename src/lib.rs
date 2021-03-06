#![allow(clippy::upper_case_acronyms)]
#![feature(non_ascii_idents)]
#![no_std]

extern crate alloc;

/// Module containing utilities to manipulate Pokemon data files.
#[macro_use]
pub mod util;

/// Module containing Pokemon file format related logic.
#[macro_use]
pub mod pkm;

/// Module containing game related logic.
pub mod game;

/// Module containing legality related logic.
pub mod legality;
