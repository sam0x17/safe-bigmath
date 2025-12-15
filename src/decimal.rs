use crate::{SafeInt, parsing::ParsedSafeDec};
use core::{fmt::Display, ops::*, str::FromStr};
use eval_macro::eval;
use quoth::Parsable;
use rug::ops::NegAssign;

/// Fixed-point decimal built on top of `SafeInt` with `D` fractional digits.
///
/// # Examples
/// ```
/// use safe_math::SafeDec;
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
    pub const ZERO: SafeDec<D> = SafeDec::<D>(SafeInt::ZERO);

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
    /// use safe_math::SafeDec;
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
/// use safe_math::SafeDec;
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
        let integer_part = (&abs_value / &divisor).unwrap_or(SafeInt::ZERO);
        let mut decimal_part = &abs_value % &divisor;

        if self.0.is_negative() {
            write!(f, "-")?;
        }

        write!(f, "{}", integer_part)?;
        write!(f, ".")?;

        // Extract D decimal digits
        let mut pow10 = (&divisor / &ten).unwrap(); // guaranteed non-zero
        for _ in 0..D {
            let (digit, rem) = decimal_part.div_rem(pow10.clone());
            write!(f, "{}", digit)?;
            decimal_part = rem;
            pow10 = (&pow10 / &ten).unwrap_or(1.into()); // last one goes to 1
        }

        Ok(())
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

impl<const D: usize> NegAssign for SafeDec<D> {
    #[inline(always)]
    fn neg_assign(&mut self) {
        self.0 = -self.0.clone();
    }
}

impl<const D: usize> NegAssign for &mut SafeDec<D> {
    #[inline(always)]
    fn neg_assign(&mut self) {
        self.0 = -self.0.clone();
    }
}

