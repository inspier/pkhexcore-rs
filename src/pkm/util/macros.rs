#[macro_export]
macro_rules! get {
    ($self:ident, $field:ident) => {
        paste::item! {
            $self.[<get_ $field:snake>]()
        }
    };
}

#[macro_export]
macro_rules! set {
    ($self:ident, $field:ident, $value:expr) => {
        paste::item! {
            $self.[<set_ $field:snake>]($value)
        }
    };
}
