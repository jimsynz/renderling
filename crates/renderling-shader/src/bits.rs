//! Helpers for bit packing/squashing.

use core::ops::RangeInclusive;

/// Statically define a shift/mask range as a literal range of bits.
pub const fn bits(range: RangeInclusive<u32>) -> (u32, u32) {
    let mut start = *range.start();
    let end = *range.end();
    let mut mask = 0;
    while start <= end {
        mask = (mask << 1) | 1;
        start += 1;
    }
    (*range.start(), mask)
}

/// Insert the value of the bits defined by the shift/mask range.
pub fn insert(bits: &mut u32, (shift, mask): (u32, u32), value: u32) {
    // rotate right
    if shift >= 1 {
        *bits = (*bits >> shift) | (*bits << (32 - shift));
    }
    // unset
    *bits &= !mask;
    // set
    *bits |= value & mask;
    // unrotate (rotate left)
    if shift >= 1 {
        *bits = (*bits << shift) | (*bits >> (32 - shift));
    }
}

/// Extract the value of the bits defined by the shift/mask range.
pub fn extract(bits: u32, (shift, mask): (u32, u32)) -> u32 {
    (bits >> shift) & mask
}
