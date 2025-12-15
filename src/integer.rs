extern crate alloc;

use core::{cmp::Ordering, fmt::Display, ops::*, str::FromStr};
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use quoth::Parsable;

#[cfg(test)]
use alloc::format;

use crate::parsing::ParsedSafeInt;

/// Arbitrary-precision integer wrapper that exposes safe, non-panicking operations.
/// 
/// # Examples
/// Create values from primitives and perform safe division (returns `Option` to avoid panics):
/// ```
/// use safe_math::SafeInt;
///
/// let a = SafeInt::from(10);
/// let b = SafeInt::from(3);
/// assert_eq!((&a / &b).unwrap(), SafeInt::from(3));
/// assert_eq!(&a + &b, SafeInt::from(13));
/// assert_eq!(SafeInt::from(5) / SafeInt::from(0), None);
/// ```
#[derive(Clone, Debug, Eq, Ord, Hash, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SafeInt(BigInt);

impl FromStr for SafeInt {
    type Err = quoth::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = quoth::ParseStream::from(s);
        let parsed = ParsedSafeInt::parse(&mut stream)?;
        Ok(parsed.value)
    }
}

impl Display for SafeInt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SafeInt {
    /// Zero value.
    pub fn zero() -> SafeInt {
        SafeInt(BigInt::zero())
    }
    /// One value.
    pub fn one() -> SafeInt {
        SafeInt(BigInt::one())
    }
    /// Negative one value.
    pub fn neg_one() -> SafeInt {
        -SafeInt::one()
    }
    /// Constant one value as a compile-time byte representation.
    pub const ONE: ConstSafeInt<2> = ConstSafeInt::from_bytes([0, 1]);
    /// Constant negative one value as a compile-time byte representation.
    pub const NEG_ONE: ConstSafeInt<2> = ConstSafeInt::from_bytes([1, 1]);

    /// Returns the underlying `BigInt` reference.
    #[inline(always)]
    pub const fn raw(&self) -> &BigInt {
        &self.0
    }

    /// Constructs a `SafeInt` from a raw `BigInt`.
    #[inline(always)]
    pub const fn from_raw(value: BigInt) -> SafeInt {
        SafeInt(value)
    }

