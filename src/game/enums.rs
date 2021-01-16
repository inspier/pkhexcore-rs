/// Useful utilities for Enums
#[macro_use]
#[doc(hidden)]
pub(crate) mod util;

/// Ability IDs for the corresponding English ability name.
pub mod ability;

/// Ball IDs for the corresponding English ball name.
pub mod ball;

/// Statuses of a bitflag.
pub mod flag;

/// GameVersion analogues used by Colosseum/XD instead of the main-series
/// values.
pub mod gc_version;

/// Gender a PKM can have.
pub mod gender;

/// Species IDs for the corresponding English species name.
pub mod species;

/// Game Version ID enum shared between actual Version IDs and lumped version
/// groupings.
pub mod game_version;

/// Game Language IDs.
pub mod language_gc;

/// Contiguous series Game Language IDs.
pub mod language_id;

/// Elemental type a move has; additionally, types a PKM can have.
pub mod move_type;

/// Move IDs for the corresponding English move name.
pub mod moves;

/// Nature ID values for the corresponding English nature name.
pub mod nature;

/// 3DS Console Region Identifiers.
pub mod region_id;
