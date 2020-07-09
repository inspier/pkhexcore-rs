use crate::util::bitconverter;

pub const SIZE_1ULIST: usize = 69;
pub const SIZE_1JLIST: usize = 59;
pub const SIZE_1PARTY: usize = 44;
pub const SIZE_1STORED: usize = 33;

pub const SIZE_2ULIST: usize = 73;
pub const SIZE_2JLIST: usize = 63;
pub const SIZE_2PARTY: usize = 48;
pub const SIZE_2STORED: usize = 32;

pub const SIZE_3CSTORED: usize = 312;
pub const SIZE_3XSTORED: usize = 196;
pub const SIZE_3PARTY: usize = 100;
pub const SIZE_3STORED: usize = 80;
pub const SIZE_3BLOCK: usize = 12;

pub const SIZE_4PARTY: usize = 236;
pub const SIZE_4STORED: usize = 136;
pub const SIZE_4BLOCK: usize = 32;

pub const SIZE_5PARTY: usize = 220;
pub const SIZE_5STORED: usize = 136;
pub const SIZE_5BLOCK: usize = 32;

pub const SIZE_6PARTY: usize = 0x104;
pub const SIZE_6STORED: usize = 0xE8;
pub const SIZE_6BLOCK: usize = 56;

// Gen7 Format is the same size as Gen6.

pub const SIZE_8STORED: usize = 8 + (4 * SIZE_8BLOCK); // 0x148
pub const SIZE_8PARTY: usize = SIZE_8STORED + 0x10; // 0x158
pub const SIZE_8BLOCK: usize = 80; // 0x50

/// Positions for shuffling.
const BLOCK_POSITION: [u8; 128] = [
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
const BLOCK_POSITION_INVERT: [u8; 32] = [
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
pub fn shuffle_array8(data: &[u8; 344], sv: u32, block_size: usize) -> [u8; 344] {
    let mut sdata = data.clone();
    let index: u32 = sv * 4;
    let start: usize = 8;
    for block in 0..4 {
        let ofs: u32 = BLOCK_POSITION[(index + block) as usize] as u32;
        let sdata_start = start + block_size * block as usize;
        let data_start = start + block_size * ofs as usize;
        let data_end = data_start + block_size;
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
pub fn decrypt_array8(ekm: &mut [u8; 344]) -> [u8; 344] {
    let pv: u32 = bitconverter::to_uint32(ekm, 0);
    let sv = pv >> 13 & 31;
    crypt_pkm(ekm, pv, SIZE_8BLOCK);
    shuffle_array8(ekm, sv, SIZE_8BLOCK)
}

/// Decrypts a Gen8 pkm byte array.
///
/// # Arguments
///
/// * `pkm` - Decrypted Pokémon data
///
pub fn encrypt_array8(pkm: &mut [u8; 344]) -> [u8; 344] {
    let pv: u32 = bitconverter::to_uint32(pkm, 0);
    let sv = pv >> 13 & 31;
    let mut ekm = shuffle_array8(pkm, BLOCK_POSITION_INVERT[sv as usize] as u32, SIZE_8BLOCK);
    crypt_pkm(&mut ekm, pv, SIZE_8BLOCK);
    ekm
}

#[inline]
fn crypt_pkm(data: &mut [u8], pv: u32, block_size: usize) {
    let start = 8;
    let end = (4 * block_size) + start;
    crypt_array(data, pv, start, end); // Blocks
    if data.len() > (end as usize) {
        crypt_array(data, pv, end, data.len()); // Party Stats
    }
}

#[inline]
pub fn crypt_array(data: &mut [u8], mut seed: u32, start: usize, end: usize) {
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
fn crypt(data: &mut [u8], seed: &mut u32, i: usize) {
    *seed = 0x41C64E6Du32.wrapping_mul(*seed) + 0x00006073;
    data[i] ^= (*seed >> 16) as u8;
    data[i + 1] ^= (*seed >> 24) as u8;
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
pub fn decrypt_if_encrypted8(pkm: &mut [u8; 344]) {
    if bitconverter::to_uint16(pkm, 0x70) != 0 || bitconverter::to_uint16(pkm, 0xC0) != 0 {
        *pkm = decrypt_array8(pkm);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array8_test() {
        let mut pk8 = include_bytes!("tests/data/Orbeetle.pk8").clone();
        let mut ek8 = include_bytes!("tests/data/Orbeetle.ek8").clone();

        assert!(ek8.iter().eq(encrypt_array8(&mut pk8).iter()));
        assert!(pk8.iter().eq(decrypt_array8(&mut ek8).iter()));
        // assert_eq!(pk8, decrypt_array8(&mut *ek8));
    }

    #[test]
    fn decrypt_if_encrypted_test() {
        let pk8 = include_bytes!("tests/data/Orbeetle.pk8").clone();
        let mut pk8_temp = include_bytes!("tests/data/Orbeetle.pk8").clone();
        let mut ek8 = include_bytes!("tests/data/Orbeetle.ek8").clone();

        decrypt_if_encrypted8(&mut ek8);
        assert!(pk8.iter().eq(ek8.iter()));
        // assert_eq!(pk8, ek8);
        decrypt_if_encrypted8(&mut pk8_temp);
        // assert_eq!(pk8, pk8_temp);
        assert!(pk8.iter().eq(pk8_temp.iter()));
    }
}
