#[macro_export]
macro_rules! get {
    ($self:ident, $field:ident) => {
        paste::expr! {
            $self.[<get_ $field:snake>]()
        }
    };
}

#[macro_export]
macro_rules! set {
    ($self:ident, $field:ident, $value:expr) => {
        paste::expr! {
            $self.[<set_ $field:snake>]($value)
        }
    };
}

macro_rules! field {
    ($self:ident; $field:ident; get: $out_type:ty => $getter:expr; set: $value_type:ty) => {
        paste::item! {
            #[logfn(INFO)]
            #[logfn_inputs(Debug)]

            pub fn [<$field:snake>](mut $self: Self, value: $value_type) -> Self {
                $self.[<set_ $field:snake>](value);
                $self
            }

            pub fn [<get_ $field:snake>]($self: &Self) -> $out_type {
                $getter
            }
        }
    };
}
