use crate::prelude::*;
use crate::tt::{Error, Input, Parse};

#[derive(Debug, Clone)]
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
    let Ok(str) = core::str::from_utf8(slice) else {
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

    tst!(Type {
        ident: "u32",
        path: "std::collections",
        prefixed_path: "::std::collections",
        arrow: "fn() -> u32",
        nested_generic: "Vec<Map<i8, u32>>",

        trailing_comma: "u32," Ok("u32"),
        unpaired_gt: "u32 >> u32" Err,
    });

    #[test]
    fn test_literal() {
        for size in [8, 16, 32, 64, 128] {
            let ty = Type::literal(size);

            assert_eq!(
                ty.to_token_stream().to_string(),
                format!(":: core :: primitive :: u{size}"),
            );
        }
    }

    #[test]
    fn test_boolean() {
        let ty = Type::boolean();
        assert_eq!(
            ty.to_token_stream().to_string(),
            ":: core :: primitive :: bool"
        );
    }
}
