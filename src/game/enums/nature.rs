#[repr(u8)]
/// Nature ID values for the corresponding English nature name.
pub enum Nature {
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,

    Random = 25,
}

macro_rules! impl_from {
    (for $($t:tt),+) => {
        $(impl_from!($t);)*
    };

    ($t:ident) => {
        impl From<Nature> for $t {
            fn from(nature: Nature) -> $t {
                nature as $t
            }
        }
    };
}

impl_from! (for u8, i32);
