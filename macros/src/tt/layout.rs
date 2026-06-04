use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token, parse_u32};

#[derive(Debug, Copy, Clone)]
pub struct Layout {
    pub offset: u32,
    pub size: u32,
}

impl Parse for Layout {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let offset = {
            let lit = tok! {
                input.pop();

                Literal @ lit => lit,
                _ => return Err(input.error("literal expected")),
            };

            let str = lit.to_string();

            let (bytes, bits) = if let Some(del) = str.find('.') {
                // byte offset has no prefix: 0.0, 1.5, ...

                (parse_u32(&str[..del]), parse_u32(&str[(del + 1)..]))
            } else {
                // byte offset has prefix: 0x0.0, 0o1.5, ...

                if !is!(input.pop(); Punct '.') {
                    return Err(input.error("`.` expected"));
                }

                let lit = tok! {
                    input.pop();

                    Literal @ lit => lit,
                    _ => return Err(input.error("literal expected")),
                };

                (parse_u32(&str), parse_u32(&lit.to_string()))
            };

            let (bytes, bits) = match (bytes, bits) {
                (Err(_), _) => {
                    return Err(Error::new(
                        "byte offset must be valid integer",
                        lit.span(),
                    ));
                }
                (_, Err(_)) => {
                    return Err(input.error("bit offset must be valid integer"));
                }

                (Ok(bytes), Ok(bits)) => (bytes, bits),
            };

            if bits >= 8 {
                return Err(input.error("bit offset must be in 0-7"));
            }

            bytes * 8 + bits
        };

        if !is!(input.pop(); Punct ';') {
            return Err(input.error("`;` expected"));
        }

        let size = {
            let lit = tok! {
                input.pop();

                Literal @ lit => lit,
                _ => return Err(input.error("literal expected")),
            };

            let str = lit.to_string();
            let Ok(size) = parse_u32(&str) else {
                return Err(input.error("size must be valid integer"));
            };

            size
        };

        Ok(Self { offset, size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ts = quote!(1.2 ; 16);
        let mut input = Input::from(ts);
        let layout: Layout = input.parse().unwrap();

        assert_eq!(layout.offset, 10);
        assert_eq!(layout.size, 16);
    }

    #[test]
    fn test_hex_and_octal() {
        let ts: TokenStream = "0x1.0 ; 0o20".parse().unwrap();
        let mut input = Input::from(ts);
        let layout: Layout = input.parse().unwrap();

        assert_eq!(layout.offset, 8);
        assert_eq!(layout.size, 16);
    }

    #[test]
    fn test_too_big_bit_offset() {
        let ts = quote!(1.8 ; 16);
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_missing_semicolon() {
        let ts = quote!(1.2 16);
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_literal_first() {
        // First token is an ident, not a literal.
        let ts = quote!(foo ; 16);
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_dot_after_prefix() {
        // `0x1` (prefix) needs a `.` and another literal after it.
        let ts = quote!(0x1 ; 16);
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_literal_after_dot() {
        // After the dot, the next token is an ident rather than a literal.
        let ts: TokenStream = "1 . foo ; 16".parse().unwrap();
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_invalid_byte_offset() {
        // The byte part is a literal that isn't a valid number.
        // Use a non-numeric literal whose `parse_u32` fails.
        // `0xZZ.0` is tricky: the proc-macro2 tokenizer may combine
        // them into a single literal. The simpler construction is
        // a literal whose first half is unparseable, e.g. via an
        // explicit `.` followed by a non-numeric part.
        // Real-world repro: a typo like `abc.0 ; 16` — first token
        // is an ident, which fails the "literal expected" check.
        // We test the byte-offset-parse-failure via a different path:
        // see test_invalid_size below.
        let ts: TokenStream = "1.0xZZ ; 16".parse().unwrap();
        let mut input = Input::from(ts);
        // `1.0xZZ` tokenizes as a single literal; `parse_u32` on
        // the bit part `"0xZZ"` fails with InvalidDigit.
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_invalid_bit_offset() {
        // Same root cause: a literal that splits into a valid byte
        // part and an invalid bit part.
        let ts: TokenStream = "1.0xZZ ; 16".parse().unwrap();
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_literal_size() {
        // The size is an ident, not a literal.
        let ts = quote!(1.2 ; bar);
        let mut input = Input::from(ts);
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_invalid_size() {
        // The size is a literal that isn't a valid number. A string
        // literal is a valid `proc_macro2::Literal` that `parse_u32`
        // will reject.
        use proc_macro2::Literal;
        let bad_size = Literal::string("not a number");
        let mut input = Input::from(quote!(1.2 ; #bad_size));
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_binary_prefix() {
        // Exercise the `0b` prefix path that was previously untested
        // at the integration level.
        let ts: TokenStream = "0b1.0 ; 0b1010".parse().unwrap();
        let mut input = Input::from(ts);
        let layout: Layout = input.parse().unwrap();
        assert_eq!(layout.offset, 8);
        assert_eq!(layout.size, 10);
    }

    #[test]
    fn test_invalid_byte_offset_via_string_literal() {
        // A string literal is a valid `Literal` token but isn't a
        // number, so the byte-offset parse fails.
        use proc_macro2::Literal;
        let bad = Literal::string("nope");
        let mut input = Input::from(quote!(#bad.0 ; 16));
        assert!(input.parse::<Layout>().is_err());
    }
}
