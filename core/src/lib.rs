#![doc = include_str!("../README.md")]
#![no_std]

/// Defines a structure with precise bit-level and byte-level fields.
///
/// This macro generates a `#[repr(C)]` struct
/// backed by a tightly packed `[u8; N]` array.
///
/// It automatically generates constant-time getter methods
/// for extracting sub-byte, unaligned, or standard fields,
/// handling all underlying bitwise shifts and masking safely.
///
/// # Syntax
///
/// The macro uses a custom syntax to define
/// the total size and the exact placement of each field:
///
/// ```rust
/// use bitx::bits;
///
/// bits! {
///     /// Optional documentation and attributes for the struct
///     // Total size in `byte.bit` notation (4.0 = 4 bytes / 32 bits)
///     pub struct Header: 4.4 {
///         /// Flag indicating active status
///         // Byte 0, Bit 0. 1-bit fields return `bool`
///         0.0 pub is_active: u1,
///         
///         /// A 3-bit status code
///         // Byte 0, Bit 1. Custom bit-widths are supported
///         0.1 pub status: u3,
///         
///         /// Standard aligned field
///         // Byte 1, Bit 0. bit offset defaults to 0
///         1 pub payload: u16,
///         
///         /// Unaligned cross-byte field
///         3.4 checksum: u8, // Byte 3, Bit 4.
///     }
/// }
/// ```
///
/// # Offsets
///
/// Offsets for the total struct size and
/// individual fields use a `<byte>.<bit>` format:
///
/// - `byte`: The byte offset.
/// - `bit`: The bit offset within the byte. Must be between 0 and 7.
///
/// If a field starts precisely on a byte boundary,
/// the `.0` suffix is optional
/// (e.g., `4` is equivalent to `4.0`).
///
/// # Types
///
/// Fields can be defined using standard or
/// custom bit-width primitives:
///
/// * `u1`: Treated as a boolean flag.
/// * `u2` to `u128`: Custom bit-width integers.
/// * `T`: Custom types are supported if created with `bits!` macro.
///   Unaligned reads for nested types are supported up to 128 bits.
///
/// # Generated API
///
/// For the declared struct, the macro generates:
///
/// - `pub const fn from_array(value: [u8; SIZE]) -> Self`
/// - `pub const fn from_slice(value: &[u8]) -> Option<&Self>`
/// - and field getters...
///
pub use bitx_macros::bits;

/// A trait representing bit-manipulatability of struct.
///
/// Since macro automatically implements and manages this trait,
/// It's highly discouraged that manually implementing this trait.
pub trait Bits {
    /// A mask type to extract data from byte stream.
    ///
    /// Unit type will be used for structs larger than 128-bit.
    type Mask;

    /// A declared size of struct, in bits.
    const BITS: u32;
}
