use core::{ops::*, str::FromStr};
use quoth::Parsable;

use crate::{parsing::ParsedSafeDec, SafeInt};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeDec<const D: usize>(SafeInt);

impl<const D: usize> SafeDec<D> {
    pub const ZERO: SafeDec<D> = SafeDec::<D>(SafeInt::ZERO);

    fn scale_up(other: &SafeInt) -> SafeInt {
        other * SafeInt::from(10).pow(D as u32)
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

        impl<const D: usize> $trait<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).$method(&other))
            }
        }

        impl<const D: usize> $trait<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).$method(&other))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).$method(other.0))
            }
        }

        impl<const D: usize> $trait<SafeDec<D>> for u8 {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn $method(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeInt::from(self).$method(other.0))
            }
        }
    };
}

impl_binary_op!(Add, add);

#[test]
fn test_safe_dec_from_str() {
    let parsed = "123.456".parse::<SafeDec<3>>().unwrap();
    assert_eq!(parsed.0, SafeInt::from(123456));
    let parsed = "123.456".parse::<SafeDec<4>>().unwrap();
    assert_eq!(parsed.0, SafeInt::from(1234560));
    let other = SafeDec::<4>::scale_up(&SafeInt::from(100));
    assert_eq!(other, SafeInt::from(100_0000));
    let parsed = other + parsed;
    assert_eq!(parsed.0, SafeInt::from(1234560 + 100_0000));
}