    /// Returns `true` if the value is negative.
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        self.0.sign() == Sign::Minus
    }

    /// Returns `true` if the value is evenly divisible by 2.
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        self.0.is_even()
    }

    /// Returns `true` if the value is not evenly divisible by 2.
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        self.0.is_odd()
    }

    /// Returns `true` if the value is exactly zero.
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns the absolute value.
    #[inline(always)]
    pub fn abs(self) -> SafeInt {
        SafeInt(self.0.abs())
    }

    /// Raises the number to an unsigned integer power.
    #[inline(always)]
    pub fn pow(self, exp: u32) -> SafeInt {
        SafeInt(self.0.pow(exp))
    }

    /// Computes quotient and remainder simultaneously.
    #[inline(always)]
    pub fn div_rem(self, other: SafeInt) -> (SafeInt, SafeInt) {
        let (div, rem) = self.0.div_rem(&other.0);
        (SafeInt(div), SafeInt(rem))
    }

    /// Converts to `u8` if the value fits.
    #[inline(always)]
    pub fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    /// Converts to `u16` if the value fits.
    #[inline(always)]
    pub fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    /// Converts to `u32` if the value fits.
    #[inline(always)]
    pub fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    /// Converts to `u64` if the value fits.
    #[inline(always)]
    pub fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    /// Converts to `u128` if the value fits.
    #[inline(always)]
    pub fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    /// Converts to `i8` if the value fits.
    #[inline(always)]
    pub fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    /// Converts to `i16` if the value fits.
    #[inline(always)]
    pub fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    /// Converts to `i32` if the value fits.
    #[inline(always)]
    pub fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    /// Converts to `i64` if the value fits.
    #[inline(always)]
    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    /// Converts to `i128` if the value fits.
    #[inline(always)]
    pub fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    /// Converts to `usize` if the value fits.
    #[inline(always)]
    pub fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    /// Converts to `isize` if the value fits.
    #[inline(always)]
    pub fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    /// Performs integer ceiling division (`self / b`, rounded up).
    #[inline(always)]
    pub fn ceil_div(&self, b: SafeInt) -> Option<SafeInt> {
        let one = SafeInt::from(1);
        Some(((self - one.clone()) / b)? + one)
    }

    /// Computes `(base_numerator / base_denominator)^(exponent_numerator / exponent_denominator)`
    /// scaled by the provided factor. Returns `None` if the base or exponent denominator is zero
    /// or if the base is non-positive.
    ///
    /// # Examples
    /// ```rust
    /// use safe_math::SafeInt;
    ///
    /// // (2 / 3) ^ (1 / 2) * 1000 (approx) => 816 when floored
    /// let result = SafeInt::pow_ratio_scaled(
    ///     &SafeInt::from(2),
    ///     &SafeInt::from(3),
    ///     &SafeInt::from(1),
    ///     &SafeInt::from(2),
    ///     0,
    ///     &SafeInt::from(1_000),
    /// )
    /// .unwrap();
    /// assert_eq!(result, SafeInt::from(816));
    /// ```
    pub fn pow_ratio_scaled(
        base_numerator: &SafeInt,
        base_denominator: &SafeInt,
        exponent_numerator: &SafeInt,
        exponent_denominator: &SafeInt,
        _precision: u32,
        scale: &SafeInt,
    ) -> Option<SafeInt> {
        if base_denominator.is_zero() || exponent_denominator.is_zero() {
            return None;
        }
        if base_numerator.is_zero() {
            return Some(SafeInt::zero());
        }
        if scale.is_negative() {
            return None;
        }
        // Restrict to positive base
        if base_numerator.is_negative() || base_denominator.is_negative() {
            return None;
        }

        let base_num = base_numerator.0.to_biguint()?;
        let base_den = base_denominator.0.to_biguint()?;
        let mut exp_num = exponent_numerator.0.to_biguint()?;
        let mut exp_den = exponent_denominator.0.to_biguint()?;

        if exp_num.is_zero() {
            return Some(scale.clone());
        }

        let g = gcd_biguint(exp_num.clone(), exp_den.clone());
        if g > BigUint::one() {
            exp_num /= g.clone();
            exp_den /= g;
        }

        let exp_num_u32 = exp_num.to_u32()?;
        let exp_den_u32 = exp_den.to_u32()?;
        if exp_den_u32 == 0 {
            return None;
        }

        let scale_abs = scale.0.to_biguint()?;

        let base_num_pow = base_num.pow(exp_num_u32);
        let base_den_pow = base_den.pow(exp_num_u32);
        let scale_pow = scale_abs.pow(exp_den_u32);

        let target_num = base_num_pow * scale_pow;
        let target_den = base_den_pow;

        let root = nth_root_ratio_floor(&target_num, &target_den, exp_den_u32);
        Some(SafeInt(BigInt::from_biguint(Sign::Plus, root)))
    }
}

fn gcd_biguint(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let r = &a % &b;
        a = b;
        b = r;
    }
    a
}

fn pow_biguint(base: &BigUint, exp: u32) -> BigUint {
    base.pow(exp)
}

fn nth_root_ratio_floor(target_num: &BigUint, target_den: &BigUint, q: u32) -> BigUint {
    if q == 0 {
        return BigUint::zero();
    }

    let mut low = BigUint::zero();
    let mut high = BigUint::one();
    while pow_biguint(&high, q) * target_den <= *target_num {
        high <<= 1;
    }

    while low < high {
        let mid = (&low + &high + 1u32) >> 1;
        if pow_biguint(&mid, q) * target_den <= *target_num {
            low = mid;
        } else {
            high = mid - BigUint::one();
        }
    }

    low
}

impl Neg for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn neg(self) -> SafeInt {
        SafeInt(-self.0)
    }
}

impl Neg for &SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn neg(self) -> SafeInt {
        SafeInt(-self.0.clone())
    }
}

macro_rules! impl_pair_ops {
    ($trait:ident, $method:ident) => {
        impl $trait for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(self.0.$method(other.0))
            }
        }

        impl $trait<&SafeInt> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(self.0.$method(&other.0))
            }
        }

        impl $trait<SafeInt> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(self.0.clone().$method(other.0))
            }
        }

        impl $trait<&SafeInt> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(self.0.clone().$method(&other.0))
            }
        }
    };
}

impl_pair_ops!(Add, add);
impl_pair_ops!(Sub, sub);
impl_pair_ops!(Mul, mul);
impl_pair_ops!(Rem, rem);
impl_pair_ops!(BitAnd, bitand);
impl_pair_ops!(BitOr, bitor);
impl_pair_ops!(BitXor, bitxor);

