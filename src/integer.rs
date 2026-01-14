extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec;
use core::{cmp::Ordering, fmt::Display, ops::*, str::FromStr};
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use quoth::Parsable;

#[cfg(test)]
use alloc::format;
use alloc::vec::Vec;
use lencode::dedupe::{DedupeDecoder, DedupeEncoder};
#[cfg(test)]
use lencode::io::Cursor;
use lencode::io::{Error, Read, Write};
use lencode::{Decode, Encode};
#[cfg(test)]
use std::time::{Duration, Instant};

use crate::parsing::ParsedSafeInt;

/// Arbitrary-precision integer wrapper that exposes safe, non-panicking operations.
///
/// # Examples
/// Create values from primitives and perform safe division (returns `Option` to avoid panics):
/// ```
/// use safe_bigmath::SafeInt;
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

/// Default iteration cap for the fixed-point approximation used by `pow_ratio_scaled` when
/// large exponents require the fallback path.
pub const DEFAULT_MAX_ITERS: usize = 4_096;
const MAX_EXACT_EXPONENT: u32 = 1_024;

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
    /// Returns `None` if `other` is zero.
    #[inline(always)]
    pub fn div_rem(self, other: SafeInt) -> Option<(SafeInt, SafeInt)> {
        if other.0.is_zero() {
            None
        } else {
            let (div, rem) = self.0.div_rem(&other.0);
            Some((SafeInt(div), SafeInt(rem)))
        }
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
    /// or if the base is non-positive. Uses an exact integer path when the exponent fits in 32
    /// bits and is below `MAX_EXACT_EXPONENT`, and falls back to a fixed-point approximation with
    /// the requested precision otherwise.
    /// The fallback uses `DEFAULT_MAX_ITERS` as its iteration cap and stops earlier once rounded
    /// digits converge.
    ///
    /// # Examples
    /// ```rust
    /// use safe_bigmath::SafeInt;
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
        precision: u32,
        scale: &SafeInt,
    ) -> Option<SafeInt> {
        Self::pow_ratio_scaled_with_max_iters(
            base_numerator,
            base_denominator,
            exponent_numerator,
            exponent_denominator,
            precision,
            scale,
            None,
        )
    }

    /// Same as [`pow_ratio_scaled`] but allows specifying a maximum iteration cap for the
    /// fixed-point approximation path. When `None`, `DEFAULT_MAX_ITERS` is used. The approximation
    /// still exits early when rounded digits converge.
    pub fn pow_ratio_scaled_with_max_iters(
        base_numerator: &SafeInt,
        base_denominator: &SafeInt,
        exponent_numerator: &SafeInt,
        exponent_denominator: &SafeInt,
        precision: u32,
        scale: &SafeInt,
        max_iters: Option<usize>,
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

        let scale_abs = scale.0.to_biguint()?;
        let scale_bits = u32::try_from(scale_abs.bits()).unwrap_or(u32::MAX);

        let exp_num_bits = exp_num.bits();
        let exp_den_bits = exp_den.bits();
        if exp_num_bits <= 32 && exp_den_bits <= 32 {
            let exp_num_u32 = exp_num.to_u32()?;
            let exp_den_u32 = exp_den.to_u32()?;
            if exp_den_u32 == 0 {
                return None;
            }

            if exp_num_u32 <= MAX_EXACT_EXPONENT && exp_den_u32 <= MAX_EXACT_EXPONENT {
                let base_num_pow = base_num.pow(exp_num_u32);
                let base_den_pow = base_den.pow(exp_num_u32);
                let scale_pow = scale_abs.pow(exp_den_u32);

                let target_num = base_num_pow * scale_pow;
                let target_den = base_den_pow;

                let root = nth_root_ratio_floor(&target_num, &target_den, exp_den_u32);
                return Some(SafeInt(BigInt::from_biguint(Sign::Plus, root)));
            }
        }

        // Fallback path for large exponents: approximate using fixed-point log/exp with guard bits.
        // Allow arbitrarily high requested precision (callers can cap via `max_iters`); enforce
        // only a reasonable floor to keep the series stable.
        // Increase the minimum precision based on the magnitude of `scale` so that even when
        // callers request coarse precision, we retain enough fractional bits to keep the final
        // scaled integer accurate.
        let requested_precision = precision.max(32).max(scale_bits.saturating_add(8));
        let guard_bits: u32 = 24;
        let internal_precision = requested_precision.saturating_add(guard_bits);
        let default_max_iters = DEFAULT_MAX_ITERS.min(internal_precision as usize + 128);
        let max_iters = max_iters.unwrap_or(default_max_iters).max(1);

        let target_scale_uint = BigUint::one() << requested_precision;
        let guard_factor_uint = BigUint::one() << guard_bits;
        let internal_scale_uint = &target_scale_uint << guard_bits;

        let target_scale = BigInt::from_biguint(Sign::Plus, target_scale_uint.clone());
        let guard_factor = BigInt::from_biguint(Sign::Plus, guard_factor_uint.clone());
        let internal_scale = BigInt::from_biguint(Sign::Plus, internal_scale_uint.clone());

        let ln_half = ln1p_fixed(
            &(-(&internal_scale >> 1usize)),
            &internal_scale,
            &guard_factor,
            max_iters,
        );
        let ln_two = -ln_half;

        // Compute ln(base_num) - ln(base_den) using normalized mantissas near 1.0 for better
        // convergence, regardless of how small or large the ratio is.
        let ln_num = ln_biguint(
            &base_num,
            internal_precision,
            &internal_scale_uint,
            &internal_scale,
            &guard_factor,
            &ln_two,
            max_iters,
        );
        let ln_den = ln_biguint(
            &base_den,
            internal_precision,
            &internal_scale_uint,
            &internal_scale,
            &guard_factor,
            &ln_two,
            max_iters,
        );
        let ln_base = ln_num - ln_den;

        let ln_scaled = (ln_base * BigInt::from_biguint(Sign::Plus, exp_num))
            .div_floor(&BigInt::from_biguint(Sign::Plus, exp_den));
        let exp_fp = exp_fixed(&ln_scaled, &internal_scale, &guard_factor, max_iters);
        let exp_requested = round_to_precision(&exp_fp, &guard_factor);
        let result =
            (exp_requested * BigInt::from_biguint(Sign::Plus, scale_abs)).div_floor(&target_scale);

        Some(SafeInt(result))
    }

    /// Exponentiate base to exponent. Base can be large integer number betwen 0 and u64::MAX
    /// Optimal exponent values are between 0.1 and 0.9
    pub fn pow_bigint_base(
        base: &SafeInt,
        exponent_numerator: &SafeInt,
        exponent_denominator: &SafeInt,
        precision: u32,
        scale: &SafeInt,
    ) -> Option<SafeInt> {
        Self::pow_bigint_base_scaled_with_max_iters(
            base,
            exponent_numerator,
            exponent_denominator,
            precision,
            scale,
            None,
        )
    }

    fn pow_bigint_base_scaled_with_max_iters(
        base: &SafeInt,
        exponent_numerator: &SafeInt,
        exponent_denominator: &SafeInt,
        precision: u32,
        scale: &SafeInt,
        max_iters: Option<usize>,
    ) -> Option<SafeInt> {
        use num_bigint::{BigInt, BigUint, Sign};
        use num_integer::Integer;
        use num_traits::{One, ToPrimitive, Zero};

        // Guard rails
        if exponent_denominator.is_zero() {
            return None;
        }
        if base.is_zero() {
            // Keep the same semantics as pow_ratio_scaled_with_max_iters:
            // 0^anything -> 0 (including 0^0).
            return Some(SafeInt::zero());
        }
        if scale.is_negative() {
            return None;
        }
        // Restrict to positive base
        if base.is_negative() {
            return None;
        }

        // base is an integer; convert to BigUint
        let base_uint = base.0.to_biguint()?;

        let mut exp_num = exponent_numerator.0.to_biguint()?;
        let mut exp_den = exponent_denominator.0.to_biguint()?;

        if exp_num.is_zero() {
            // base^0 ~= 1, scaled by `scale`
            return Some(scale.clone());
        }

        // Reduce exponent fraction exp_num/exp_den
        let g = gcd_biguint(exp_num.clone(), exp_den.clone());
        if g > BigUint::one() {
            exp_num /= g.clone();
            exp_den /= g;
        }

        let scale_abs = scale.0.to_biguint()?;
        let scale_bits = u32::try_from(scale_abs.bits()).unwrap_or(u32::MAX);

        // ---- Fast path: small rational exponent, exact pow/root on integers ----
        let exp_num_bits = exp_num.bits();
        let exp_den_bits = exp_den.bits();
        if exp_num_bits <= 32 && exp_den_bits <= 32 {
            let exp_num_u32 = exp_num.to_u32()?;
            let exp_den_u32 = exp_den.to_u32()?;
            if exp_den_u32 == 0 {
                return None;
            }

            if exp_num_u32 <= MAX_EXACT_EXPONENT && exp_den_u32 <= MAX_EXACT_EXPONENT{
                // base^(exp_num/exp_den) * scale
                //
                // Compute:
                //   (base^exp_num * scale^exp_den)^(1/exp_den)
                // via nth_root_ratio_floor, same as in ratio version but with
                // denominator fixed to 1.
                let base_pow = base_uint.pow(exp_num_u32);
                let scale_pow = scale_abs.pow(exp_den_u32);

                let target_num = base_pow * scale_pow;
                let target_den = BigUint::one();

                let root = nth_root_ratio_floor(&target_num, &target_den, exp_den_u32);
                return Some(SafeInt(BigInt::from_biguint(Sign::Plus, root)));
            }
        }

        // ---- Fallback path: fixed-point log/exp with guard bits ----

        // Same heuristic as in pow_ratio_scaled_with_max_iters:
        let requested_precision = precision.max(32).max(scale_bits.saturating_add(8));
        let guard_bits: u32 = 24;
        let internal_precision = requested_precision.saturating_add(guard_bits);
        let default_max_iters = DEFAULT_MAX_ITERS.min(internal_precision as usize + 128);
        let max_iters = max_iters.unwrap_or(default_max_iters).max(1);

        let target_scale_uint = BigUint::one() << requested_precision;
        let guard_factor_uint = BigUint::one() << guard_bits;
        let internal_scale_uint = &target_scale_uint << guard_bits;

        let target_scale = BigInt::from_biguint(Sign::Plus, target_scale_uint.clone());
        let guard_factor = BigInt::from_biguint(Sign::Plus, guard_factor_uint.clone());
        let internal_scale = BigInt::from_biguint(Sign::Plus, internal_scale_uint.clone());

        // ln(2) via ln1p_fixed(-1/2)
        let ln_half = ln1p_fixed(
            &(-(&internal_scale >> 1usize)),
            &internal_scale,
            &guard_factor,
            max_iters,
        );
        let ln_two = -ln_half;

        // ln(base) using normalized mantissa (same helper as ratio version)
        let ln_base = ln_biguint(
            &base_uint,
            internal_precision,
            &internal_scale_uint,
            &internal_scale,
            &guard_factor,
            &ln_two,
            max_iters,
        );

        // ln(base) * exp_num / exp_den
        let ln_scaled = (ln_base * BigInt::from_biguint(Sign::Plus, exp_num))
            .div_floor(&BigInt::from_biguint(Sign::Plus, exp_den));

        // exp(ln_scaled) in fixed-point
        let exp_fp = exp_fixed(&ln_scaled, &internal_scale, &guard_factor, max_iters);

        // Drop guard bits
        let exp_requested = round_to_precision(&exp_fp, &guard_factor);

        // Scale by `scale_abs` and rescale to integer
        let result =
            (exp_requested * BigInt::from_biguint(Sign::Plus, scale_abs)).div_floor(&target_scale);

        Some(SafeInt(result))
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

fn ln1p_fixed(x_fp: &BigInt, scale: &BigInt, guard_factor: &BigInt, max_iters: usize) -> BigInt {
    // Fixed-point natural log using the Taylor series for ln(1 + x), stopping once the rounded
    // value at the target precision stops changing or the incremental term is below guard bits.
    let mut term = x_fp.clone();
    let mut result = term.clone();
    let mut prev_rounded = round_to_precision(&result, guard_factor);
    for n in 2..=max_iters {
        term = (&term * x_fp).div_floor(scale);
        if term.is_zero() {
            break;
        }

        let next = term.div_floor(&BigInt::from(n as u32));
        if next.is_zero() {
            break;
        }

        if n % 2 == 0 {
            result -= &next;
        } else {
            result += &next;
        }

        let rounded = round_to_precision(&result, guard_factor);
        if next.abs() < guard_factor.abs() || rounded == prev_rounded {
            break;
        }
        prev_rounded = rounded;
    }

    result
}

fn exp_fixed(x_fp: &BigInt, scale: &BigInt, guard_factor: &BigInt, max_iters: usize) -> BigInt {
    // Fixed-point exponential using the Taylor series for exp(x), stopping once rounded digits
    // stabilize at the target precision or the incremental term is below guard bits.
    let mut term = scale.clone(); // 1.0 in fixed-point space
    let mut result = term.clone();
    let mut prev_rounded = round_to_precision(&result, guard_factor);
    for n in 1..=max_iters {
        term = (&term * x_fp).div_floor(&(scale * BigInt::from(n as u32)));
        if term.is_zero() {
            break;
        }
        result += &term;

        let rounded = round_to_precision(&result, guard_factor);
        if term.abs() < guard_factor.abs() || rounded == prev_rounded {
            break;
        }
        prev_rounded = rounded;
    }

    result
}

fn ln_biguint(
    value: &BigUint,
    internal_precision: u32,
    internal_scale_uint: &BigUint,
    internal_scale: &BigInt,
    guard_factor: &BigInt,
    ln_two: &BigInt,
    max_iters: usize,
) -> BigInt {
    debug_assert!(!value.is_zero());
    if value.is_zero() {
        return BigInt::zero();
    }

    let int_prec = internal_precision as usize;
    let mut shift = value.bits().saturating_sub(1);
    let mut mantissa = value.clone() << int_prec;
    mantissa >>= shift;

    // Keep the mantissa close to 1.0 (in [0.5, 1.5)) for fast ln1p convergence.
    let half_scale = internal_scale_uint >> 1;
    let scale_plus_half = internal_scale_uint + &half_scale;
    if mantissa >= scale_plus_half {
        mantissa >>= 1;
        shift = shift.saturating_add(1);
    }

    let mantissa_int = BigInt::from_biguint(Sign::Plus, mantissa);
    let ln_mantissa = ln1p_fixed(
        &(mantissa_int - internal_scale),
        internal_scale,
        guard_factor,
        max_iters,
    );

    ln_mantissa + ln_two * BigInt::from(shift)
}

fn round_to_precision(value: &BigInt, guard_factor: &BigInt) -> BigInt {
    let (mut truncated, remainder) = value.div_rem(guard_factor);
    if !remainder.is_zero() && (remainder.abs() << 1) >= guard_factor.abs() {
        truncated += remainder.signum();
    }
    truncated
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

macro_rules! impl_pair_rem_ops {
    () => {
        impl Rem for SafeInt {
            type Output = Option<SafeInt>;

            #[inline(always)]
            fn rem(self, other: SafeInt) -> Option<SafeInt> {
                if other.0.is_zero() {
                    None
                } else {
                    Some(SafeInt(self.0 % other.0))
                }
            }
        }

        impl Rem<&SafeInt> for SafeInt {
            type Output = Option<SafeInt>;

            #[inline(always)]
            fn rem(self, other: &SafeInt) -> Option<SafeInt> {
                if other.0.is_zero() {
                    None
                } else {
                    Some(SafeInt(self.0 % &other.0))
                }
            }
        }

        impl Rem<SafeInt> for &SafeInt {
            type Output = Option<SafeInt>;

            #[inline(always)]
            fn rem(self, other: SafeInt) -> Option<SafeInt> {
                if other.0.is_zero() {
                    None
                } else {
                    Some(SafeInt(self.0.clone() % other.0))
                }
            }
        }

        impl Rem<&SafeInt> for &SafeInt {
            type Output = Option<SafeInt>;

            #[inline(always)]
            fn rem(self, other: &SafeInt) -> Option<SafeInt> {
                if other.0.is_zero() {
                    None
                } else {
                    Some(SafeInt(self.0.clone() % &other.0))
                }
            }
        }
    };
}

impl_pair_ops!(Add, add);
impl_pair_ops!(Sub, sub);
impl_pair_ops!(Mul, mul);
impl_pair_rem_ops!();
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

macro_rules! impl_prim_rem_ops {
    ($($t:ty),*) => {
        $(
            impl Rem<$t> for SafeInt {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn rem(self, other: $t) -> Option<SafeInt> {
                    if other == 0 {
                        None
                    } else {
                        Some(SafeInt(self.0 % BigInt::from(other)))
                    }
                }
            }

            impl Rem<$t> for &SafeInt {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn rem(self, other: $t) -> Option<SafeInt> {
                    if other == 0 {
                        None
                    } else {
                        Some(SafeInt(self.0.clone() % BigInt::from(other)))
                    }
                }
            }

            impl Rem<SafeInt> for $t {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn rem(self, other: SafeInt) -> Option<SafeInt> {
                    if other.0.is_zero() {
                        None
                    } else {
                        Some(SafeInt(BigInt::from(self) % other.0))
                    }
                }
            }

            impl Rem<&SafeInt> for $t {
                type Output = Option<SafeInt>;

                #[inline(always)]
                fn rem(self, other: &SafeInt) -> Option<SafeInt> {
                    if other.0.is_zero() {
                        None
                    } else {
                        Some(SafeInt(BigInt::from(self) % other.0.clone()))
                    }
                }
            }
        )*
    };
}

impl_prim_ops!(
    Add,
    add,
    [
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    ]
);
impl_prim_ops!(
    Sub,
    sub,
    [
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    ]
);
impl_prim_ops!(
    Mul,
    mul,
    [
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    ]
);
impl_prim_rem_ops!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl_prim_ops!(
    BitAnd,
    bitand,
    [
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    ]
);
impl_prim_ops!(
    BitOr,
    bitor,
    [
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    ]
);
impl_prim_ops!(
    BitXor,
    bitxor,
    [
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    ]
);

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
        if !rhs.0.is_zero() {
            self.0 %= rhs.0;
        }
    }
}

