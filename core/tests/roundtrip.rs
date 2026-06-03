#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

// A struct that exercises every layout shape: aligned, unaligned,
// cross-byte single, cross-byte multi, and a 1-bit bool field.
bits! {
    pub struct Mixed: u32 {
        0.0;01 pub flag,                       // bool
        0.1;04 pub aligned_nibble,             // 4 bits, unaligned
        0.5;11 pub cross_byte,                 // 11 bits, unaligned, crosses byte
        2.0;08 pub aligned_byte,               // 8 bits, byte-aligned
        3.0;04 pub tail_nibble,                // 4 bits, byte-aligned
        3.4;04 pub tail_subnibble,             // 4 bits, unaligned (offset 28)
    }
}

// Aligned builtin fields of differing widths (u8, u16, u32) in a
// single struct: each exercises a distinct <T>::from_be_bytes path.
bits! {
    pub struct Wide: u64 {
        0.0;16 pub head,
        2.0;32 pub middle,
        6.0;16 pub tail,
    }
}

// Single field spanning the entire struct (exercises the
// `upper >= mask_bytes` branch in hir::Layout::from).
bits! {
    pub struct AllBits: u16 {
        0.0;16 pub whole,
    }
}

// Struct smaller than its storage array: upper bits must be masked.
bits! {
    pub struct Short: u12 {
        0.0;12 pub low,
    }
}

#[inline]
fn extract_bits(bytes: &[u8], bit_offset: usize, size: usize) -> u128 {
    let mut val: u128 = 0;

    for index in 0..size {
        let abs = bit_offset + index;
        let byte = bytes[abs / 8];
        // big-endian / MSB-0
        let bit = (byte >> (7 - (abs % 8))) & 1;
        val = (val << 1) | u128::from(bit);
    }

    val
}

proptest! {
    #[test]
    fn roundtrip_mixed(bytes in any::<[u8; 4]>()) {
        let header = Mixed::from_array(bytes);

        // 1-bit bool
        prop_assert_eq!(header.flag(), extract_bits(&bytes, 0, 1) == 1);
        prop_assert_eq!(u128::from(header.aligned_nibble()),
            extract_bits(&bytes, 1, 4));
        prop_assert_eq!(u128::from(header.cross_byte()),
            extract_bits(&bytes, 5, 11));
        prop_assert_eq!(u128::from(header.aligned_byte()),
            extract_bits(&bytes, 16, 8));
        prop_assert_eq!(u128::from(header.tail_nibble()),
            extract_bits(&bytes, 24, 4));
        prop_assert_eq!(u128::from(header.tail_subnibble()),
            extract_bits(&bytes, 28, 4));
    }

    #[test]
    fn roundtrip_wide(bytes in any::<[u8; 8]>()) {
        let header = Wide::from_array(bytes);

        prop_assert_eq!(header.head(),
            u16::from_be_bytes([bytes[0], bytes[1]]));
        prop_assert_eq!(header.middle(),
            u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]));
        prop_assert_eq!(header.tail(),
            u16::from_be_bytes([bytes[6], bytes[7]]));
    }

    #[test]
    fn roundtrip_allbits(bytes in any::<[u8; 2]>()) {
        let header = AllBits::from_array(bytes);

        // The struct IS the mask, so the entire input is the field.
        prop_assert_eq!(header.whole(), u16::from_be_bytes(bytes));
    }

    /// A 12-bit struct: from_array takes 2 bytes (rounded up), and
    /// the low nibble of byte 1 is padding, not part of the field.
    #[test]
    fn short_struct_masks_upper_bits(input in any::<[u8; 2]>()) {
        let header = Short::from_array(input);

        // The codegen reads the 2-byte storage as a u16 BE, then
        // shifts right by `mask_size - field_size = 4` to align the
        // 12-bit field to the low bits, then masks off the top 4.
        let expected = (u16::from_be_bytes(input) >> 4) & 0x0FFF;
        prop_assert_eq!(header.low(), expected);
    }
}
