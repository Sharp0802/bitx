use crate::tt::*;

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
                        lit.span()
                    ));
                },
                (_, Err(_)) => {
                    return Err(input.error("bit offset must be valid integer"));
                },

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
    use crate::prelude::*;

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
}
