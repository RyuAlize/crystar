use crc32fast::Hasher;

const MASK_DELTA: u32 = 0xa282ead8;

pub fn hash(data: &[u8]) -> u32 {
    let mut h = Hasher::new();
    h.update(data);
    h.finalize()
}

pub fn extend(crc: u32, data: &[u8]) -> u32 {
    let mut h = Hasher::new_with_initial(crc);
    h.update(data);
    h.finalize()
}

pub fn mask(crc: u32) -> u32 {
    ((crc >> 15) | (crc << 17)).wrapping_add(MASK_DELTA)
}

pub fn unmask(masked: u32) -> u32 {
    let rot = masked.wrapping_sub(MASK_DELTA);
    (rot >> 17) | (rot << 15)
}
