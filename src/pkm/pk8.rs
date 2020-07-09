use crate::pkm::util::pokecrypto::{SIZE_8PARTY};

// Alignment bytes
static UNUSED: [u16; 12] = [
    0x17, 0x1A, 0x1B, 0x23, 0x33, 0x3E, 0x3F,
    0xE0, 0xE1,
    0xC5, 0x115, 0x11F,
];

static FORMAT: u32 = 8;

// TODO: PersonalInfo

pub struct PK8 {
    data: [u8; SIZE_8PARTY],
    affixed_ribbon: i8,
}

impl Default for PK8 {
    fn default() -> Self {
        PK8 {
            data: [0; SIZE_8PARTY],
            affixed_ribbon: -1,
        }
    }
}

impl From<&[u8; 344]> for PK8 {
    fn from(data: &[u8; 344]) -> Self {
        PK8 { data: data.clone(), ..Default::default() }
    }
}

impl From<&[u8]> for PK8 {
    fn from(data: &[u8]) -> Self {
        let mut array: [u8; SIZE_8PARTY] = [0; SIZE_8PARTY];
        array.copy_from_slice(&data[0..SIZE_8PARTY]);
        PK8 { data: array, ..Default::default() }
    }
}

impl PK8 {
    pub fn new() -> PK8 {
        return PK8 { ..Default::default() };
    }
}

impl PartialEq for PK8 {
    fn eq(&self, other: &Self) -> bool {
        (self.data.iter().eq(other.data.iter())) && (self.affixed_ribbon == other.affixed_ribbon)
    }
}
impl Eq for PK8 {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pk8_new_test() {
        let orbeetle_1 = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.pk8"));
        let orbeetle_2 = PK8::from(&*include_bytes!("util/tests/data/Orbeetle.pk8"));
        let dracovish = PK8::from(&*include_bytes!("util/tests/data/Dracovish.pk8"));

        assert_eq!(true, orbeetle_1 == orbeetle_2);
        assert_eq!(false, dracovish == orbeetle_1);
    }
}

