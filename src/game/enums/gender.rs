#[repr(u8)]
/// Gender a PKM can have
pub enum Gender {
    Male = 0,
    Female = 1,
    Genderless = 2,
}

#[allow(non_upper_case_globals)]
impl Gender {
    pub const Random: Gender = Gender::Genderless;
}