eval! {
    for self_type in ["SafeDec<D>","&SafeDec<D>"] {
        // integer primitives
        for impl_type in [
            "u8",
            "u16",
            "u32",
            "u64",
            "u128",
            "i8",
            "i16",
            "i32",
            "i64",
            "i128",
            "usize",
            "isize",
        ] {
            for op in ["Add", "Sub", "BitAnd", "BitOr", "BitXor"] {
                let method = op.to_lowercase();
                let maybe_clone = if self_type == "&SafeDec<D>" { ".clone()" } else { "" };
                let test_name = format!(
                    "test_{}_{}_{}",
                    op.to_lowercase(),
                    if self_type == "&SafeDec<D>" {
                        "safe_dec_ref"
                    } else {
                        "safe_dec"
                    },
                    impl_type.to_lowercase()
                );
                let expected_answer = match op {
                    "Add" => 65012,
                    "Sub" => 64988,
                    "BitXor" => 64996,
                    "BitOr" => 65004,
                    "BitAnd" => 8,
                    _ => unreachable!(),
                };
                output! {
                    impl<const D: usize> {{op}}<{{self_type}}> for {{impl_type}} {
                        type Output = SafeDec<D>;

                        #[inline(always)]
                        fn {{method}}(self, other: {{self_type}}) -> SafeDec<D> {
                            SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).{{method}}(other.0{{maybe_clone}}))
                        }
                    }

                    #[test]
                    fn {{test_name}}() {
                        let a = SafeDec::<3>::from_raw(12);
                        let b = {{impl_type}}::try_from(65).unwrap();
                        let c = b.{{method}}(a);
                        assert_eq!(c.0, SafeInt::from({{expected_answer}}));
                    }
                }
            }
            for op in ["Mul"] {
                let method = op.to_lowercase();
                let maybe_clone = if self_type == "&SafeDec<D>" { ".clone()" } else { "" };
                let test_name = format!(
                    "test_{}_{}_{}",
                    op.to_lowercase(),
                    if self_type == "&SafeDec<D>" {
                        "safe_dec_ref"
                    } else {
                        "safe_dec"
                    },
                    impl_type.to_lowercase()
                );
                let expected_answer = 780;
                output! {
                    impl<const D: usize> {{op}}<{{self_type}}> for {{impl_type}} {
                        type Output = SafeDec<D>;

                        #[inline(always)]
                        fn {{method}}(self, other: {{self_type}}) -> SafeDec<D> {
                            SafeDec(SafeInt::from(self).{{method}}(other.0{{maybe_clone}}))
                        }
                    }

                    #[test]
                    fn {{test_name}}() {
                        let a = SafeDec::<3>::from_raw(12);
                        let b = {{impl_type}}::try_from(65).unwrap();
                        let c = b.{{method}}(a);
                        assert_eq!(c.0, SafeInt::from({{expected_answer}}));
                    }
                }
            }
        }
        // SafeDec types linear
        for impl_type in ["SafeDec<D>", "&SafeDec<D>"] {
            for op in ["Add", "Sub", "BitAnd", "BitOr", "BitXor"] {
                let method = op.to_lowercase();
                let maybe_clone = if self_type == "&SafeDec<D>" { ".clone()" } else { "" };
                let impl_maybe_clone = if impl_type == "&SafeDec<D>" { ".clone()" } else { "" };
                let test_name = format!(
                    "test_{}_{}_{}",
                    op.to_lowercase(),
                    if self_type == "&SafeDec<D>" {
                        "safe_dec_ref"
                    } else {
                        "safe_dec"
                    },
                    if impl_type == "&SafeDec<D>" {
                        "safe_dec_ref"
                    } else {
                        "safe_dec"
                    }
                );
                let expected_answer = match op {
                    "Add" => 77,
                    "Sub" => 53,
                    "BitXor" => 77,
                    "BitOr" => 77,
                    "BitAnd" => 0,
                    _ => unreachable!(),
                };
                output! {
                    impl<const D: usize> {{op}}<{{self_type}}> for {{impl_type}} {
                        type Output = SafeDec<D>;

                        #[inline(always)]
                        fn {{method}}(self, other: {{self_type}}) -> SafeDec<D> {
                            SafeDec(self{{impl_maybe_clone}}.0.{{method}}(other.0{{maybe_clone}}))
                        }
                    }

                    #[test]
                    fn {{test_name}}() {
                        let a = SafeDec::<3>::from_raw(12);
                        let b = SafeDec::<3>::from_raw(65);
                        let c = b.{{method}}(a);
                        assert_eq!(c.0, SafeInt::from({{expected_answer}}));
                    }
                }
            }
        }
        // SafeInt types
        for impl_type in ["SafeInt", "&SafeInt"] {
            for op in ["Add", "Sub", "BitAnd", "BitOr", "BitXor"] {
                let method = op.to_lowercase();
                let maybe_clone = if self_type == "&SafeDec<D>" { ".clone()" } else { "" };
                let test_name = format!(
                    "test_{}_{}_{}",
                    op.to_lowercase(),
                    if self_type == "&SafeDec<D>" {
                        "safe_dec_ref"
                    } else {
                        "safe_dec"
                    },
                    if impl_type == "&SafeInt" {
                        "safe_int_ref"
                    } else {
                        "safe_int"
                    }
                );
                let expected_answer = match op {
                    "Add" => 77,
                    "Sub" => 53,
                    "BitXor" => 77,
                    "BitOr" => 77,
                    "BitAnd" => 0,
                    _ => unreachable!(),
                };
                output! {
                    impl<const D: usize> {{op}}<{{self_type}}> for {{impl_type}} {
                        type Output = SafeDec<D>;

                        #[inline(always)]
                        fn {{method}}(self, other: {{self_type}}) -> SafeDec<D> {
                            SafeDec(SafeDec::<D>::scale_up(&self).{{method}}(other.0{{maybe_clone}}))
                        }
                    }

                    #[test]
                    fn {{test_name}}() {
                        let a = SafeDec::<3>::from_raw(12);
                        let b = SafeDec::<3>::from_raw(65);
                        let c = b.{{method}}(a);
                        assert_eq!(c.0, SafeInt::from({{expected_answer}}));
                    }
                }
            }
            for op in ["Mul"] {
                let method = op.to_lowercase();
                let maybe_clone = if self_type == "&SafeDec<D>" { ".clone()" } else { "" };
                let test_name = format!(
                    "test_{}_{}_{}",
                    op.to_lowercase(),
                    if self_type == "&SafeDec<D>" {
                        "safe_dec_ref"
                    } else {
                        "safe_dec"
                    },
                    if impl_type == "&SafeInt" {
                        "safe_int_ref"
                    } else {
                        "safe_int"
                    }
                );
                let expected_answer = 0;
                output! {
                    impl<const D: usize> {{op}}<{{self_type}}> for {{impl_type}} {
                        type Output = SafeDec<D>;

                        #[inline(always)]
                        fn {{method}}(self, other: {{self_type}}) -> SafeDec<D> {
                            SafeDec(self.{{method}}(other.0{{maybe_clone}}))
                        }
                    }

                    #[test]
                    fn {{test_name}}() {
                        let a = SafeDec::<3>::from_raw(12);
                        let b = SafeDec::<3>::from_raw(65);
                        let c = b.{{method}}(a);
                        assert_eq!(c.0, SafeInt::from({{expected_answer}}));
                    }
                }
            }
        }
    }
}

eval! {
    for self_type in ["SafeDec<D>","&SafeDec<D>"] {
        // integer primitives
        for impl_type in [
            "u8",
            "u16",
            "u32",
            "u64",
            "u128",
            "i8",
            "i16",
            "i32",
            "i64",
            "i128",
            "usize",
            "isize",
        ] {
            let maybe_clone = if self_type == "&SafeDec<D>" { ".clone()" } else { "" };
            let test_name = format!(
                "test_div_{}_{}",
                if self_type == "&SafeDec<D>" {
                    "safe_dec_ref"
                } else {
                    "safe_dec"
                },
                impl_type.to_lowercase()
            );
            output! {
                impl<const D: usize> Div<{{self_type}}> for {{impl_type}} {
                    type Output = Option<SafeDec<D>>;

                    #[inline(always)]
                    fn div(self, other: {{self_type}}) -> Option<SafeDec<D>> {
                        Some(SafeDec(SafeDec::<D>::scale_up(&SafeInt::from(self)).div(other.0{{maybe_clone}})?))
                    }
                }

                #[test]
                fn {{test_name}}() {
                    let a = SafeDec::<3>::from_raw(12);
                    let b = {{impl_type}}::try_from(65).unwrap();
                    let c = b.div(a).unwrap();
                    assert_eq!(c.0, SafeInt::from(5416));
                }
            }
        }
    }
}

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
