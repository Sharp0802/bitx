#![allow(missing_docs)]

use bitx::bits;
use proptest::prelude::*;

bits! {
    #[derive(Debug, Eq, PartialEq)]
    struct Type: u256 {
        00.0;128 lhs,
        16.0;128 rhs,
    }
}

bits! {
    struct Struct: u256 {
        0.0;256 field: Type,
    }
}

proptest! {
    #[test]
    fn fuzz(seed in any::<[u8; 32]>()) {
        let strct = Struct::from_array(seed);
        let inner = Type::from_array(seed);

        assert_eq!(strct.field(), &inner);
        assert_eq!(inner.lhs().to_be_bytes(), &seed[..16]);
        assert_eq!(inner.rhs().to_be_bytes(), &seed[16..]);
    }
}
