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
    pub struct NestedData: u8 {
        0.0;8 pub inner,
    }
}

bits! {
    pub struct ComplexPacket: u24 {
        0.0;01 pub flag,
        0.1;08 pub nested: NestedData,
        1.1;15 pub remainder,
    }
}

#[test]
fn test_ipv4_header_extraction() {
    // 0x45 = 0100 0101 (Version: 4, IHL: 5)
    // 0x3F = 0011 1111 (DSCP: 15, ECN: 3)
    const DATA: [u8; 2] = [0x45, 0x3F];

    // Const context validation
    const HEADER: IpV4Header = IpV4Header::from_array(DATA);

    assert_eq!(HEADER.version(), 4);
    assert_eq!(HEADER.ihl(), 5);
    assert_eq!(HEADER.dscp(), 15);
    assert_eq!(HEADER.ecn(), 3);
}

#[test]
fn test_complex_unaligned_nested() {
    // 0b1_1010101, 0b0_0000000, 0b00000001
    // flag      = 1
    // nested    = 1010101 0        (0xAA)
    // remainder = 0000000 00000001 (1)
    let data = [0xD5, 0x00, 0x01];

    let packet = ComplexPacket::from_array(data);

    assert!(packet.flag());
    assert_eq!(packet.nested().inner(), 0xAA);
    assert_eq!(packet.remainder(), 1);
}

#[test]
fn test_from_slice_bounds() {
    let too_small = [0x45]; // Only 1 byte, IpV4Header expects 2
    assert!(IpV4Header::from_slice(&too_small).is_none());

    let exact = [0x45, 0x3F];
    assert!(IpV4Header::from_slice(&exact).is_some());
}
