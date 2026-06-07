#![allow(missing_docs)]

use bitx::bits;

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
