extern crate alloc;

use crate::{SafeInt, parsing::ParsedSafeDec};
#[cfg(test)]
use alloc::string::ToString;
#[cfg(test)]
use alloc::vec::Vec;
use core::{fmt::Display, ops::*, str::FromStr};
use lencode::dedupe::{DedupeDecoder, DedupeEncoder};
#[cfg(test)]
use lencode::io::Cursor;
use lencode::io::{Read, Write};
use lencode::{Decode, Encode};
use quoth::Parsable;

/// Fixed-point decimal built on top of `SafeInt` with `D` fractional digits.
///
/// # Examples
/// ```
/// use safe_bigmath::SafeDec;
///
/// // 3 decimal places
/// let a: SafeDec<3> = "1.500".parse().unwrap();
/// let b: SafeDec<3> = "2.250".parse().unwrap();
/// let c = a + b;
/// assert_eq!(c.to_string(), "3.750");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeDec<const D: usize>(SafeInt);

impl<const D: usize> SafeDec<D> {
    /// Zero value at the given scale.
    pub fn zero() -> SafeDec<D> {
        SafeDec::<D>(SafeInt::zero())
    }

    fn scale_up(other: &SafeInt) -> SafeInt {
        other * SafeInt::from(10).pow(D as u32)
    }

    fn scale_down(other: &SafeInt) -> SafeInt {
        (other / SafeInt::from(10).pow(D as u32)).unwrap_or(0.into())
    }

    /// Creates a `SafeDec` from an already scaled integer.
    ///
    /// # Examples
    /// ```
    /// use safe_bigmath::SafeDec;
    ///
    /// let raw = 123_456; // represents 123.456 at 3 decimal places
    /// let dec = SafeDec::<3>::from_raw(raw);
    /// assert_eq!(dec.to_string(), "123.456");
    /// ```
    pub fn from_raw(raw: impl Into<SafeInt>) -> Self {
        SafeDec(raw.into())
    }

    /// Converts between decimal scales, preserving magnitude.
    ///
    /// # Examples
    /// ```
    /// use safe_bigmath::SafeDec;
    ///
    /// let a: SafeDec<4> = "1.2345".parse().unwrap();
    /// let b = SafeDec::<2>::from_other_scale(a);
    /// assert_eq!(b.to_string(), "1.24");
    /// ```
    pub fn from_other_scale<const D2: usize>(other: SafeDec<D2>) -> Self {
        if D2 > D {
            SafeDec((other.0.ceil_div(SafeInt::from(10).pow((D2 - D) as u32))).unwrap())
        } else {
            SafeDec(other.0 * SafeInt::from(10).pow((D - D2) as u32))
        }
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
        let ten = SafeInt::from(10);
        let divisor = ten.clone().pow(D as u32);
        let abs_value = self.0.clone().abs();

        // These divisions are safe since divisor is never zero
        let integer_part = (&abs_value / &divisor).unwrap_or(SafeInt::zero());
        let mut decimal_part = (&abs_value % &divisor).unwrap_or_else(SafeInt::zero);

        if self.0.is_negative() {
            write!(f, "-")?;
        }

        write!(f, "{}", integer_part)?;
        write!(f, ".")?;

        // Extract D decimal digits
        let mut pow10 = (&divisor / &ten).unwrap(); // guaranteed non-zero
        for _ in 0..D {
            let (digit, rem) = decimal_part
                .div_rem(pow10.clone())
                .unwrap_or_else(|| (SafeInt::zero(), SafeInt::zero()));
            write!(f, "{}", digit)?;
            decimal_part = rem;
            pow10 = (&pow10 / &ten).unwrap_or(1.into()); // last one goes to 1
        }

        Ok(())
    }
}

impl<const D: usize> Encode for SafeDec<D> {
    #[inline(always)]
    fn encode_ext(
        &self,
        writer: &mut impl Write,
        dedupe_encoder: Option<&mut DedupeEncoder>,
    ) -> lencode::Result<usize> {
        self.0.encode_ext(writer, dedupe_encoder)
    }
}

impl<const D: usize> Decode for SafeDec<D> {
    #[inline(always)]
    fn decode_ext(
        reader: &mut impl Read,
        dedupe_decoder: Option<&mut DedupeDecoder>,
    ) -> lencode::Result<Self> {
        Ok(SafeDec(SafeInt::decode_ext(reader, dedupe_decoder)?))
    }
}

impl<const D: usize> Neg for SafeDec<D> {
    type Output = SafeDec<D>;

