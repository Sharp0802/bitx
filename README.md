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

```rust
use bitx::bits;

bits! {
    #[derive(Clone, Copy)]
    pub struct Header: 4.4 { // size in `byte.bit` notation (so 36-bit)
        // 1-bit fields automatically return `bool`
        0.0 pub is_active: u1,

        // Custom bit-widths are supported
        0.1 pub status: u3,
        
        // The `.0` bit offset can be omitted.
        1 pub payload: u16,

        // Unaligned cross-byte field
        3.4 checksum: u8,
    }
}
```

Note that **Big-Endian / MSB-0** is used.

See [docs.rs](https://docs.rs/bitx/latest/bitx/macro.bits.html)
for more information.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.

