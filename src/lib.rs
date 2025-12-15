#![no_std]
#![deny(missing_docs)]
//! Safe, non-panicking numeric primitives built on top of pure-Rust `num-bigint` (alloc-only).

extern crate alloc;

#[cfg(test)]
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
