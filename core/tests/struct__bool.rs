#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

bits! {
    struct Struct: u1 {
        0.0;1 field,
    }
}

proptest! {
    #[test]
    fn fuzz(seed in any::<u8>()) {
        let strct = Struct::from_array([seed]);
        assert_eq!(strct.field(), (seed >> 7) == 1);
    }
}
