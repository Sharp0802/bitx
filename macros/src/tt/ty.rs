use crate::prelude::*;
use crate::tt::*;

pub struct Type(TokenStream);

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
}
