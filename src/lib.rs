#![feature(min_const_generics)]
#![feature(non_ascii_idents)]

/// Module containing utilities to manipulate Pokemon data files
#[macro_use]
pub mod util;

/// Module containing Pokemon file format related logic
#[macro_use]
pub mod pkm;

/// Module containing game related logic
pub mod game;