macro_rules! impl_prim_ops {
    ($trait:ident, $method:ident, [$($t:ty),*]) => {
        $(
            impl $trait<$t> for SafeInt {
                type Output = SafeInt;

                #[inline(always)]
                fn $method(self, other: $t) -> SafeInt {
                    SafeInt(self.0.$method(BigInt::from(other)))
                }
            }

            impl $trait<$t> for &SafeInt {
                type Output = SafeInt;

                #[inline(always)]
                fn $method(self, other: $t) -> SafeInt {
                    SafeInt(self.0.clone().$method(BigInt::from(other)))
                }
            }

            impl $trait<SafeInt> for $t {
                type Output = SafeInt;

                #[inline(always)]
                fn $method(self, other: SafeInt) -> SafeInt {
                    SafeInt(BigInt::from(self).$method(other.0))
                }
            }

            impl $trait<&SafeInt> for $t {
                type Output = SafeInt;

                #[inline(always)]
                fn $method(self, other: &SafeInt) -> SafeInt {
                    SafeInt(BigInt::from(self).$method(other.0.clone()))
                }
            }
        )*
    };
}

impl_prim_ops!(Add, add, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_prim_ops!(Sub, sub, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_prim_ops!(Mul, mul, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_prim_ops!(Rem, rem, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_prim_ops!(BitAnd, bitand, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_prim_ops!(BitOr, bitor, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_prim_ops!(BitXor, bitxor, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);

impl AddAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn add_assign(&mut self, rhs: SafeInt) {
        self.0 += rhs.0;
    }
}

impl AddAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &SafeInt) {
        self.0 += &rhs.0;
    }
}

impl SubAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: SafeInt) {
        self.0 -= rhs.0;
    }
}

impl SubAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &SafeInt) {
        self.0 -= &rhs.0;
    }
}

impl MulAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: SafeInt) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &SafeInt) {
        self.0 *= &rhs.0;
    }
}

impl RemAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: SafeInt) {
        self.0 %= rhs.0;
    }
}

impl RemAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: &SafeInt) {
        self.0 %= &rhs.0;
    }
}

impl BitAndAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: SafeInt) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &SafeInt) {
        self.0 &= &rhs.0;
    }
}

impl BitOrAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: SafeInt) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &SafeInt) {
        self.0 |= &rhs.0;
    }
}

impl BitXorAssign<SafeInt> for SafeInt {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: SafeInt) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &SafeInt) {
        self.0 ^= &rhs.0;
    }
}

macro_rules! impl_assign_prim {
    ($trait:ident, $method:ident, $op:tt, [$($t:ty),*]) => {
        $(
            impl $trait<$t> for SafeInt {
                #[inline(always)]
                fn $method(&mut self, rhs: $t) {
                    self.0 $op BigInt::from(rhs);
                }
            }
        )*
    };
}

impl_assign_prim!(AddAssign, add_assign, +=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(SubAssign, sub_assign, -=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(MulAssign, mul_assign, *=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(RemAssign, rem_assign, %=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(BitAndAssign, bitand_assign, &=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(BitOrAssign, bitor_assign, |=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(BitXorAssign, bitxor_assign, ^=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);

impl Div for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.0 / other.0))
        }
    }
}

impl Div<&SafeInt> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.0 / &other.0))
        }
    }
}

impl Div<SafeInt> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.0.clone() / other.0))
        }
    }
}

impl Div<&SafeInt> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.0.clone() / &other.0))
        }
    }
}

macro_rules! impl_div_safeint_rhs_prim {
    ($($t:ty),*) => {
        $(
            impl Div<$t> for SafeInt {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn div(self, other: $t) -> Option<SafeInt> {
                    if other == 0 {
                        None
                    } else {
                        Some(SafeInt(self.0 / BigInt::from(other)))
                    }
                }
            }

            impl Div<$t> for &SafeInt {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn div(self, other: $t) -> Option<SafeInt> {
                    if other == 0 {
                        None
                    } else {
                        Some(SafeInt(self.0.clone() / BigInt::from(other)))
                    }
                }
            }
        )*
    };
}

macro_rules! impl_div_prim_lhs_safeint {
    ($($t:ty),*) => {
        $(
            impl Div<SafeInt> for $t {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn div(self, other: SafeInt) -> Option<SafeInt> {
                    if other.0.is_zero() {
                        None
                    } else {
                        Some(SafeInt(BigInt::from(self) / other.0))
                    }
                }
            }

            impl Div<&SafeInt> for $t {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn div(self, other: &SafeInt) -> Option<SafeInt> {
                    if other.0.is_zero() {
                        None
                    } else {
                        Some(SafeInt(BigInt::from(self) / other.0.clone()))
                    }
                }
            }
        )*
    };
}