impl RemAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: &SafeInt) {
        if !rhs.0.is_zero() {
            self.0 %= &rhs.0;
        }
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

macro_rules! impl_rem_assign_prim {
    ($($t:ty),*) => {
        $(
            impl RemAssign<$t> for SafeInt {
                #[inline(always)]
                fn rem_assign(&mut self, rhs: $t) {
                    if rhs != 0 {
                        self.0 %= BigInt::from(rhs);
                    }
                }
            }
        )*
    };
}

impl_assign_prim!(AddAssign, add_assign, +=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(SubAssign, sub_assign, -=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_assign_prim!(MulAssign, mul_assign, *=, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
impl_rem_assign_prim!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
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

impl_div_safeint_rhs_prim!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl_div_prim_lhs_safeint!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

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

impl_prim_cmp!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// Fixed-size, byte-backed integer that can be converted into `SafeInt`.
///
/// # Examples
/// ```
/// use safe_bigmath::integer::ConstSafeInt;
/// use safe_bigmath::SafeInt;
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
    /// use safe_bigmath::integer::ConstSafeInt;
    /// use safe_bigmath::SafeInt;
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

const LENCODE_SAFE_INT_VARIANT_MASK: u8 = 0x80;
const LENCODE_SAFE_INT_VARIANT_BYTES: u8 = 0x80;
const LENCODE_SAFE_INT_SIZE_MASK: u8 = 0x40;
const LENCODE_SAFE_INT_SIZE_LARGE: u8 = 0x40;
const LENCODE_SAFE_INT_PAYLOAD_MASK: u8 = 0x3F;
const LENCODE_SAFE_INT_SMALL_MAX: u8 = LENCODE_SAFE_INT_PAYLOAD_MASK;
const LENCODE_MAX_VARINT_BYTES: usize = LENCODE_SAFE_INT_PAYLOAD_MASK as usize;

#[inline(always)]
fn lencode_encode_biguint_varint_bytes(
    bytes: &[u8],
    writer: &mut impl Write,
) -> lencode::Result<usize> {
    if bytes.is_empty() {
        return writer.write(&[0u8]);
    }
    if bytes.len() == 1 && bytes[0] <= LENCODE_SAFE_INT_SMALL_MAX {
        return writer.write(&[bytes[0]]);
    }
    if bytes.len() > LENCODE_MAX_VARINT_BYTES {
        return Err(Error::IncorrectLength);
    }

    let mut total = 0;
    total += writer.write(&[LENCODE_SAFE_INT_SIZE_LARGE | (bytes.len() as u8)])?;
    total += writer.write(bytes)?;
    Ok(total)
}

#[inline(always)]
fn lencode_decode_biguint_varint_from_prefix(
    prefix: u8,
    reader: &mut impl Read,
) -> lencode::Result<BigUint> {
    if (prefix & LENCODE_SAFE_INT_SIZE_MASK) == 0 {
        return Ok(BigUint::from(prefix & LENCODE_SAFE_INT_PAYLOAD_MASK));
    }

    let len = (prefix & LENCODE_SAFE_INT_PAYLOAD_MASK) as usize;
    if len == 0 {
        return Err(Error::InvalidData);
    }

    let mut buf = vec![0u8; len];
    let mut read = 0usize;
    while read < len {
        let n = reader.read(&mut buf[read..])?;
        if n == 0 {
            return Err(Error::ReaderOutOfData);
        }
        read += n;
    }
    Ok(BigUint::from_bytes_le(&buf))
}

#[inline(always)]
fn lencode_encode_biguint_with_variant(
    value: &BigUint,
    writer: &mut impl Write,
) -> lencode::Result<usize> {
    let bytes = value.to_bytes_le();
    if bytes.len() <= LENCODE_MAX_VARINT_BYTES {
        return lencode_encode_biguint_varint_bytes(&bytes, writer);
    }

    let mut total = 0;
    total += writer.write(&[LENCODE_SAFE_INT_VARIANT_BYTES])?;
    total += bytes.encode(writer)?;
    Ok(total)
}

#[inline(always)]
fn lencode_decode_biguint_with_variant(reader: &mut impl Read) -> lencode::Result<BigUint> {
    let mut tag = [0u8; 1];
    if reader.read(&mut tag)? != 1 {
        return Err(Error::ReaderOutOfData);
    }

    if (tag[0] & LENCODE_SAFE_INT_VARIANT_MASK) != 0 {
        let bytes: Vec<u8> = Vec::decode(reader)?;
        Ok(BigUint::from_bytes_le(&bytes))
    } else {
        lencode_decode_biguint_varint_from_prefix(tag[0], reader)
    }
}

#[inline(always)]
fn lencode_zigzag_encode_bigint(value: &BigInt) -> BigUint {
    if value.is_negative() {
        let magnitude = (-value).to_biguint().unwrap_or(BigUint::ZERO);
        (magnitude << 1usize) - BigUint::from(1u8)
    } else {
        let magnitude = value.to_biguint().unwrap_or(BigUint::ZERO);
        magnitude << 1usize
    }
}

#[inline(always)]
fn lencode_zigzag_decode_biguint(value: BigUint) -> BigInt {
    if value.is_odd() {
        let magnitude = (value + BigUint::from(1u8)) >> 1usize;
        -BigInt::from(magnitude)
    } else {
        BigInt::from(value >> 1usize)
    }
}

impl Encode for SafeInt {
    #[inline(always)]
    fn encode_ext(
        &self,
        writer: &mut impl Write,
        _dedupe_encoder: Option<&mut DedupeEncoder>,
    ) -> lencode::Result<usize> {
        let encoded = lencode_zigzag_encode_bigint(&self.0);
        lencode_encode_biguint_with_variant(&encoded, writer)
    }
}

impl Decode for SafeInt {
    #[inline(always)]
    fn decode_ext(
        reader: &mut impl Read,
        _dedupe_decoder: Option<&mut DedupeDecoder>,
    ) -> lencode::Result<Self> {
        let unsigned = lencode_decode_biguint_with_variant(reader)?;
        Ok(SafeInt::from_raw(lencode_zigzag_decode_biguint(unsigned)))
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
        SafeInt::from_str("3798473984798349793847979798879878978334738744739847983749837").unwrap()
            > 10
    );
    assert_eq!(
        SafeInt::from(33) / SafeInt::from(3),
        Some(SafeInt::from(11))
    );
    assert_eq!(33 / SafeInt::from(3), Some(SafeInt::from(11)));
    assert_eq!(SafeInt::from(33) / 3, Some(SafeInt::from(11)));
    assert_eq!(SafeInt::from(10) % SafeInt::from(3), Some(SafeInt::from(1)));
    assert_eq!(SafeInt::from(10) % SafeInt::from(0), None);
    assert_eq!(10 % SafeInt::from(3), Some(SafeInt::from(1)));
    assert_eq!(10 % SafeInt::from(0), None);
    assert_eq!(SafeInt::from(10) % 3, Some(SafeInt::from(1)));
    assert_eq!(SafeInt::from(10) % 0, None);
    assert_eq!(
        SafeInt::from(10).div_rem(SafeInt::from(3)),
        Some((SafeInt::from(3), SafeInt::from(1)))
    );
    assert_eq!(SafeInt::from(10).div_rem(SafeInt::from(0)), None);
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

    assert_eq!(
        perquintill_result,
        SafeInt::from(649_519_052_838_328_985u128)
    );
    let readable = crate::SafeDec::<18>::from_raw(perquintill_result);
    assert_eq!(format!("{}", readable), "0.649519052838328985");
}

#[test]
fn pow_ratio_scaled_handles_large_weight_denominators() {
    let x = SafeInt::from(21_000_000_000_000_000i128);
    let denominator = SafeInt::from(21_000_000_000_000_100i128);
    let perquintill = SafeInt::from(1_000_000_000_000_000_000i128);

    let cases = [
        (
            SafeInt::from(500_000_000_000_000_000i128),
            SafeInt::from(500_000_000_000_000_000i128),
        ),
        (
            SafeInt::from(499_999_999_999_500_000i128),
            SafeInt::from(500_000_000_000_500_000i128),
        ),
        (
            SafeInt::from(500_000_000_000_250_000i128),
            SafeInt::from(499_999_999_999_750_000i128),
        ),
    ];

    for (w1, w2) in cases {
        let result =
            SafeInt::pow_ratio_scaled(&x, &denominator, &w1, &w2, 256, &perquintill).unwrap();
        assert_eq!(result, SafeInt::from(999_999_999_999_995_238i128));
    }
}

#[test]
fn pow_ratio_scaled_converges_on_boundary_weights() {
    let x = SafeInt::from(21_000_000_000_000_000i128);
    let denominator = SafeInt::from(21_000_000_000_000_001i128);
    let w1 = SafeInt::from(499_999_999_500_000_000i128);
    let w2 = SafeInt::from(500_000_000_500_000_000i128);
    let scale = SafeInt::from(1_000_000_000_000_000_000i128);
    let precision = 256u32;

    let start = std::time::Instant::now();
    let result = SafeInt::pow_ratio_scaled(&x, &denominator, &w1, &w2, precision, &scale).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(result, SafeInt::from(999_999_999_999_999_952i128));
    assert!(
        elapsed < core::time::Duration::from_secs(1),
        "pow_ratio_scaled took {:?}",
        elapsed
    );
}

#[test]
fn pow_ratio_scaled_exact_path_handles_high_exponent() {
    let base_num = SafeInt::from(999_999_999i128);
    let base_den = SafeInt::from(1_000_000_001i128);
    let exp_num = SafeInt::from(MAX_EXACT_EXPONENT as i128 - 1);
    let exp_den = SafeInt::one();
    let scale = SafeInt::from(1_000_000i128);
    let precision = 64u32;

    let start = std::time::Instant::now();
    let result =
        SafeInt::pow_ratio_scaled(&base_num, &base_den, &exp_num, &exp_den, precision, &scale)
            .expect("exact path should return");
    let elapsed = start.elapsed();

    assert!(result > SafeInt::zero());
    assert!(
        elapsed < core::time::Duration::from_secs(1),
        "exact path took {:?}",
        elapsed
    );
}

#[test]
fn pow_ratio_scaled_default_max_iters_completes_quickly() {
    let base_num = SafeInt::from(1i128);
    let base_den = SafeInt::from(1_000_000_000_000i128);
    let exp_num = SafeInt::one();
    let exp_den = SafeInt::from(1u128 << 40);
    let scale = SafeInt::one();
    let precision = 32u32;

    let start = std::time::Instant::now();
    let default_iter =
        SafeInt::pow_ratio_scaled(&base_num, &base_den, &exp_num, &exp_den, precision, &scale)
            .unwrap();
    let elapsed = start.elapsed();

    let explicit_default = SafeInt::pow_ratio_scaled_with_max_iters(
        &base_num,
        &base_den,
        &exp_num,
        &exp_den,
        precision,
        &scale,
        Some(DEFAULT_MAX_ITERS),
    )
    .unwrap();

    assert_eq!(default_iter, explicit_default);
    assert!(
        elapsed < core::time::Duration::from_secs(2),
        "default iterations took {:?}",
        elapsed
    );
}

#[test]
fn pow_ratio_scaled_uses_scale_to_pick_precision() {
    // Force the fallback path (large exponent bits) and make sure the minimum precision we pick
    // based on `scale` is close to a much higher requested precision.
    let base_num = SafeInt::from(123_456_789u64);
    let base_den = SafeInt::from(987_654_321u64);
    let exp_num = SafeInt::from(987_654_321_123_456_789u128);
    let exp_den = SafeInt::from(123_456_789_987_654_321u128);
    let scale = SafeInt::from(1_000_000_000_000_000_000i128);

    let coarse = SafeInt::pow_ratio_scaled(&base_num, &base_den, &exp_num, &exp_den, 0, &scale)
        .expect("coarse precision result");
    let precise = SafeInt::pow_ratio_scaled(&base_num, &base_den, &exp_num, &exp_den, 256, &scale)
        .expect("high precision result");
    let delta = (precise.clone() - coarse.clone()).abs();

    assert!(
        delta <= SafeInt::from(1u32),
        "coarse {coarse} vs precise {precise} differed by {delta}"
    );
}

#[test]
fn pow_ratio_scaled_handles_small_base_fractional_exponent() {
    // Base far from 1 with a fractional exponent; previously this would under-approximate badly.
    let base_num = SafeInt::from(1u8);
    let base_den = SafeInt::from(10u8);
    let exp_num = SafeInt::from(500_000_000_001u64);
    let exp_den = SafeInt::from(1_000_000_000_000u64);
    let scale = SafeInt::from(1_000_000_000_000_000_000u128);
    let precision = 128u32;

    let result =
        SafeInt::pow_ratio_scaled(&base_num, &base_den, &exp_num, &exp_den, precision, &scale)
            .expect("small base fractional exponent");
    let expected =
        ((0.1f64).powf(0.500000000001f64) * 1_000_000_000_000_000_000f64).floor() as u128;
    let delta = (result.clone() - SafeInt::from(expected)).abs();

    assert!(
        delta <= SafeInt::from(128u32),
        "result {result} vs expected {expected} (delta {delta})"
    );
}

#[test]
fn pow_ratio_scaled_handles_extreme_delta_x() {
    let x = SafeInt::from(400_775_553u64);
    let dx = SafeInt::from(14_446_633_907_665_582u64);
    let base_den = &x + &dx;
    let w1 = SafeInt::from(102_337_248_363_782_924u128);
    let w2 = SafeInt::from(1_000_000_000_000_000_000u128) - &w1;
    let scale = SafeInt::from(1_000_000_000_000_000_000u128);
    let precision = 256u32;

    let result = SafeInt::pow_ratio_scaled(&x, &base_den, &w1, &w2, precision, &scale)
        .expect("extreme delta x");

    let expected = ((x.0.to_f64().unwrap() / base_den.0.to_f64().unwrap())
        .powf(w1.0.to_f64().unwrap() / w2.0.to_f64().unwrap())
        * 1_000_000_000_000_000_000f64)
        .floor() as u128;
    let delta = (result.clone() - SafeInt::from(expected)).abs();

    assert!(
        delta <= SafeInt::from(1_000_000u128),
        "result {result} vs expected {expected} (delta {delta})"
    );
}

#[test]
fn pow_ratio_scaled_with_crafted_gcd_values() {
    /*
        w1_safe and w2_safe are picked with the following rules:
           - they don't have GCD > 1
           - w1_safe < 1024
           - bits(w2_safe) <= 32
     */
    let x_safe = SafeInt::from_str("2100000000000000000000000").unwrap();
    let denominator = SafeInt::from_str("210000000000000000000000").unwrap();
    let w1_safe = SafeInt::from_str("499").unwrap();
    let w2_safe = SafeInt::from_str("1538820023").unwrap();
    let precision = 256;
    let perquintill_scale = SafeInt::from_str("1000000000000000000").unwrap();

    let start = Instant::now();

    SafeInt::pow_ratio_scaled(
        &x_safe,
        &denominator,
        &w1_safe,
        &w2_safe,
        precision,
        &perquintill_scale,
    );
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_secs(1),
        "pow_ratio_scaled took {:?} (expected < 1s)",
        elapsed
    );
}

#[test]
fn pow_bigint_base_with_crafted_gcd_values() {
    /*
        w1_safe and w2_safe are picked with the following rules:
           - they don't have GCD > 1
           - w1_safe < 1024
           - bits(w2_safe) <= 32
     */
    let x_safe = SafeInt::from_str("2100000000000000000000000").unwrap();
    let w1_safe = SafeInt::from_str("499").unwrap();
    let w2_safe = SafeInt::from_str("1538820023").unwrap();
    let precision = 256;
    let perquintill_scale = SafeInt::from_str("1000000000000000000").unwrap();

    let start = Instant::now();

    SafeInt::pow_bigint_base(
        &x_safe,
        &w1_safe,
        &w2_safe,
        precision,
        &perquintill_scale,
    );
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_secs(1),
        "pow_ratio_scaled took {:?} (expected < 1s)",
        elapsed
    );
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

#[cfg(test)]
#[inline(always)]
fn expected_varint_bytes(value: &BigUint) -> Vec<u8> {
    if *value <= BigUint::from(LENCODE_SAFE_INT_SMALL_MAX) {
        vec![value.to_u8().expect("value fits in u8")]
    } else {
        let bytes = value.to_bytes_le();
        let mut out = Vec::with_capacity(1 + bytes.len());
        out.push(LENCODE_SAFE_INT_SIZE_LARGE | (bytes.len() as u8));
        out.extend_from_slice(&bytes);
        out
    }
}

#[test]
fn lencode_safe_int_roundtrip() {
    let big = BigInt::from(1u8) << 200usize;
    let values = [
        SafeInt::from(0),
        SafeInt::from(1),
        SafeInt::from(-1),
        SafeInt::from(127),
        SafeInt::from(128),
        SafeInt::from(-128),
        SafeInt::from(255),
        SafeInt::from(-255),
        SafeInt::from_raw(big.clone()),
        SafeInt::from_raw(-big),
    ];

    for value in values {
        let mut buf = Vec::new();
        let written = value.encode(&mut buf).unwrap();
        assert_eq!(written, buf.len());
        let decoded = SafeInt::decode(&mut Cursor::new(&buf)).unwrap();
        assert_eq!(decoded, value);
    }
}

#[test]
fn lencode_safe_int_known_encodings() {
    let cases: &[(i32, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x02]),
        (-1, &[0x01]),
        (63, &[0x41, 0x7E]),
        (-64, &[0x41, 0x7F]),
        (64, &[0x41, 0x80]),
        (128, &[0x42, 0x00, 0x01]),
        (-128, &[0x41, 0xFF]),
        (300, &[0x42, 0x58, 0x02]),
    ];

    for &(value, expected) in cases {
        let mut buf = Vec::new();
        let written = SafeInt::from(value).encode(&mut buf).unwrap();
        assert_eq!(written, buf.len());
        assert_eq!(buf, expected);
    }
}

#[test]
fn lencode_safe_int_large_encoding_structure() {
    let base = BigInt::from(1u8) << 200usize;
    let values = [
        SafeInt::from_raw(base.clone()),
        SafeInt::from_raw(base.clone() + BigInt::from(0x1234u32)),
        SafeInt::from_raw(-base),
    ];

    for value in values {
        let mut buf = Vec::new();
        value.encode(&mut buf).unwrap();

        let raw = value.raw();
        let zigzag = if raw.is_negative() {
            let magnitude = (-raw).to_biguint().expect("negative magnitude");
            (magnitude << 1usize) - BigUint::from(1u8)
        } else {
            let magnitude = raw.to_biguint().unwrap_or_else(|| BigUint::ZERO);
            magnitude << 1usize
        };
        let payload = zigzag.to_bytes_le();
        assert!(payload.len() > 1);
        assert_eq!(buf[0], LENCODE_SAFE_INT_SIZE_LARGE | (payload.len() as u8));
        assert_eq!(&buf[1..], payload.as_slice());
    }
}

#[test]
fn lencode_safe_int_matches_varint_bytes() {
    let values = [
        SafeInt::from(0),
        SafeInt::from(42),
        SafeInt::from(-42),
        SafeInt::from(1_000_000),
        SafeInt::from(-1_000_000),
    ];

    for value in values {
        let raw = value.raw();
        let zigzag = if raw.is_negative() {
            let magnitude = (-raw).to_biguint().expect("negative magnitude");
            (magnitude << 1usize) - BigUint::from(1u8)
        } else {
            let magnitude = raw.to_biguint().unwrap_or_else(|| BigUint::ZERO);
            magnitude << 1usize
        };
        let expected = expected_varint_bytes(&zigzag);

        let mut buf = Vec::new();
        value.encode(&mut buf).unwrap();
        assert_eq!(buf, expected);
    }
}

#[test]
fn lencode_safe_int_rejects_zero_length_prefix() {
    let data = [LENCODE_SAFE_INT_SIZE_LARGE];
    let err = SafeInt::decode(&mut Cursor::new(&data[..])).unwrap_err();
    assert!(matches!(err, Error::InvalidData));
}

#[test]
fn lencode_safe_int_rejects_truncated_payload() {
    let data = [LENCODE_SAFE_INT_SIZE_LARGE | 0x02u8, 0x01];
    let err = SafeInt::decode(&mut Cursor::new(&data[..])).unwrap_err();
    assert!(matches!(err, Error::ReaderOutOfData));
}

#[test]
fn lencode_safe_int_large_values_use_bytes_variant() {
    let too_large = BigInt::from(1u8) << (8 * LENCODE_MAX_VARINT_BYTES);
    let value = SafeInt::from_raw(too_large);
    let mut buf = Vec::new();
    value.encode(&mut buf).unwrap();

    assert_eq!(buf[0], LENCODE_SAFE_INT_VARIANT_BYTES);
    let bytes: Vec<u8> = Vec::decode(&mut Cursor::new(&buf[1..])).unwrap();
    let zigzag = lencode_zigzag_encode_bigint(value.raw());
    assert_eq!(bytes, zigzag.to_bytes_le());

    let decoded = SafeInt::decode(&mut Cursor::new(&buf)).unwrap();
    assert_eq!(decoded, value);
}
