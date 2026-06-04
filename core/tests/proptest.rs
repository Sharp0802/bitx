#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

bits! {
    pub struct FuzzHeader: u32 {
        0.0;01 pub flag,
        0.1;03 pub status,
        0.4;12 pub unaligned_cross,
        2.0;16 pub tail,
    }
}

proptest! {
    #[test]
    fn fuzz_struct_reads(bytes in any::<[u8; 4]>()) {
        let header = FuzzHeader::from_array(bytes);

        assert!(FuzzHeader::from_slice(&bytes).is_some());

        let _flag = header.flag();
        let status = header.status();
        let unaligned = header.unaligned_cross();

        assert!(status <= 7);
        assert!(unaligned <= 4095);
    }
}
