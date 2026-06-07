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
