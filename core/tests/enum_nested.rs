#![allow(missing_docs)]

use bitx::bits;

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum PowerState: u2 {
        0 Off,
        1 Sleep,
        2 Standby,
        3 On,
    }
}

bits! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Status: u3 {
        1 Active,
        2 Paused,
        _ Unknown,
    }
}

bits! {
    pub struct DeviceHeader: u6 {
        0.0;3 pub status: Status,
        0.3;3 pub flag,
    }
}

#[test]
fn test_enum_in_struct() {
    // [ Status (3) | Flag (3) | Pad ]
    // [   0 1 0    |  1 1 0   | 00  ]

    let header = DeviceHeader::from_array([0b010_110_00]);

    assert_eq!(header.status(), Status::Paused);
    assert_eq!(header.flag(), 6);
}
