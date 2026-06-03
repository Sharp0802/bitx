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
    // Prefix with garbage bytes: from_slice must read the FIRST
    // `bytes` of the slice, not skip them.
    let buf = [0xFF, 0xFF, 0x45, 0x3F, 0xAA, 0xBB];

    let header: &IpV4Header =
        IpV4Header::from_slice(&buf[2..]).expect("slice should be long enough");

    // The returned reference must observe the same bytes the slice
    // points to; reading through it must yield the same fields as
    // a from_array of those exact bytes.
    assert_eq!(header.version(), 4);
    assert_eq!(header.ihl(), 5);
    assert_eq!(header.dscp(), 15);
    assert_eq!(header.ecn(), 3);

    // Sanity: from_array on the same 2 bytes matches.
    let owned = IpV4Header::from_array([0x45, 0x3F]);
    assert_eq!(header.version(), owned.version());
    assert_eq!(header.ihl(), owned.ihl());
    assert_eq!(header.dscp(), owned.dscp());
    assert_eq!(header.ecn(), owned.ecn());
}

#[test]
fn from_slice_too_short_returns_none() {
    // 1 byte is too few for a 2-byte struct.
    let buf = [0x45u8];
    assert!(IpV4Header::from_slice(&buf).is_none());

    // Empty slice: also too short.
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
    // A slice longer than the struct is fine: only the first
    // `bytes` are read, the rest is ignored.
    let buf = [0x45u8, 0x3F, 0xFF, 0xFF, 0xFF];
    let header: &IpV4Header = IpV4Header::from_slice(&buf).unwrap();
    assert_eq!(header.version(), 4);
    assert_eq!(header.ihl(), 5);
}

#[test]
fn from_slice_unaligned() {
    // Unaligned 24-bit struct: 1 byte too few.
    let too_short = [0x01u8, 0x02];
    assert!(UnalignedHeader::from_slice(&too_short).is_none());

    let just_right = [0x01, 0x02, 0x03];
    let header: &UnalignedHeader =
        UnalignedHeader::from_slice(&just_right).unwrap();

    // version at 0.0;4 = high nibble of byte 0 = 0
    assert_eq!(header.version(), 0);
    // mid at 0.4;12 = low nibble of byte 0 + all of byte 1
    //   byte 0 low nibble = 0x1
    //   byte 1 = 0x02
    //   combined 12-bit value (MSB-0) = 0001_0000_0010 = 0x102 = 258
    assert_eq!(header.mid(), 0x102);
    // tail at 2.0;8 = byte 2
    assert_eq!(header.tail(), 0x03);
}

#[test]
fn from_slice_alignment_independent() {
    // The returned reference must be safe to read regardless of
    // the source slice's alignment. Pass an odd-starting slice.
    let buf = [0u8, 0, 0x45, 0x3F];
    let header: &IpV4Header = IpV4Header::from_slice(&buf[2..]).unwrap();
    assert_eq!(header.version(), 4);
    assert_eq!(header.ihl(), 5);
}
