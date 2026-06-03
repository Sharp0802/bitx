#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

bits! {
    pub struct FuzzHeader: u4 {
        0.0;01 pub flag,
        0.1;03 pub status,
        0.4;12 pub unaligned_cross, // Crosses byte boundaries
        2.0;16 pub tail,
    }
}

proptest! {
    #[test]
    fn fuzz_struct_reads(bytes in any::<[u8; 4]>()) {
        // Ensure constructors never panic
        let header = FuzzHeader::from_array(bytes);

        // Ensure slice reads never panic on valid sizes
        assert!(FuzzHeader::from_slice(&bytes).is_some());

        // Getters shouldn't panic and should stay within bit bounds
        let _flag = header.flag();
        let status = header.status();
        let unaligned = header.unaligned_cross();

        assert!(status <= 7); // u3 max is 7
        assert!(unaligned <= 4095); // u12 max is 4095
    }
}
