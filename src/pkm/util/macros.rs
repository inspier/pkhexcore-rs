#[macro_export]
macro_rules! get {
    ($self:ident, $field:ident) => {
        paste::paste! {
            $self.[<get_ $field:snake>]()
        }
    };
}

#[macro_export]
macro_rules! set {
    ($self:ident, $field:ident, $value:expr) => {
        paste::paste! {
            $self.[<set_ $field:snake>]($value)
        }
    };
}

macro_rules! field {
    ($self:ident; $field:ident; get: $out_type:ty => $getter:expr; set: $value_type:ty) => {
        paste::paste! {
            #[logfn(INFO)]
            #[logfn_inputs(Debug)]

            #[doc = "Update the `" $field "`."]
            pub fn [<$field:snake>](mut $self: Self, value: $value_type) -> Self {
                $self.[<set_ $field:snake>](value);
                $self
            }

            #[doc = "Get the `" $field "`."]
            pub fn [<get_ $field:snake>]($self: &Self) -> $out_type {
                $getter
            }
        }
    };

    ($self:ident; $field:ident; get: $out_type:ty => $getter:expr; set: T: $value_type:ty) => {
        paste::paste! {
            #[logfn(INFO)]
            #[logfn_inputs(Debug)]

            #[doc = "Update the `" $field "`."]
            pub fn [<$field:snake>]<T: $value_type + Debug>(mut $self: Self, value: T) -> Self {
                $self.[<set_ $field:snake>](value);
                $self
            }

            #[doc = "Get the `" $field "`."]
            pub fn [<get_ $field:snake>]($self: &Self) -> $out_type {
                $getter
            }
        }
    };
}
