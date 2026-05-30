use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::{Ident, Path, PathSegment, Token, Type, TypePath};

fn seg(seg: &str) -> PathSegment {
    let ident = Ident::new(seg, Span::call_site());
    ident.into()
}

pub fn ty(name: &str) -> Type {
    let mut punct = Punctuated::new();
    punct.push(seg("core"));
    punct.push(seg("primitive"));
    punct.push(seg(name));

    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: Some(<Token![::]>::default()),
            segments: punct,
        },
    })
}

pub fn size_of(ty: &Type) -> Option<usize> {
    let Type::Path(ty_path) = ty else {
        return None;
    };

    let ident = ty_path.path.get_ident()?;

    let str = ident.to_string();
    let bits = str.strip_prefix('u')?;

    let bits = bits.parse().ok()?;
    if bits == 0 {
        return None;
    }

    Some(bits)
}

pub fn with_bits(size: usize) -> Option<Type> {
    if size == 0 || 128 < size {
        return None;
    }

    Some(ty(&format!("u{size}")))
}
