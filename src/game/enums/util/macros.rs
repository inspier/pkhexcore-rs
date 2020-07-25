#[macro_use]
macro_rules! impl_from {
    ($enum_name:tt for $($type:tt),+) => {
        $(impl_from!($type, $enum_name, $enum_name);)*
    };

    ($type:ident, $enum_name:ident, $var_name:ident) => {

        impl From<$enum_name> for $type {
            fn from($var_name: $enum_name) -> $type {
                $var_name as $type
            }
        }
    };
}

#[macro_use]
macro_rules! string_to_enum {
    ($enum_name:tt, $value_name:tt) => {
        paste::item! {
        $enum_name::$value_name
        }
    };
}
