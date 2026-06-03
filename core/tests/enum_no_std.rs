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
fn test_enum_full_coverage() {
    let state_off = PowerState::from_array([0]);
    assert_eq!(state_off, PowerState::Off);

    let state_on = PowerState::from_array([3]);
    assert_eq!(state_on, PowerState::On);

    let state_sleep = PowerState::from_slice(&[1]).unwrap();
    assert_eq!(state_sleep, PowerState::Sleep);
}
