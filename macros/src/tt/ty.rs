use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};

#[derive(Clone)]
pub struct Type(TokenStream);

// NOTE: maximum length of 32-bit unsigned integer is 10 in base 10.
#[inline]
const fn itoa(buffer: &mut [u8; 11], mut size: u32) -> &str {
    buffer[0] = b'u';

    let len = size.ilog10() as usize + 1;
    let mut i = len;
    while i > 0 {
        buffer[i] = (size % 10) as u8 + b'0';
        size /= 10;
        i -= 1;
    }

    let slice = buffer.split_at(len + 1).0;

    // SAFETY: 1. b'u' is a valid ASCII.
    //         2. b'0' + x (where 0 <= x < 10) is a valid ASCII.
    let Ok(str) = ::core::str::from_utf8(slice) else {
        unreachable!();
    };

    str
}

impl Type {
    #[inline]
    fn literal_tokens(size: u32, span: Span) -> [TokenTree; 9] {
        let mut buffer = [0u8; 11];
        let name = itoa(&mut buffer, size);

        [
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("core", span)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("primitive", span)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new(name, span)),
        ]
    }

    pub fn literal(size: u32) -> Self {
        let mut ts = TokenStream::new();
        ts.extend(Self::literal_tokens(size, Span::call_site()));

        Self(ts)
    }

    pub fn boolean() -> Self {
        let span = Span::call_site();

        Self(TokenStream::from_iter([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("core", span)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("primitive", span)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new("bool", span)),
        ]))
    }
}

impl Parse for Type {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let mut ret = TokenStream::new();
        let mut depth = 0i32;
        let mut aft_hyp = false;

        loop {
            tok! {
                input.peek();

                Punct '>' if aft_hyp => {
                    // `->`
                },
                Punct '>' @ tt if depth == 1 => {
                    let tt = input.pop();
                    tt.to_tokens(&mut ret);
                    break;
                },
                Punct '>' if depth == 0 => {
                    return Err(input.error("found `>` without matching `<`"));
                },

                Punct ',' if depth == 0 => break,
                End => break,

                Punct '<' => {
                    depth += 1;
                },
                Punct '>' => {
                    depth -= 1;
                },

                _ => {},
            };

            let tt = input.pop();
            aft_hyp = is!(&tt; Punct '-');
            tt.to_tokens(&mut ret);
        }

        Ok(Self(ret))
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, to: &mut TokenStream) {
        to.extend(self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ts = quote!(std::collections::HashMap<String, u32>);
        let mut input: Input = ts.clone().into();

        let parsed: Type = input.parse().unwrap();
        let mut out = TokenStream::new();
        parsed.to_tokens(&mut out);

        assert_eq!(ts.to_string(), out.to_string());
    }

    #[test]
    fn test_single_ident() {
        // No `<...>`, no leading `::` — just a bare ident. Hits the
        // `End => break` branch in the parser.
        let ts = quote!(u32);
        let mut input: Input = ts.clone().into();

        let parsed: Type = input.parse().unwrap();
        let mut out = TokenStream::new();
        parsed.to_tokens(&mut out);

        assert_eq!(ts.to_string(), out.to_string());
    }

    #[test]
    fn test_return_type_with_arrow() {
        // `-> u32` inside a function signature: the parser must NOT
        // treat `>` as a generic-close. The `->` is a single arrow.
        let ts = quote!(fn() -> u32);
        let mut input: Input = ts.clone().into();

        // The full source is `fn() -> u32`. Parsing should consume
        // `fn() -` first, then encounter `>` with `aft_hyp == true`,
        // which is the `->` special case. The trailing ` u32` then
        // becomes the return type.
        let parsed: Type = input.parse().unwrap();
        let mut out = TokenStream::new();
        parsed.to_tokens(&mut out);

        assert_eq!(ts.to_string(), out.to_string());
    }

    #[test]
    fn test_nested_generics() {
        // depth goes 0 -> 1 -> 2 -> 1 -> 0 across the
        // `Vec<HashMap<String, u32>>` shape.
        let ts = quote!(Vec<HashMap<String, u32>>);
        let mut input: Input = ts.clone().into();

        let parsed: Type = input.parse().unwrap();
        let mut out = TokenStream::new();
        parsed.to_tokens(&mut out);

        assert_eq!(ts.to_string(), out.to_string());
    }

    #[test]
    fn test_stray_gt_rejected() {
        // A `>` with no matching `<` is a parse error.
        let ts = quote!(u32 >> u32);
        let mut input: Input = ts.into();

        let result: Result<Type, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_comma_in_generics_terminates() {
        // A `,` at depth 0 terminates the type. Useful for parsing
        // the second generic arg without consuming the comma.
        let ts = quote!(u32,);
        let mut input: Input = ts.into();

        let parsed: Type = input.parse().unwrap();
        let mut out = TokenStream::new();
        parsed.to_tokens(&mut out);

        assert_eq!(out.to_string(), "u32");
    }

    #[test]
    fn test_literal_emits_typed_path() {
        // `Type::literal(8)` should produce the path
        // `::core::primitive::u8`. Note: `TokenStream::to_string`
        // inserts spaces around `::` because the puncts are joint.
        let ty = Type::literal(8);
        let mut out = TokenStream::new();
        ty.to_tokens(&mut out);
        assert_eq!(out.to_string(), ":: core :: primitive :: u8");
    }

    #[test]
    fn test_literal_wider_widths() {
        for size in [16u32, 32, 64, 128] {
            let ty = Type::literal(size);
            let mut out = TokenStream::new();
            ty.to_tokens(&mut out);
            let expected = format!(":: core :: primitive :: u{size}");
            assert_eq!(out.to_string(), expected, "size = {size}");
        }
    }

    #[test]
    fn test_boolean_emits_bool_path() {
        let ty = Type::boolean();
        let mut out = TokenStream::new();
        ty.to_tokens(&mut out);
        assert_eq!(out.to_string(), ":: core :: primitive :: bool");
    }
}