    #[inline(always)]
    fn neg(self) -> SafeDec<D> {
        SafeDec(-self.0)
    }
}

impl<const D: usize> Neg for &SafeDec<D> {
    type Output = SafeDec<D>;

    #[inline(always)]
    fn neg(self) -> SafeDec<D> {
        SafeDec(-self.0.clone())
    }
}

macro_rules! impl_decimal_ops_for_primitive {
    ($prim:ty, rhs_value) => {
        impl<const D: usize> Add<SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).add(other.0))
            }
        }

        impl<const D: usize> Sub<SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).sub(other.0))
            }
        }

        impl<const D: usize> BitAnd<SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).bitand(other.0))
            }
        }

        impl<const D: usize> BitOr<SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).bitor(other.0))
            }
        }

        impl<const D: usize> BitXor<SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).bitxor(other.0))
            }
        }

        impl<const D: usize> Mul<SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn mul(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeInt::from(self).mul(other.0))
            }
        }
    };
    ($prim:ty, rhs_ref) => {
        impl<const D: usize> Add<&SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).add(other.0.clone()))
            }
        }

        impl<const D: usize> Sub<&SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).sub(other.0.clone()))
            }
        }

        impl<const D: usize> BitAnd<&SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).bitand(other.0.clone()))
            }
        }

        impl<const D: usize> BitOr<&SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).bitor(other.0.clone()))
            }
        }

        impl<const D: usize> BitXor<&SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).bitxor(other.0.clone()))
            }
        }

        impl<const D: usize> Mul<&SafeDec<D>> for $prim {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn mul(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeInt::from(self).mul(other.0.clone()))
            }
        }
    };
}

macro_rules! impl_decimal_div_for_primitive {
    ($prim:ty, rhs_value) => {
        impl<const D: usize> Div<SafeDec<D>> for $prim {
            type Output = Option<SafeDec<D>>;

            #[inline(always)]
            fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
                Some(SafeDec(
                    SafeDec::<D>::scale_up(&SafeInt::from(self)).div(other.0)?,
                ))
            }
        }
    };
    ($prim:ty, rhs_ref) => {
        impl<const D: usize> Div<&SafeDec<D>> for $prim {
            type Output = Option<SafeDec<D>>;

            #[inline(always)]
            fn div(self, other: &SafeDec<D>) -> Option<SafeDec<D>> {
                Some(SafeDec(
                    SafeDec::<D>::scale_up(&SafeInt::from(self)).div(other.0.clone())?,
                ))
            }
        }
    };
}

macro_rules! for_each_primitive {
    ($macro:ident, $rhs_kind:ident) => {
        $macro!(u8, $rhs_kind);
        $macro!(u16, $rhs_kind);
        $macro!(u32, $rhs_kind);
        $macro!(u64, $rhs_kind);
        $macro!(u128, $rhs_kind);
        $macro!(i8, $rhs_kind);
        $macro!(i16, $rhs_kind);
        $macro!(i32, $rhs_kind);
        $macro!(i64, $rhs_kind);
        $macro!(i128, $rhs_kind);
        $macro!(usize, $rhs_kind);
        $macro!(isize, $rhs_kind);
    };
}

macro_rules! impl_decimal_ops_for_safe_dec {
    (lhs_value, rhs_value) => {
        impl<const D: usize> Add<SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.add(other.0))
            }
        }

        impl<const D: usize> Sub<SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.sub(other.0))
            }
        }

        impl<const D: usize> BitAnd<SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.bitand(other.0))
            }
        }

        impl<const D: usize> BitOr<SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.bitor(other.0))
            }
        }

        impl<const D: usize> BitXor<SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.bitxor(other.0))
            }
        }
    };
    (lhs_ref, rhs_value) => {
        impl<const D: usize> Add<SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.add(other.0))
            }
        }

        impl<const D: usize> Sub<SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.sub(other.0))
            }
        }

        impl<const D: usize> BitAnd<SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.bitand(other.0))
            }
        }

        impl<const D: usize> BitOr<SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.bitor(other.0))
            }
        }

        impl<const D: usize> BitXor<SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.bitxor(other.0))
            }
        }
    };
    (lhs_value, rhs_ref) => {
        impl<const D: usize> Add<&SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.add(other.0.clone()))
            }
        }

        impl<const D: usize> Sub<&SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.sub(other.0.clone()))
            }
        }

        impl<const D: usize> BitAnd<&SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.bitand(other.0.clone()))
            }
        }

        impl<const D: usize> BitOr<&SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.bitor(other.0.clone()))
            }
        }

        impl<const D: usize> BitXor<&SafeDec<D>> for SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.0.bitxor(other.0.clone()))
            }
        }
    };
    (lhs_ref, rhs_ref) => {
        impl<const D: usize> Add<&SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.add(other.0.clone()))
            }
        }

        impl<const D: usize> Sub<&SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.sub(other.0.clone()))
            }
        }

        impl<const D: usize> BitAnd<&SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.bitand(other.0.clone()))
            }
        }

        impl<const D: usize> BitOr<&SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.bitor(other.0.clone()))
            }
        }

        impl<const D: usize> BitXor<&SafeDec<D>> for &SafeDec<D> {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.clone().0.bitxor(other.0.clone()))
            }
        }
    };
}

