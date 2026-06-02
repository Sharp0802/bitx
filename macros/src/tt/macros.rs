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
        match $s { $($t)* }
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
