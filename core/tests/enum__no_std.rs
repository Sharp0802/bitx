#![allow(missing_docs)]
#![no_std]

use bitx::bits;

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum PowerState: u2 {
        0 => Off,
        1 => Sleep,
        2 => Standby,
        3 => On,
    }
}

#[test]
fn test_enum_no_std() {
    assert_eq!(PowerState::from_array([0]), PowerState::Off);
    assert_eq!(PowerState::from_array([3]), PowerState::On);
    assert_eq!(PowerState::from_slice(&[1]).unwrap(), PowerState::Sleep);
}
