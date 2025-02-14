use crate::{parsing::ParsedSafeDec, SafeInt};
use core::{fmt::Display, ops::*, str::FromStr};
use quoth::Parsable;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeDec<const D: usize>(SafeInt);

impl<const D: usize> SafeDec<D> {
    pub const ZERO: SafeDec<D> = SafeDec::<D>(SafeInt::ZERO);

    fn scale_up(other: &SafeInt) -> SafeInt {
        other * SafeInt::from(10).pow(D as u32)
    }

    pub fn from_raw(raw: impl Into<SafeInt>) -> Self {
        SafeDec(raw.into())
    }
}

impl<const D: usize> FromStr for SafeDec<D> {
    type Err = quoth::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = quoth::ParseStream::from(s);
        let parsed = ParsedSafeDec::<D>::parse(&mut stream)?;
        Ok(SafeDec::<D>(parsed.raw))
    }
}

impl<const D: usize> Display for SafeDec<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let divisor = SafeInt::from(10).pow(D as u32);
        let abs_value = self.0.clone().abs();

        let integer_part = (&abs_value / &divisor).unwrap_or(SafeInt::ZERO);
        let decimal_part = abs_value % divisor;

        // Handle negative sign separately
        if self.0.is_negative() {
            write!(f, "-")?;
        }

        // Print integer part
        write!(f, "{}", integer_part)?;

        // Print decimal part with zero-padding
        write!(f, ".{:0width$}", decimal_part, width = D)
    }
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident) => {
        impl<const D: usize> $trait for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.$method(other.0))
            }
        }

        impl<const D: usize> $trait<&SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.clone().$method(&other.0))
            }
        }

        impl<const D: usize> $trait<&SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.$method(&other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.clone().$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).$method(&other.0))
            }
        }

        impl<const D: usize> $trait<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).$method(&other.0))
            }
        }

        impl<const D: usize> $trait<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).$method(&other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).$method(&other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for u8 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for u16 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for u32 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for u64 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for u128 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for i8 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for i16 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for i32 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for i64 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for i128 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for usize {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for isize {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).$method(other.0))
            }
        }
    };
}

impl_binary_op!(Add, add);
impl_binary_op!(Sub, sub);
impl_binary_op!(Mul, mul);
impl_binary_op!(Rem, rem);
impl_binary_op!(BitAnd, bitand);
impl_binary_op!(BitOr, bitor);
impl_binary_op!(BitXor, bitxor);

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
use alloc::string::ToString;

#[test]
fn test_safe_dec_from_str() {
    let parsed = "123.456".parse::<SafeDec<3>>().unwrap();
    assert_eq!(parsed.0, SafeInt::from(123456));
    let parsed = "123.456".parse::<SafeDec<4>>().unwrap();
    assert_eq!(parsed.0, SafeInt::from(1234560));
}

#[test]
fn test_safe_dec_add() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = a + b;
    assert_eq!(c.0, SafeInt::from(123456 + 654321));
    assert_eq!(c.to_string().as_str(), "777.777");
    assert_eq!(c, SafeDec::from_raw(777777));
}
