#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
//! Safe, non-panicking numeric primitives built on top of pure-Rust `num-bigint` (alloc-only).
//!
//! `std` support is enabled by default; disable default features to use `no_std` + `alloc`.
//!
//! `SafeInt` and `SafeDec` implement `lencode::Encode`/`Decode`. The wire format uses a
//! compact, little-endian varint header for values with up to 63 payload bytes, and
//! falls back to lencode's `Vec<u8>` encoding for larger magnitudes.

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

/// Fixed-precision decimal support built on `SafeInt`.
pub mod decimal;
/// Arbitrary-precision integer support and helpers.
pub mod integer;
/// Parsers for `SafeInt` and `SafeDec` literals.
pub mod parsing;

/// Re-export of the fixed-precision decimal type.
pub use decimal::SafeDec;
/// Re-export of the arbitrary-precision integer type.
pub use integer::SafeInt;
