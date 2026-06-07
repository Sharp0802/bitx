#![allow(missing_docs)]

use bitx::bits;

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
