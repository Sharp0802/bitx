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
    const DATA: [u8; 2] = [0x45, 0x3F];

    const HEADER: IpV4Header = IpV4Header::from_array(DATA);

    assert_eq!(HEADER.version(), 4);
    assert_eq!(HEADER.ihl(), 5);
    assert_eq!(HEADER.dscp(), 15);
    assert_eq!(HEADER.ecn(), 3);
}

#[test]
fn test_complex_unaligned_nested() {
    let data = [0xD5, 0x00, 0x01];
    let packet = ComplexPacket::from_array(data);

    assert!(packet.flag());
    assert_eq!(packet.nested().inner(), 0xAA);
    assert_eq!(packet.remainder(), 1);
}

#[test]
fn test_from_slice_bounds() {
    let too_small = [0x45];
    assert!(IpV4Header::from_slice(&too_small).is_none());

    let exact = [0x45, 0x3F];
    assert!(IpV4Header::from_slice(&exact).is_some());
}
