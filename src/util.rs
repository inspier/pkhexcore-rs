/// A big-endian specific module that converts base data types to an array of
/// bytes, and an array of bytes to base data types.
pub mod bigendian;

/// A little-endian specific module that converts base data types to an array of
/// bytes, and an array of bytes to base data types.
pub mod bitconverter;

/// Utility logic for dealing with bitflags in a byte array.
pub mod flagutil;

/// Utility to work with game dates
pub mod date;
