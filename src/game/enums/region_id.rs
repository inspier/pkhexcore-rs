#[repr(u8)]
/// 3DS Console Region Identifiers
pub enum RegionID {
    None = 0,
    Japan = 1,
    NorthAmerica = 2,
    Europe = 3,
    China = 4,
    Korea = 5,
    Taiwan = 6,
}