macro_rules! impl_decimal_ops_for_safe_int {
    (lhs_value, rhs_value) => {
        impl<const D: usize> Add<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).add(other.0))
            }
        }

        impl<const D: usize> Sub<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).sub(other.0))
            }
        }

        impl<const D: usize> BitAnd<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).bitand(other.0))
            }
        }

        impl<const D: usize> BitOr<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).bitor(other.0))
            }
        }

        impl<const D: usize> BitXor<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).bitxor(other.0))
            }
        }

        impl<const D: usize> Mul<SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn mul(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.mul(other.0))
            }
        }
    };
    (lhs_ref, rhs_value) => {
        impl<const D: usize> Add<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).add(other.0))
            }
        }

        impl<const D: usize> Sub<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).sub(other.0))
            }
        }

        impl<const D: usize> BitAnd<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).bitand(other.0))
            }
        }

        impl<const D: usize> BitOr<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).bitor(other.0))
            }
        }

        impl<const D: usize> BitXor<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).bitxor(other.0))
            }
        }

        impl<const D: usize> Mul<SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn mul(self, other: SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.mul(other.0))
            }
        }
    };
    (lhs_value, rhs_ref) => {
        impl<const D: usize> Add<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).add(other.0.clone()))
            }
        }

        impl<const D: usize> Sub<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).sub(other.0.clone()))
            }
        }

        impl<const D: usize> BitAnd<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).bitand(other.0.clone()))
            }
        }

        impl<const D: usize> BitOr<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).bitor(other.0.clone()))
            }
        }

        impl<const D: usize> BitXor<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(&self).bitxor(other.0.clone()))
            }
        }

        impl<const D: usize> Mul<&SafeDec<D>> for SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn mul(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.mul(other.0.clone()))
            }
        }
    };
    (lhs_ref, rhs_ref) => {
        impl<const D: usize> Add<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn add(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).add(other.0.clone()))
            }
        }

        impl<const D: usize> Sub<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn sub(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).sub(other.0.clone()))
            }
        }

        impl<const D: usize> BitAnd<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitand(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).bitand(other.0.clone()))
            }
        }

        impl<const D: usize> BitOr<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).bitor(other.0.clone()))
            }
        }

        impl<const D: usize> BitXor<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn bitxor(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(SafeDec::<D>::scale_up(self).bitxor(other.0.clone()))
            }
        }

        impl<const D: usize> Mul<&SafeDec<D>> for &SafeInt {
            type Output = SafeDec<D>;

            #[inline(always)]
            fn mul(self, other: &SafeDec<D>) -> SafeDec<D> {
                SafeDec(self.mul(other.0.clone()))
            }
        }
    };
}

for_each_primitive!(impl_decimal_ops_for_primitive, rhs_value);
for_each_primitive!(impl_decimal_ops_for_primitive, rhs_ref);
for_each_primitive!(impl_decimal_div_for_primitive, rhs_value);
for_each_primitive!(impl_decimal_div_for_primitive, rhs_ref);

impl_decimal_ops_for_safe_dec!(lhs_value, rhs_value);
impl_decimal_ops_for_safe_dec!(lhs_ref, rhs_value);
impl_decimal_ops_for_safe_dec!(lhs_value, rhs_ref);
impl_decimal_ops_for_safe_dec!(lhs_ref, rhs_ref);

impl_decimal_ops_for_safe_int!(lhs_value, rhs_value);
impl_decimal_ops_for_safe_int!(lhs_ref, rhs_value);
impl_decimal_ops_for_safe_int!(lhs_value, rhs_ref);
impl_decimal_ops_for_safe_int!(lhs_ref, rhs_ref);

