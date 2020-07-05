use crate::util::bitconverter;

static SIZE_1ULIST: u32 = 69;
static SIZE_1JLIST: u32 = 59;
static SIZE_1PARTY: u32 = 44;
static SIZE_1STORED: u32 = 33;

static SIZE_2ULIST: u32 = 73;
static SIZE_2JLIST: u32 = 63;
static SIZE_2PARTY: u32 = 48;
static SIZE_2STORED: u32 = 32;

static SIZE_3CSTORED: u32 = 312;
static SIZE_3XSTORED: u32 = 196;
static SIZE_3PARTY: u32 = 100;
static SIZE_3STORED: u32 = 80;
static SIZE_3BLOCK: u32 = 12;

static SIZE_4PARTY: u32 = 236;
static SIZE_4STORED: u32 = 136;
static SIZE_4BLOCK: u32 = 32;

static SIZE_5PARTY: u32 = 220;
static SIZE_5STORED: u32 = 136;
static SIZE_5BLOCK: u32 = 32;

static SIZE_6PARTY: u32 = 0x104;
static SIZE_6STORED: u32 = 0xE8;
static SIZE_6BLOCK: u32 = 56;

// Gen7 Format is the same size as Gen6.

static SIZE_8STORED: u32 = 8 + (4 * SIZE_8BLOCK); // 0x148
static SIZE_8PARTY: u32 = SIZE_8STORED + 0x10; // 0x158
static SIZE_8BLOCK: u32 = 80; // 0x50

/// Positions for shuffling.
static BLOCK_POSITION: [u8; 128] = [
    0, 1, 2, 3, //
    0, 1, 3, 2, //
    0, 2, 1, 3, //
    0, 3, 1, 2, //
    0, 2, 3, 1, //
    0, 3, 2, 1, //
    1, 0, 2, 3, //
    1, 0, 3, 2, //
    2, 0, 1, 3, //
    3, 0, 1, 2, //
    2, 0, 3, 1, //
    3, 0, 2, 1, //
    1, 2, 0, 3, //
    1, 3, 0, 2, //
    2, 1, 0, 3, //
    3, 1, 0, 2, //
    2, 3, 0, 1, //
    3, 2, 0, 1, //
    1, 2, 3, 0, //
    1, 3, 2, 0, //
    2, 1, 3, 0, //
    3, 1, 2, 0, //
    2, 3, 1, 0, //
    3, 2, 1, 0, //
    // duplicates of 0-7 to eliminate modulus
    0, 1, 2, 3, //
    0, 1, 3, 2, //
    0, 2, 1, 3, //
    0, 3, 1, 2, //
    0, 2, 3, 1, //
    0, 3, 2, 1, //
    1, 0, 2, 3, //
    1, 0, 3, 2, //
];

/// Positions for unshuffling.
static BLOCK_POSITION_INVERT: [u8; 32] = [
    0, 1, 2, 4, 3, 5, 6, 7, 12, 18, 13, 19, 8, 10, 14, 20, 16, 22, 9, 11, 15, 21, 17, 23, //
    0, 1, 2, 4, 3, 5, 6, 7, // duplicates of 0-7 to eliminate modulus
];

/// Shuffles a 232 byte array containing Pokémon data.
///
/// # Arguments
///
/// * `data` - Data to shuffle
/// * `sv` - Block Shuffle order
/// * `block_size` - Size of shuffling chunks
///
#[inline]
pub fn shuffle_array(data: &[u8], sv: u32, block_size: u32) -> Vec<u8> {
    let mut sdata = data.to_vec();
    let index: u32 = sv * 4;
    let start: u32 = 8;
    for block in 0..4 {
        let ofs: u32 = BLOCK_POSITION[(index + block) as usize] as u32;
        let sdata_start = (start + (block_size * block)) as usize;
        let data_start = (start + (block_size * ofs)) as usize;
        let data_end = (data_start as u32 + block_size) as usize;
        let slice_size = data_end - data_start;
        sdata[sdata_start..sdata_start + slice_size].copy_from_slice(&data[data_start..data_end]);
    }
    return sdata;
}

