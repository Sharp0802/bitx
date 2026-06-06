macro_rules! tok {
    (@v Ident [$v:literal] $n:ident) => { $n == $v };
    (@v Punct [$v:literal] $n:ident) => { $n.as_char() == $v };
    (@v $k:ident [] $n:ident) => { true };

    (@a [$s:expr] [$($t:tt)*]  End => $b:expr,  $($r:tt)*) => {
        tok!(@a [$s] [$($t)* Token::End => $b,] $($r)*)
    };
    (@a [$s:expr] [$($t:tt)*]  _ @ $k:ident => $b:expr,  $($r:tt)*) => {
        tok!(@a [$s] [$($t)* $k => $b,] $($r)*)
    };
    (@a [$s:expr] [$($t:tt)*]  _ => $b:expr,  $($r:tt)*) => {
        tok!(@a [$s] [$($t)* _ => $b,] $($r)*)
    };
    (@a [$s:expr] [$($t:tt)*]
     $k:ident $($v:literal)? @ $n:ident $(if $c:expr)? => $b:expr,
     $($r:tt)*
    ) => {
        tok!(@a [$s] [
            $($t)* Token::$k($n) if tok!(@v $k [$($v)?] $n) $(&& $c)? => $b,
        ] $($r)*)
    };
    (@a [$s:expr] [$($t:tt)*]
     $k:ident $($v:literal)? $(if $c:expr)? => $b:expr,
     $($r:tt)*
    ) => {
        tok!(@a [$s] [
            $($t)* Token::$k(val) if tok!(@v $k [$($v)?] val) $(&& $c)? => $b,
        ] $($r)*)
    };
    (@a [$s:expr] [$($t:tt)*]) => {
        {
            use $crate::tt::Token;
            match $s { $($t)* }
        }
    };

    ($s:expr ; $($t:tt)*) => { tok!(@a [$s] [] $($t)*) };
}

pub(crate) use tok;

macro_rules! is {
    ($s:expr ; $k:ident $($v:literal)?) => {
        tok!($s ; $k $($v)? => true, _ => false,)
    };
}

pub(crate) use is;

#[cfg(test)]
macro_rules! tst {
    ([$ty:ty] $name:ident $from:literal) => {
        ::paste::paste! {
            #[test]
            fn [< roundtrip_ $name >]() {
                _ = $crate::tt::internal::roundtrip::<$ty>($from);
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal Ok) => {
        ::paste::paste! {
            #[test]
            fn [< parse_ $name >]() {
                _ = $crate::tt::internal::parse::<$ty>($from);
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal Ok($lit:literal)) => {
        ::paste::paste! {
            #[test]
            fn [< parse_ $name >]() {
                $crate::tt::internal::ok::<$ty>($from, $lit);
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal Err) => {
        ::paste::paste! {
            #[test]
            fn [< deny_ $name >]() {
                $crate::tt::internal::deny::<$ty>($from, "");
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal Err($lit:literal)) => {
        ::paste::paste! {
            #[test]
            fn [< deny_ $name >]() {
                $crate::tt::internal::deny::<$ty>($from, $lit);
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal Display($lit:literal)) => {
        ::paste::paste! {
            #[test]
            fn [< display_ $name >]() {
                assert_eq!(
                    $crate::tt::internal::parse::<$ty>($from).to_string(),
                    $lit,
                );
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal @|$arg:ident| { $($tt:tt)* }) => {
        ::paste::paste! {
            #[test]
            fn [< test_ $name >]() {
                let $arg = $crate::tt::internal::parse::<$ty>($from);
                $($tt)*
            }
        }
    };
    ([$ty:ty] $name:ident $from:literal as $pat:pat) => {
        ::paste::paste! {
            #[test]
            fn [< match_ $name >]() {
                assert!(matches!(&$crate::tt::internal::parse($from), $pat));
            }
        }
    };

    ($ty:ty {
        $(
            $name:ident: $from:literal
            $(Ok$(($lit:literal))?)?
            $(Err$(($msg:literal))?)?
            $(Display($dis:literal))?
            $(@|$arg:ident| { $($tt:tt)* })?
            $(as $pat:pat)?
        ),* $(,)?
    }) => {
        $($crate::tt::tst!(
            [$ty] $name $from
            $(Ok$(($lit))?)?
            $(Err$(($msg))?)?
            $(Display($dis))?
            $(@|$arg| { $($tt)* })?
            $(as $pat)?
        );)*
    };
}

#[cfg(test)]
pub(crate) use tst;

#[cfg(test)]
pub mod internal {
    use crate::prelude::*;
    use crate::tt::{Input, Parse};
    use core::fmt::Debug;
    use core::str::FromStr;

    pub fn parse<T: Parse>(src: &str) -> T {
        let ts: TokenStream = src.parse().unwrap();
        let mut input: Input = ts.into();
        input.parse::<T>().unwrap()
    }

    pub fn ok<T: Parse + ToTokens>(src: &str, cmp: &str) {
        assert_eq!(
            parse::<T>(src).to_token_stream().to_string(),
            TokenStream::from_str(cmp).unwrap().to_string(),
        );
    }

    pub fn roundtrip<T: Parse + ToTokens>(src: &str) {
        let ts: TokenStream = src.parse().unwrap();
        let mut input: Input = ts.clone().into();
        let val: T = input.parse().unwrap();

        let inn = ts.to_string();
        let out = val.to_token_stream().to_string();

        tok!(input.peek();
            End => {},
            _ @ tt => panic!("parsing was not exhaustive; got {:?}", tt),
        );

        assert_eq!(&inn, &out);
    }

    pub fn deny<T: Parse + Debug>(src: &str, msg: &str) {
        let ts: TokenStream = src.parse().unwrap();
        let mut input: Input = ts.into();
        let err = input.parse::<T>().unwrap_err();

        assert!(
            err.message().contains(msg),
            "`{}` expected; got `{}`",
            msg,
            err.message()
        );
    }
}