impl<const D: usize> Mul for SafeDec<D> {
    type Output = SafeDec<D>;

    fn mul(self, rhs: Self) -> Self::Output {
        let raw_product = self.0 * rhs.0;
        let scaled = SafeDec::<D>::scale_down(&raw_product);
        SafeDec(scaled)
    }
}

impl<const D: usize> Div<SafeDec<D>> for SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(
            SafeDec::<D>::scale_up(&self.0).div(other.0.clone())?,
        ))
    }
}

impl<const D: usize> Div<&SafeDec<D>> for SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(
            SafeDec::<D>::scale_up(&self.0).div(other.0.clone())?,
        ))
    }
}

impl<const D: usize> Div<SafeDec<D>> for &SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(
            SafeDec::<D>::scale_up(&self.0).div(other.0.clone())?,
        ))
    }
}

impl<const D: usize> Div<&SafeDec<D>> for &SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(
            SafeDec::<D>::scale_up(&self.0).div(other.0.clone())?,
        ))
    }
}

impl<const D: usize> Div<SafeInt> for SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeDec<D>> {
        Some(SafeDec(self.0.div(other)?))
    }
}

impl<const D: usize> Div<&SafeInt> for SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeDec<D>> {
        Some(SafeDec(self.0.div(other.clone())?))
    }
}

impl<const D: usize> Div<SafeInt> for &SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeDec<D>> {
        Some(SafeDec(self.0.clone().div(other)?))
    }
}

impl<const D: usize> Div<&SafeInt> for &SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeDec<D>> {
        Some(SafeDec(self.0.clone().div(other.clone())?))
    }
}

impl<const D: usize> Div<SafeDec<D>> for SafeInt {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(self.div(other.0)?))
    }
}

impl<const D: usize> Div<&SafeDec<D>> for SafeInt {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(self.div(other.0.clone())?))
    }
}

impl<const D: usize> Div<SafeDec<D>> for &SafeInt {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(self.clone().div(other.0)?))
    }
}

impl<const D: usize> Div<&SafeDec<D>> for &SafeInt {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeDec<D>> {
        Some(SafeDec(self.clone().div(other.0.clone())?))
    }
}

impl<const D: usize, O> AddAssign<O> for SafeDec<D>
where
    SafeDec<D>: Add<O, Output = SafeDec<D>>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: O) {
        *self = self.clone() + rhs;
    }
}

impl<const D: usize, O> MulAssign<O> for SafeDec<D>
where
    SafeDec<D>: Mul<O, Output = SafeDec<D>>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: O) {
        *self = self.clone() * rhs;
    }
}

impl<const D: usize, O> SubAssign<O> for SafeDec<D>
where
    SafeDec<D>: Sub<O, Output = SafeDec<D>>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: O) {
        *self = self.clone() - rhs;
    }
}

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
    let c = a.clone() + b;
    assert_eq!(c.0, SafeInt::from(123456 + 654321));
    assert_eq!(c.to_string().as_str(), "777.777");
    assert_eq!(c, SafeDec::from_raw(777777));
    let mut d = c;
    d += a;
    assert_eq!(d.to_string().as_str(), "901.233");
}

#[test]
fn test_safe_dec_sub() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = a.clone() - b;
    assert_eq!(c.0, SafeInt::from(123456 - 654321));
    assert_eq!(c.to_string().as_str(), "-530.865");
    assert_eq!(c, SafeDec::from_raw(-530865));
    let mut d = c;
    d -= a;
    assert_eq!(d.to_string().as_str(), "-654.321");
}

#[test]
fn test_safe_dec_mul() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = a.clone() * b;
    assert_eq!(c.to_string().as_str(), "80779.853");
    let mut d = c;
    d *= a;
    assert_eq!(d.to_string().as_str(), "9972757.531");
    let a = 4u64;
    let b = "-247.842".parse::<SafeDec<3>>().unwrap();
    let c = a * b;
    assert_eq!(c.to_string().as_str(), "-991.368");
}

#[test]
fn test_safe_dec_div() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = b / a;
    assert_eq!(c.unwrap().to_string().as_str(), "5.300");
}

#[test]
fn test_safe_dec_safe_int_div() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = SafeInt::from(654321);
    let c = b / a;
    assert_eq!(c, Some(SafeDec::from_raw(654321 / 123456)));
}