/// Decrypts a Gen8 pkm byte array.
///
/// # Arguments
///
/// * `ekm` - Encrypted Pokémon data
///
pub fn decrypt_array8(ekm: &mut [u8]) -> Vec<u8> {
    let pv: u32 = bitconverter::to_uint32(ekm, 0);
    let sv = pv >> 13 & 31;
    crypt_pkm(ekm, pv, SIZE_8BLOCK);
    shuffle_array(ekm, sv, SIZE_8BLOCK)
}

/// Decrypts a Gen8 pkm byte array.
///
/// # Arguments
///
/// * `pkm` - Decrypted Pokémon data
///
pub fn encrypt_array8(pkm: &mut [u8]) -> Vec<u8> {
    let pv: u32 = bitconverter::to_uint32(pkm, 0);
    let sv = pv >> 13 & 31;
    let mut ekm = shuffle_array(pkm, BLOCK_POSITION_INVERT[sv as usize] as u32, SIZE_8BLOCK);
    crypt_pkm(&mut ekm, pv, SIZE_8BLOCK);
    ekm
}

#[inline]
fn crypt_pkm(data: &mut [u8], pv: u32, block_size: u32) {
    let start = 8;
    let end = (4 * block_size) + start;
    crypt_array(data, pv, start, end); // Blocks
    if data.len() > (end as usize) {
        crypt_array(data, pv, end, data.len() as u32); // Party Stats
    }
}

#[inline]
pub fn crypt_array(data: &mut [u8], mut seed: u32, start: u32, end: u32) {
    let mut i = start;
    while {
        crypt(data, &mut seed, i);
        i += 2;
        crypt(data, &mut seed, i);
        i += 2;

        i < end
    } {}
}

#[inline]
fn crypt(data: &mut [u8], seed: &mut u32, i: u32) {
    *seed = 0x41C64E6Du32.wrapping_mul(*seed) + 0x00006073;
    data[i as usize] ^= (*seed >> 16) as u8;
    data[(i + 1) as usize] ^= (*seed >> 24) as u8;
}

/// Gets the checksum of a 232 byte array.
///
/// # Arguments
///
/// * `data` - Decrypted Pokémon data.
///
pub fn get_chk(data: &[u8]) -> u16 {
    let mut chk = 0;
    for i in (8..SIZE_6STORED).step_by(2) {
        chk += bitconverter::to_uint16(data, i as usize);
    }
    chk
}

/// Decrypts a Gen8 pkm byte array.
///
/// # Arguments
///
/// * `pkm` - Possibly encrypted Pokémon data.
///
pub fn decrypt_if_encrypted8(pkm: &mut Vec<u8>) {
    if bitconverter::to_uint16(pkm, 0x70) != 0 || bitconverter::to_uint16(pkm, 0xC0) != 0 {
        *pkm = decrypt_array8(pkm)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array8_test() {
        let mut pk8 = include_bytes!("tests/data/Orbeetle.pk8").to_vec();
        let mut ek8 = include_bytes!("tests/data/Orbeetle.ek8").to_vec();

        assert_eq!(ek8, encrypt_array8(&mut *pk8));
        assert_eq!(pk8, decrypt_array8(&mut *ek8));
    }

    #[test]
    fn decrypt_if_encrypted_test() {
        let pk8 = include_bytes!("tests/data/Orbeetle.pk8").to_vec();
        let mut pk8_temp = include_bytes!("tests/data/Orbeetle.pk8").to_vec();
        let mut ek8 = include_bytes!("tests/data/Orbeetle.ek8").to_vec();

        decrypt_if_encrypted8(&mut ek8);
        assert_eq!(pk8, ek8);
        decrypt_if_encrypted8(&mut pk8_temp);
        assert_eq!(pk8, pk8_temp);
    }
}
