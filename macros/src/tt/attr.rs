use crate::prelude::*;
use crate::tt::{Error, Input, Parse};

#[derive(Debug)]
pub struct Attr(TokenStream);

impl Parse for Attr {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let mut attr = TokenStream::new();

        loop {
            if !is!(input.peek(); Punct '#') {
                break;
            }

            let pound = input.pop();
            pound.to_tokens(&mut attr);

            let group = tok! {
                input.pop();

                Group @ group => group,
                _ => return Err(input.error("`[` expected")),
            };

            group.to_tokens(&mut attr);
        }

        Ok(Self(attr))
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, to: &mut TokenStream) {
        to.extend(self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    tst!(Attr {
        single: "#[derive(Debug, Clone)]",
        multiple: "#[derive(Debug)]\n#[inline(always)]\n#[cfg(target_os = \"linux\")]",
        no_attr: "pub struct MyStruct" Ok(""),
        malformed: "# pub struct" Err,
    });
}
