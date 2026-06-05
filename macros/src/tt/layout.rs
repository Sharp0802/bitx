use crate::prelude::*;
use crate::tt::{Error, Input, Parse, parse_u32};

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
    fn test_dec() {
        let ts = quote!(1.2 ; 16);
        let mut input = Input::from(ts);
        let layout: Layout = input.parse().unwrap();

        assert_eq!(layout.offset, 10);
        assert_eq!(layout.size, 16);
    }

    #[test]
    fn test_hex() {
        let ts: TokenStream = "0x1.0;0x10".parse().unwrap();
        let mut input: Input = ts.into();
        let layout: Layout = input.parse().unwrap();

        assert_eq!(layout.offset, 8);
        assert_eq!(layout.size, 16);
    }

    #[test]
    fn test_oct() {
        let ts: TokenStream = "0o1.0;0o20".parse().unwrap();
        let mut input: Input = ts.into();
        let layout: Layout = input.parse().unwrap();

        assert_eq!(layout.offset, 8);
        assert_eq!(layout.size, 16);
    }

    #[test]
    fn test_bin() {
        let ts: TokenStream = "0b10.1;0b11".parse().unwrap();
        let mut input: Input = ts.into();
        let layout: Layout = input.parse().unwrap();

        assert_eq!(layout.offset, 17);
        assert_eq!(layout.size, 3);
    }

    #[test]
    fn test_too_big_bit_offset() {
        let mut input: Input = quote!(1.8 ; 16).into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_missing_semicolon() {
        let mut input: Input = quote!(1.2 16).into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_literal_first() {
        let mut input: Input = quote!(foo ; 16).into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_dot_after_prefix() {
        let mut input: Input = quote!(0x1 ; 16).into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_no_literal_after_dot() {
        let ts: TokenStream = "1 . foo ; 16".parse().unwrap();
        let mut input: Input = ts.into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_byte_suffix() {
        let ts: TokenStream = "1.0f32 ; 16".parse().unwrap();
        let mut input: Input = ts.into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_bit_suffix() {
        let ts: TokenStream = "1f32.0 ; 16".parse().unwrap();
        let mut input: Input = ts.into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_size_suffix() {
        let ts: TokenStream = "1.0 ; 16f32".parse().unwrap();
        let mut input: Input = ts.into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_nan_byte_offset() {
        let mut input: Input = quote!(bar . 2 ; 1).into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_nan_bit_offset() {
        let mut input: Input = quote!(1 . bar ; 1).into();
        assert!(input.parse::<Layout>().is_err());
    }

    #[test]
    fn test_nan_size() {
        let mut input: Input = quote!(1.2 ; bar).into();
        assert!(input.parse::<Layout>().is_err());
    }
}
