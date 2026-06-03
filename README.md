# `bitx` [![Version]][crates.io] [![License]][crates.io] [![Docs]][docs.rs]

[Version]: https://img.shields.io/crates/v/bitx.svg
[License]: https://img.shields.io/crates/l/bitx.svg
[crates.io]: https://crates.io/crates/bitx
[Docs]: https://docs.rs/bitx/badge.svg
[docs.rs]: https://docs.rs/bitx/latest/bitx

`bitx` provides a prodecural macro for defining structures
with precise bit-level and byte-level fields,
using more ergonomic syntax.

## At a glance

Note that **Big-Endian / MSB-0** is used.

```rust
use bitx::bits;

bits! {
    /// Outer attributes are supported
    pub enum State: u3 {
        #[allow(dead_code)]
        0         => Inactive,
        1         => Active,
        2 | 6..=7 => Error,   // Arbitrary bit patterns
        _         => Unknown, // Default fallback for unmapped bit patterns
    }
}

bits! {
    pub struct Header: u36 { 
        // 1-bit fields automatically return `bool`
        0.0;01 pub is_active,

        // Custom nested types are supported
        0.1;03 pub state: State,

        // 20-bit integer; custom bit-widths are supported
        1.0;20 pub(crate) payload,

        // Unaligned cross-byte field
        3.4;08 checksum,
    }
}
```

```rust
use bitx::bits;
```

See [docs.rs](https://docs.rs/bitx/latest/bitx/macro.bits.html)
for more information.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.
