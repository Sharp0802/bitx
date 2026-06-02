use crate::prelude::*;
use crate::tt::*;

pub struct Value {
    pub start: u128,
    pub end: u128,
}

impl Parse for Value {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let lhs = {
            let lit: Literal = input.parse()?;
            let Ok(lhs) = parse_u128(&lit.to_string()) else {
                return Err(input.error("start of range must be valid integer"));
            };

            lhs
        };

        tok! {
            input.peek();

            Punct '.' => {
                _ = input.pop();

                if !is!(input.pop(); Punct '.') {
                    return Err(input.error("`.` expected"));
                }

                let incl = if is!(input.peek(); Punct '=') {
                    _ = input.pop();
                    true
                } else {
                    false
                };

                let lit: Literal = input.parse()?;

                let Ok(rhs) = parse_u128(&lit.to_string()) else {
                    return Err(input.error(
                        "end of range must be valid integer"
                    ));
                };

                let rhs = if incl {
                    rhs
                } else if rhs > lhs {
                    rhs - 1
                } else {
                    return Err(input.error(
                        "end of exclusive range must be \
                         greater than start of range"
                    ));
                };

                Ok(Self {
                    start: lhs,
                    end: rhs,
                })
            },
            _ => Ok(Self {
                start: lhs,
                end: lhs,
            }),
        }
    }
}