#[test]
fn test_from_other_scale() {
    let a = "123.455".parse::<SafeDec<3>>().unwrap();
    let b = SafeDec::<4>::from_other_scale(a);
    assert_eq!(b.to_string().as_str(), "123.4550");
    let a = "123.4550".parse::<SafeDec<4>>().unwrap();
    let b = SafeDec::<3>::from_other_scale(a);
    assert_eq!(b.to_string().as_str(), "123.455");
    let a = "123.456789".parse::<SafeDec<6>>().unwrap();
    let b = SafeDec::<3>::from_other_scale(a);
    assert_eq!(b.to_string().as_str(), "123.457");
    let a = "123.456789".parse::<SafeDec<6>>().unwrap();
    let b = SafeDec::<10>::from_other_scale(a);
    assert_eq!(b.to_string().as_str(), "123.4567890000");
}

#[test]
fn test_parsing_round_trip() {
    assert_eq!(
        "-0.000005587".parse::<SafeDec<10>>().unwrap().to_string(),
        "-0.0000055870"
    );
}

#[test]
fn test_complex() {
    let a = "-348973984.9879837849".parse::<SafeDec<10>>().unwrap();
    let b = "195.0000000001".parse::<SafeDec<10>>().unwrap();
    let c = b / a;
    assert_eq!(c.unwrap(), "-0.0000005587".parse::<SafeDec<10>>().unwrap());
    let a = "50.01".parse::<SafeDec<7>>().unwrap();
    assert_eq!(a.to_string(), "50.0100000");
    let b = "-2.9".parse::<SafeDec<7>>().unwrap();
    assert_eq!(b.to_string(), "-2.9000000");
    let c = a * b;
    assert_eq!(c.to_string(), "-145.0290000");
    let a = "33497398473894.1".parse::<SafeDec<10>>().unwrap();
    let b = "0.0000000001".parse::<SafeDec<10>>().unwrap();
    let c = a * b;
    assert_eq!(c.to_string(), "3349.7398473894");
    let a = "0.00005".parse::<SafeDec<5>>().unwrap();
    let b = "0.00001".parse::<SafeDec<5>>().unwrap();
    let c = a * b;
    assert_eq!(c.to_string(), "0.00000"); // because truncation
    let a = "0.00005".parse::<SafeDec<10>>().unwrap();
    let b = "0.00001".parse::<SafeDec<10>>().unwrap();
    let c = a * b;
    assert_eq!(c.to_string(), "0.0000000005");
    let a = SafeInt::from(5);
    let b = "0.00001".parse::<SafeDec<10>>().unwrap();
    let c = a * b;
    assert_eq!(c.to_string(), "0.0000500000");
}

#[test]
fn lencode_safe_dec_roundtrip_scale_2() {
    let values = [
        SafeDec::<2>::from_raw(0),
        SafeDec::<2>::from_raw(12345),
        SafeDec::<2>::from_raw(-9876),
        "99.99".parse::<SafeDec<2>>().unwrap(),
        "-0.01".parse::<SafeDec<2>>().unwrap(),
    ];

    for value in values {
        let mut buf = Vec::new();
        let written = value.encode(&mut buf).unwrap();
        assert_eq!(written, buf.len());
        let decoded = SafeDec::<2>::decode(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(decoded, value);
    }
}

#[test]
fn lencode_safe_dec_roundtrip_scale_6() {
    let values = [
        SafeDec::<6>::from_raw(0),
        SafeDec::<6>::from_raw(123_456_789),
        SafeDec::<6>::from_raw(-654_321_000),
        "0.000123".parse::<SafeDec<6>>().unwrap(),
        "-987.654321".parse::<SafeDec<6>>().unwrap(),
    ];

    for value in values {
        let mut buf = Vec::new();
        let written = value.encode(&mut buf).unwrap();
        assert_eq!(written, buf.len());
        let decoded = SafeDec::<6>::decode(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(decoded, value);
    }
}

#[test]
fn lencode_safe_dec_matches_safe_int_encoding() {
    let raw = SafeInt::from(-123_456i32);
    let dec = SafeDec::<4>::from_raw(raw.clone());

    let mut dec_buf = Vec::new();
    let mut int_buf = Vec::new();
    dec.encode(&mut dec_buf).unwrap();
    raw.encode(&mut int_buf).unwrap();

    assert_eq!(dec_buf, int_buf);
}


// TODO: Restore and fix the bug
// #[test]
// fn test_large_safe_dec_const_difference() {
//     let a = "123.455".parse::<SafeDec<3>>().unwrap();
//     let b = SafeDec::<400000>::from_other_scale(a);
//
//     assert_eq!(b.to_string().as_str(), "123.4550");
// }