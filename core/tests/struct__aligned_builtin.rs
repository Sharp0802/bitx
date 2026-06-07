#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

bits! {
    struct Struct: u32 {
        0.0;16 dead,
        2.0;16 beef,
    }
}

proptest! {
    #[test]
    fn fuzz(seed in any::<[u8; 4]>()) {
        let strct = Struct::from_array(seed);

        assert_eq!(strct.dead().to_be_bytes(), &seed[..2]);
        assert_eq!(strct.beef().to_be_bytes(), &seed[2..]);
    }
}