impl_div_safeint_rhs_prim!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_div_prim_lhs_safeint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Into<BigInt>> From<T> for SafeInt {
    #[inline(always)]
    fn from(value: T) -> SafeInt {
        SafeInt(value.into())
    }
}

impl<T> PartialEq<T> for SafeInt
where
    T: Copy,
    BigInt: From<T>,
{
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        self.0 == BigInt::from(*other)
    }
}

impl<T> PartialOrd<T> for SafeInt
where
    T: Copy,
    BigInt: From<T>,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&BigInt::from(*other))
    }
}

macro_rules! impl_prim_cmp {
    ($($t:ty),*) => {
        $(
            impl PartialEq<SafeInt> for $t {
                #[inline(always)]
                fn eq(&self, other: &SafeInt) -> bool {
                    BigInt::from(*self) == other.0
                }
            }

            impl PartialOrd<SafeInt> for $t {
                #[inline(always)]
                fn partial_cmp(&self, other: &SafeInt) -> Option<Ordering> {
                    BigInt::from(*self).partial_cmp(&other.0)
                }
            }
        )*
    };
}

impl_prim_cmp!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// Fixed-size, byte-backed integer that can be converted into `SafeInt`.
///
/// # Examples
/// ```
/// use safe_math::integer::ConstSafeInt;
/// use safe_math::SafeInt;
///
/// const ONE: ConstSafeInt<2> = ConstSafeInt::from_bytes([0, 1]);
/// assert_eq!(SafeInt::from(ONE), SafeInt::from(1));
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct ConstSafeInt<const N: usize>([u8; N]);

impl<const N: usize> ConstSafeInt<N> {
    /// Creates a constant from big-endian two's-complement bytes, where the first byte encodes the sign.
    pub const fn from_bytes(value: [u8; N]) -> Self {
        Self(value)
    }

    /// Returns the underlying bytes.
    pub const fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }

    /// Converts into a runtime `SafeInt`.
    pub fn to_val(self) -> SafeInt {
        self.into()
    }
}

impl ConstSafeInt<17> {
    /// Builds a 17-byte representation from an `i128` while preserving the sign bit.
    ///
    /// # Examples
    /// ```
    /// use safe_math::integer::ConstSafeInt;
    /// use safe_math::SafeInt;
    ///
    /// const NEG_ONE: ConstSafeInt<17> = ConstSafeInt::from_i128(-1);
    /// assert_eq!(SafeInt::from(NEG_ONE), SafeInt::from(-1));
    /// ```
    pub const fn from_i128(value: i128) -> Self {
        let is_neg = value < 0;
        let value = if value == i128::MIN {
            i128::MAX as u128 + 1
        } else {
            value.unsigned_abs()
        };
        let mut res = Self::from_u128(value);
        if is_neg {
            res.0[0] = 1;
        }
        res
    }
    /// Builds a 17-byte representation from an unsigned 128-bit value.
    pub const fn from_u128(value: u128) -> Self {
        let mut res = [0; 17];
        let mut value = value;
        let mut i = 17;
        while i > 0 {
            i -= 1;
            res[i] = (value & 0xff) as u8;
            value >>= 8;
        }
        Self(res)
    }
}

impl<const N: usize> From<ConstSafeInt<N>> for SafeInt {
    #[inline(always)]
    fn from(value: ConstSafeInt<N>) -> SafeInt {
        let pos = value.0.first().cloned().unwrap_or(0) == 0;
        let magnitude = BigUint::from_bytes_be(&value.0[1..]);
        let mut res = SafeInt(BigInt::from_biguint(Sign::Plus, magnitude));
        if !pos {
            res = -res;
        }
        res
    }
}

impl<const N: usize> From<&ConstSafeInt<N>> for SafeInt {
    #[inline(always)]
    fn from(value: &ConstSafeInt<N>) -> SafeInt {
        let pos = value.0.first().cloned().unwrap_or(0) == 0;
        let magnitude = BigUint::from_bytes_be(&value.0[1..]);
        let mut res = SafeInt(BigInt::from_biguint(Sign::Plus, magnitude));
        if !pos {
            res = -res;
        }
        res
    }
}

