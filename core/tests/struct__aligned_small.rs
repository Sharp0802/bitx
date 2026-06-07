#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

bits! {
    #[derive(Debug, Eq, PartialEq)]
    struct Type: u64 {
        0.0;32 lhs,
        4.0;32 rhs,
    }
}

bits! {
    struct Struct: u64 {
        0.0;64 field: Type,
    }
}

proptest! {
    #[test]
    fn fuzz(seed in any::<[u8; 8]>()) {
        let strct = Struct::from_array(seed);
        let inner = Type::from_array(seed);

        assert_eq!(strct.field(), &inner);
        assert_eq!(inner.lhs().to_be_bytes(), &seed[..4]);
        assert_eq!(inner.rhs().to_be_bytes(), &seed[4..]);
    }
}
