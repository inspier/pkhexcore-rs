macro_rules! impl_from {
    (for $($type:tt, $enum_name:tt),+) => {
        $(impl_from!($t);)*
    };

    ($t:ident) => {
        impl From<$enum_name> for $type {
            fn from($enum_name: Species) -> $type {
                $enum_name as $type
            }
        }
    };
}