#[test]
fn test_const_safe_int() {
    assert_eq!(
        SafeInt::from(ConstSafeInt::<4>::from_bytes([0, 0, 0, 1])),
        1
    );
    assert_eq!(SafeInt::from(ConstSafeInt::<2>::from_bytes([0, 1])), 1);
    assert_eq!(SafeInt::from(ConstSafeInt::<2>::from_bytes([1, 1])), -1);
    assert_eq!(SafeInt::from(ConstSafeInt::<2>::from_bytes([1, 0])), -0);
    assert_eq!(
        SafeInt::from(ConstSafeInt::<3>::from_bytes([1, 5, 254])),
        -1534
    );
    assert_eq!(
        SafeInt::from(ConstSafeInt::<17>::from_i128(-538525)),
        -538525
    );
    assert_eq!(
        SafeInt::from(ConstSafeInt::<17>::from_i128(123456789)),
        123456789
    );
    assert_eq!(
        SafeInt::from(ConstSafeInt::<17>::from_i128(i128::MIN)),
        i128::MIN
    );
    assert_eq!(
        SafeInt::from(ConstSafeInt::<17>::from_i128(i128::MAX)),
        i128::MAX
    );
    assert_eq!(
        SafeInt::from(ConstSafeInt::<17>::from_u128(u128::MAX)),
        u128::MAX
    );
    assert_eq!(
        SafeInt::from(ConstSafeInt::<17>::from_u128(39874398749837343434343434344)),
        39874398749837343434343434344u128
    );
    assert_eq!(SafeInt::from(ConstSafeInt::<17>::from_u128(0)), 0);
}

#[test]
fn general() {
    let a = SafeInt::from(10);
    let b = SafeInt::from(20);
    let c = &a + &b;
    let d = a.clone() + c.clone();
    let e = a.clone() + &b;
    let f = &a + b.clone();
    assert_eq!(c, 30);
    assert!(d > a);
    assert!(a < d);
    assert!(a < b);
    assert_eq!(e, f);
    assert_eq!(f, a + b);
    assert_eq!((SafeInt::from(10) / SafeInt::from(3)).unwrap(), 3);
    assert_eq!(SafeInt::from(10) / SafeInt::from(0), None);
    assert_ne!(SafeInt::from(10), SafeInt::from(20));
    assert!(SafeInt::from(37984739847983497938479797988798789783u128).is_odd());
    assert!(
        SafeInt::from_str(
            "3798473984798349793847979798879878978334738744739847983749837"
        )
        .unwrap()
            > 10
    );
    assert_eq!(
        SafeInt::from(33) / SafeInt::from(3),
        Some(SafeInt::from(11))
    );
    assert_eq!(33 / SafeInt::from(3), Some(SafeInt::from(11)));
    assert_eq!(SafeInt::from(33) / 3, Some(SafeInt::from(11)));
    assert_eq!(33 + SafeInt::from(2), 35);
    assert_eq!(SafeInt::from(33) + 2, 35);
    assert_eq!(SafeInt::from(5) / SafeInt::from(0), None);
    assert_eq!(5 / SafeInt::from(0), None);
    assert_eq!(SafeInt::from(5) / 0, None);
    assert_eq!(&SafeInt::from(789) / 893797983, Some(SafeInt::from(0)));
    assert_eq!(&SafeInt::from(28249) / SafeInt::zero(), None);
}

#[test]
fn test_perquintill_power() {
    const PRECISION: u32 = 256;
    const PERQUINTILL: u128 = 1_000_000_000_000_000_000;

    let x = SafeInt::from(21_000_000_000_000_000u64);
    let delta = SafeInt::from(7_000_000_000_000_000u64);
    let w1 = SafeInt::from(600_000_000_000_000_000u128);
    let w2 = SafeInt::from(400_000_000_000_000_000u128);
    let denominator = &x + &delta;
    assert_eq!(w1.clone() + w2.clone(), SafeInt::from(PERQUINTILL));

    let perquintill_result = SafeInt::pow_ratio_scaled(
        &x,
        &denominator,
        &w1,
        &w2,
        PRECISION,
        &SafeInt::from(PERQUINTILL),
    )
    .expect("perquintill integer result");

    assert_eq!(perquintill_result, SafeInt::from(649_519_052_838_328_985u128));
    let readable = crate::SafeDec::<18>::from_raw(perquintill_result);
    assert_eq!(format!("{}", readable), "0.649519052838328985");
}

#[test]
fn test_zero() {
    assert_eq!(SafeInt::zero(), 0);
    assert!(SafeInt::zero().is_zero());
}

#[test]
fn test_one() {
    let one = SafeInt::one();
    assert_eq!(one, 1);
}
