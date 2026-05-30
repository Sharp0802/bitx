#![allow(missing_docs)]

use bitx::bits;

bits! {
    #[derive(Debug)]
    pub enum Power: 0.2 {
        0 Off,
        1 Sleep,
        2 Standby,
        3 On,
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
    #[derive(Debug)]
    pub enum Status: 0.3 {
        1 Active,
        2 Paused,
        _ Unknown,
    }
}

#[test]
fn test_fallback() {
    assert_eq!(Status::from_array([0]), Status::Unknown);
    assert_eq!(Status::from_array([1]), Status::Active);
    assert_eq!(Status::from_array([2]), Status::Paused);
    assert_eq!(Status::from_array([3]), Status::Unknown);
    assert_eq!(Status::from_array([7]), Status::Unknown);
}
