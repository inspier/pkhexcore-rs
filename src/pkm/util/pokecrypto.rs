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
    println!("{}", seed);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array8_test() {
        let mut pk8 = vec![
            81, 58, 53, 43, 0, 0, 142, 78, 58, 3, 0, 0, 32, 99, 91, 234, 192, 75, 3, 0, 68, 0, 17,
            0, 85, 1, 0, 0, 47, 121, 63, 200, 22, 22, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 147, 141, 0, 0, 0, 0, 0, 0, 79, 0, 114, 0, 98, 0, 101, 0, 101, 0, 116, 0, 108, 0,
            101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 1, 94, 0, 247, 0, 91, 1, 10, 10, 15, 20, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 255, 255, 250, 63, 10, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 110, 0, 117, 0, 98, 0, 105, 0, 115,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 50, 6, 82, 15, 47, 3, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 2, 0, 0, 0, 0, 0, 255, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 104, 0, 114, 0, 105, 0, 115, 0, 116, 0,
            111, 0, 112, 0, 104, 0, 101, 0, 0, 0, 0, 0, 0, 0, 50, 3, 83, 0, 99, 0, 18, 0, 0, 0, 20,
            2, 8, 0, 0, 0, 162, 0, 9, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 160, 0, 77, 0, 155, 0, 112, 0, 119,
            0, 183, 0, 0, 0,
        ];

        let mut ek8 = vec![
            81, 58, 53, 43, 0, 0, 142, 78, 91, 73, 216, 240, 17, 239, 104, 36, 45, 218, 139, 224,
            16, 231, 20, 46, 108, 254, 238, 153, 238, 152, 8, 241, 34, 104, 120, 152, 3, 117, 139,
            71, 169, 216, 66, 152, 164, 24, 111, 175, 41, 226, 51, 223, 138, 69, 174, 144, 254,
            112, 129, 109, 193, 101, 93, 235, 253, 172, 86, 159, 167, 93, 24, 129, 138, 169, 186,
            11, 126, 76, 9, 74, 7, 228, 240, 31, 222, 238, 251, 78, 245, 170, 201, 3, 115, 142, 3,
            119, 80, 210, 92, 76, 92, 194, 79, 34, 62, 93, 79, 85, 181, 221, 112, 29, 82, 250, 85,
            81, 210, 181, 61, 23, 60, 122, 63, 125, 1, 144, 107, 0, 121, 175, 54, 151, 247, 71,
            165, 16, 174, 52, 145, 148, 89, 179, 87, 98, 187, 61, 190, 234, 1, 53, 153, 90, 51, 87,
            160, 99, 224, 212, 59, 78, 137, 162, 102, 32, 47, 35, 224, 28, 248, 25, 56, 236, 193,
            20, 84, 177, 70, 213, 177, 232, 46, 95, 4, 157, 153, 98, 200, 230, 132, 26, 133, 70,
            120, 159, 204, 3, 173, 168, 103, 83, 149, 176, 151, 76, 88, 47, 3, 81, 239, 5, 238,
            177, 164, 51, 114, 199, 188, 132, 65, 64, 196, 87, 62, 38, 153, 64, 123, 155, 156, 222,
            86, 74, 114, 103, 179, 66, 236, 156, 124, 136, 90, 67, 109, 217, 157, 153, 122, 115,
            149, 117, 73, 186, 168, 95, 192, 59, 187, 157, 154, 107, 199, 9, 77, 117, 72, 134, 50,
            201, 177, 126, 92, 37, 63, 41, 131, 247, 170, 88, 176, 63, 139, 212, 243, 115, 250,
            196, 17, 36, 241, 119, 85, 214, 108, 63, 219, 113, 106, 66, 225, 23, 170, 165, 214,
            180, 74, 249, 53, 115, 173, 64, 120, 29, 81, 44, 170, 49, 130, 201, 86, 64, 159, 159,
            236, 249, 80, 246, 215, 88, 40, 73, 10, 240, 62, 239, 150, 36, 56, 218, 136, 224, 203,
            231, 113, 46,
        ];

        assert_eq!(ek8, encrypt_array8(&mut pk8));
        assert_eq!(pk8, decrypt_array8(&mut ek8));
    }
}
