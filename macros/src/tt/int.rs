use std::num::ParseIntError;

pub fn parse_u32(str: &str) -> Result<u32, ParseIntError> {
    if let Some(hex) = str.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)
    } else if let Some(oct) = str.strip_prefix("0o") {
        u32::from_str_radix(oct, 8)
    } else if let Some(bin) = str.strip_prefix("0b") {
        u32::from_str_radix(bin, 2)
    } else {
        str.parse()
    }
}

pub fn parse_u128(str: &str) -> Result<u128, ParseIntError> {
    if let Some(hex) = str.strip_prefix("0x") {
        u128::from_str_radix(hex, 16)
    } else if let Some(oct) = str.strip_prefix("0o") {
        u128::from_str_radix(oct, 8)
    } else if let Some(bin) = str.strip_prefix("0b") {
        u128::from_str_radix(bin, 2)
    } else {
        str.parse()
    }
}

