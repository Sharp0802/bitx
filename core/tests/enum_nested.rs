#![allow(missing_docs)]

use bitx::bits;

bits! {
    #[derive(Debug)]
    pub enum PowerState: 0.2 {
        0 Off,
        1 Sleep,
        2 Standby,
        3 On,
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

bits! {
    #[derive(Debug)]
    pub enum Status: 0.3 {
        1 Active,
        2 Paused,
        _ Unknown,
    }
}

#[test]
fn test_enum_with_fallback() {
    assert_eq!(Status::from_array([1]), Status::Active);
    assert_eq!(Status::from_array([2]), Status::Paused);
    assert_eq!(Status::from_array([0]), Status::Unknown);
    assert_eq!(Status::from_array([3]), Status::Unknown);
    assert_eq!(Status::from_array([7]), Status::Unknown);
}

bits! {
    #[derive(Debug)]
    pub enum Connection: 0.1 {
        0 Disconnected,
        1 Connected,
    }
}

#[test]
fn test_enum_attributes() {
    assert_eq!(Connection::from_array([1]), Connection::Connected);
}

bits! {
    pub struct DeviceHeader: 1.0 {
        0.0 pub power: PowerState,
        0.2 pub status: Status,
        0.5 pub flag: u3,
    }
}

#[test]
fn test_enum_in_struct() {
    // [ Power (2) | Status (3) | Flag (3) ]
    // [   1 0     |   0 1 0    |  1 1 1   ]
    // -> 10010111 in binary -> 0x97

    let header = DeviceHeader::from_array([0b10_010_111]);

    assert_eq!(header.power(), PowerState::Standby);
    assert_eq!(header.status(), Status::Paused);
    assert_eq!(header.flag(), 7);
}
