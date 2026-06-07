#![allow(missing_docs)]

use bitx::bits;

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum MaxSize: u128 {
        0..0x80000000000000000000000000000000 => Beg,
        _ => End,
    }
}

#[test]
fn test_ranges_u128() {
    const BEG: u128 = u128::MAX >> 1;
    const END: u128 = BEG + 1;

    assert_eq!(MaxSize::from_array([0u8; 16]), MaxSize::Beg(0));
    assert_eq!(MaxSize::from_array(BEG.to_be_bytes()), MaxSize::Beg(BEG));
    assert_eq!(MaxSize::from_array(END.to_be_bytes()), MaxSize::End(END));
    assert_eq!(MaxSize::from_array([0xFFu8; 16]), MaxSize::End(u128::MAX));
}
