#![cfg_attr(not(test), no_std)]

pub mod decimal;
pub mod integer;

pub use decimal::SafeDec;
pub use integer::SafeInt;
