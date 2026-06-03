use std::num::ParseIntError;

pub fn parse_u32(str: &str) -> Result<u32, ParseIntError> {
    match str.split_at_checked(2) {
        Some(("0x", hex)) => u32::from_str_radix(hex, 16),
        Some(("0o", oct)) => u32::from_str_radix(oct, 8),
        Some(("0b", bin)) => u32::from_str_radix(bin, 2),
        _ => str.parse(),
    }
}

pub fn parse_u128(str: &str) -> Result<u128, ParseIntError> {
    match str.split_at_checked(2) {
        Some(("0x", hex)) => u128::from_str_radix(hex, 16),
        Some(("0o", oct)) => u128::from_str_radix(oct, 8),
        Some(("0b", bin)) => u128::from_str_radix(bin, 2),
        _ => str.parse(),
    }
}
