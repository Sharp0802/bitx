#![allow(missing_docs)]

use bitx::bits;

bits! {
    pub struct IpV4Header: u16 {
        0.0;4 pub version,
        0.4;4 pub ihl,
        1.0;6 pub dscp,
        1.6;2 pub ecn,
    }
}

bits! {
    pub struct UnalignedHeader: u24 {
        0.0;4 pub version,
        0.4;12 pub mid,
        2.0;8 pub tail,
    }
}

#[test]
fn from_slice_returns_ref() {
    let buf = [0xFF, 0xFF, 0x45, 0x3F, 0xAA, 0xBB];

    let header: &IpV4Header =
        IpV4Header::from_slice(&buf[2..]).expect("slice should be long enough");

    assert_eq!(header.version(), 4);
    assert_eq!(header.ihl(), 5);
    assert_eq!(header.dscp(), 15);
    assert_eq!(header.ecn(), 3);

    let owned = IpV4Header::from_array([0x45, 0x3F]);
    assert_eq!(header.version(), owned.version());
    assert_eq!(header.ihl(), owned.ihl());
    assert_eq!(header.dscp(), owned.dscp());
    assert_eq!(header.ecn(), owned.ecn());
}

#[test]
fn from_slice_too_short_returns_none() {
    let buf = [0x45u8];
    assert!(IpV4Header::from_slice(&buf).is_none());
    assert!(IpV4Header::from_slice(&[]).is_none());
}

#[test]
fn from_slice_exact_size() {
    let buf = [0x45u8, 0x3F];
    let header: &IpV4Header = IpV4Header::from_slice(&buf).unwrap();
    assert_eq!(header.version(), 4);
}

#[test]
fn from_slice_oversize_is_ok() {
    let buf = [0x45u8, 0x3F, 0xFF, 0xFF, 0xFF];
    let header: &IpV4Header = IpV4Header::from_slice(&buf).unwrap();
    assert_eq!(header.version(), 4);
    assert_eq!(header.ihl(), 5);
}

#[test]
fn from_slice_unaligned() {
    let too_short = [0x01u8, 0x02];
    assert!(UnalignedHeader::from_slice(&too_short).is_none());

    let just_right = [0x01, 0x02, 0x03];
    let header: &UnalignedHeader =
        UnalignedHeader::from_slice(&just_right).unwrap();

    assert_eq!(header.version(), 0);
    assert_eq!(header.mid(), 0x102);
    assert_eq!(header.tail(), 0x03);
}

#[test]
fn from_slice_alignment_independent() {
    let buf = [0u8, 0, 0x45, 0x3F];
    let header: &IpV4Header = IpV4Header::from_slice(&buf[2..]).unwrap();
    assert_eq!(header.version(), 4);
    assert_eq!(header.ihl(), 5);
}
