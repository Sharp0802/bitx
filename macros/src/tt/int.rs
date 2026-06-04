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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u32_dec() {
        assert_eq!(parse_u32("0").unwrap(), 0);
        assert_eq!(parse_u32("42").unwrap(), 42);
        assert_eq!(parse_u32("4294967295").unwrap(), u32::MAX);
    }

    #[test]
    fn parse_u32_hex() {
        assert_eq!(parse_u32("0x0").unwrap(), 0);
        assert_eq!(parse_u32("0xFF").unwrap(), 0xFF);
        assert_eq!(parse_u32("0xdeadbeef").unwrap(), 0xDEAD_BEEF);
    }

    #[test]
    fn parse_u32_oct() {
        assert_eq!(parse_u32("0o0").unwrap(), 0);
        assert_eq!(parse_u32("0o17").unwrap(), 0o17);
        assert_eq!(parse_u32("0o755").unwrap(), 0o755);
    }

    #[test]
    fn parse_u32_bin() {
        assert_eq!(parse_u32("0b0").unwrap(), 0);
        assert_eq!(parse_u32("0b101").unwrap(), 0b101);
        assert_eq!(parse_u32("0b11111111").unwrap(), 0xFF);
    }

    #[test]
    fn parse_u32_invalid() {
        assert!(parse_u32("0xZZ").is_err());
        assert!(parse_u32("0o9").is_err());
        assert!(parse_u32("0b2").is_err());
        assert!(parse_u32("not a number").is_err());
    }

    #[test]
    fn parse_u128_dec() {
        assert_eq!(parse_u128("0").unwrap(), 0);
        assert_eq!(parse_u128("42").unwrap(), 42);
        assert!(parse_u128("not a number").is_err());
    }

    #[test]
    fn parse_u128_hex() {
        assert_eq!(parse_u128("0x0").unwrap(), 0);
        assert_eq!(
            parse_u128("0xffffffffffffffffffffffffffffffff").unwrap(),
            u128::MAX,
        );
    }

    #[test]
    fn parse_u128_oct() {
        assert_eq!(parse_u128("0o17").unwrap(), 0o17);
    }

    #[test]
    fn parse_u128_bin() {
        assert_eq!(parse_u128("0b1010").unwrap(), 0xA);
        assert_eq!(parse_u128("0b10101010").unwrap(), 0xAA);
    }
}
