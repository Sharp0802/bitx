use crate::prelude::*;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Offset {
    pub byte: usize,
    pub bit: usize,
}

impl Offset {
    #[must_use]
    #[inline]
    pub const fn from_bits(bits: usize) -> Self {
        Self {
            byte: bits / 8,
            bit: bits % 8,
        }
    }

    #[must_use]
    #[inline]
    pub const fn from_bytes(bytes: usize) -> Self {
        Self {
            byte: bytes,
            bit: 0,
        }
    }

    #[must_use]
    #[inline]
    pub const fn bits(self) -> usize {
        self.byte * 8 + self.bit
    }
}

impl Add for Offset {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let bits = self.bit + rhs.bit;
        Self {
            byte: self.byte + rhs.byte + bits / 8,
            bit: bits % 8,
        }
    }
}

impl Parse for Offset {
    fn parse(input: ParseStream) -> Result<Self> {
        let (byte, bit) = if input.peek(LitFloat) {
            let lit: LitFloat = input.parse()?;

            if !lit.suffix().is_empty() {
                return Err(Error::new(
                    lit.span(),
                    "offset cannot have a type suffix",
                ));
            }

            let str = lit.to_string();
            let (byte_str, bit_str) = str.split_once('.').unwrap();

            if bit_str.is_empty() {
                return Err(Error::new(
                    lit.span(),
                    "dot must be followed by bit offset",
                ));
            }

            let byte = LitInt::new(byte_str, lit.span());
            let bit = LitInt::new(bit_str, lit.span());

            (byte, Some(bit))
        } else {
            let byte = input.parse()?;

            let bit = if input.peek(Token![.]) {
                let _ = input.parse::<Token![.]>()?;
                Some(input.parse()?)
            } else {
                None
            };

            (byte, bit)
        };

        if !byte.suffix().is_empty() {
            return Err(Error::new(
                byte.span(),
                "byte offset cannot have a type suffix",
            ));
        }
        let byte = byte.base10_parse()?;

        let bit = if let Some(bit) = bit {
            if !bit.suffix().is_empty() {
                return Err(Error::new(
                    bit.span(),
                    "bit offset cannot have a type suffix",
                ));
            }
            let num = bit.base10_parse()?;
            if num >= 8 {
                return Err(Error::new(bit.span(), "bit offset must be 0-7"));
            }
            num
        } else {
            0
        };

        Ok(Self { byte, bit })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn from_float() {
        let tokens = quote! { 2.5 };
        let off: Offset = syn::parse2(tokens).unwrap();

        assert_eq!(off.byte, 2);
        assert_eq!(off.bit, 5);
    }

    #[test]
    fn from_ints() {
        let tokens = quote! { 0x2 . 5 };
        let off: Offset = syn::parse2(tokens).unwrap();

        assert_eq!(off.byte, 2);
        assert_eq!(off.bit, 5);
    }

    #[test]
    fn invalid_bit() {
        let tokens = quote! { 1.8 };
        let result: Result<Offset> = syn::parse2(tokens);

        assert!(result.is_err());
    }
}
