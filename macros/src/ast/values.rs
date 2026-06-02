use crate::tt::*;
use crate::ast::Value;

pub struct Values(Vec<Value>);

impl Parse for Values {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let mut values = Vec::new();

        if is!(input.peek(); Punct '_') {
            _ = input.pop();
            return Ok(Self(values));
        }

        values.push(input.parse()?);

        while is!(input.peek(); Punct '|') {
            _ = input.pop();
            values.push(input.parse()?);
        }

        Ok(Self(values))
    }
}

impl Values {
    pub fn take(self) -> Vec<Value> {
        self.0
    }
}
