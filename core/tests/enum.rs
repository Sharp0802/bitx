#![allow(missing_docs)]

use bitx::bits;

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Power: u2 {
        0 => Off,
        1 => Sleep,
        2 => Standby,
        3 => On,
    }
}

#[test]
fn test_coverage() {
    assert_eq!(Power::from_array([0]), Power::Off);
    assert_eq!(Power::from_array([1]), Power::Sleep);
    assert_eq!(Power::from_array([2]), Power::Standby);
    assert_eq!(Power::from_array([3]), Power::On);

    // The macro should mask out the upper 6 bits:
    assert_eq!(Power::from_array([4]), Power::Off);
    assert_eq!(Power::from_array([5]), Power::Sleep);
    assert_eq!(Power::from_array([6]), Power::Standby);
    assert_eq!(Power::from_array([7]), Power::On);
}

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Status: u3 {
        1 => Active,
        2 => Paused,
        _ => Unknown,
    }
}

#[test]
fn test_fallback() {
    assert_eq!(Status::from_array([0]), Status::Unknown(0));
    assert_eq!(Status::from_array([1]), Status::Active);
    assert_eq!(Status::from_array([2]), Status::Paused);
    assert_eq!(Status::from_array([3]), Status::Unknown(3));
    assert_eq!(Status::from_array([7]), Status::Unknown(7));

    // The macro should mask out the upper 5 bits
    assert_eq!(Status::from_array([8]), Status::Unknown(0));
    assert_eq!(Status::from_array([10]), Status::Paused);
    assert_eq!(Status::from_array([15]), Status::Unknown(7));
}

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum State: u3 {
        1..3 | 7  => Active,
        4 | 3..=5 => Error,
        _         => Unknown,
    }
}

#[test]
fn test_ranges() {
    assert_eq!(State::from_array([0]), State::Unknown(0));
    assert_eq!(State::from_array([1]), State::Active(1));
    assert_eq!(State::from_array([2]), State::Active(2));
    assert_eq!(State::from_array([3]), State::Error(3));
    assert_eq!(State::from_array([4]), State::Error(4));
    assert_eq!(State::from_array([5]), State::Error(5));
    assert_eq!(State::from_array([6]), State::Unknown(6));
    assert_eq!(State::from_array([7]), State::Active(7));

    // The macro should mask out the upper 5 bits
    assert_eq!(State::from_array([8]), State::Unknown(0));
    assert_eq!(State::from_array([9]), State::Active(1));
    assert_eq!(State::from_array([11]), State::Error(3));
    assert_eq!(State::from_array([14]), State::Unknown(6));
}

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
