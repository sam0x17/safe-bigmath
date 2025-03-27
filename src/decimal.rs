use crate::{SafeInt, parsing::ParsedSafeDec};
use core::{fmt::Display, ops::*, str::FromStr};
use eval_macro::eval;
use quoth::Parsable;
use rug::ops::NegAssign;

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

// impl<const D: usize> Div for SafeDec<D> {
//     type Output = Option<SafeDec<D>>;

//     fn div(self, rhs: Self) -> Self::Output {
//         if rhs.0.is_zero() {
//             None
//         } else {
//             Some(SafeDec(self.0.div(rhs.0).unwrap()))
//         }
//     }
// }

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
            for op in ["Add", "Sub", "Mul", "BitAnd", "BitOr", "BitXor"] {
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
                    "Mul" => 780000,
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
        }
        // SafeDec types
        for impl_type in ["SafeDec<D>", "&SafeDec<D>"] {
            for op in ["Add", "Sub", "Mul", "BitAnd", "BitOr", "BitXor"] {
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
                    "Mul" => 780,
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
            for op in ["Add", "Sub", "Mul", "BitAnd", "BitOr", "BitXor"] {
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
                    "Mul" => 780,
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

impl<const D: usize> Div<SafeDec<D>> for SafeDec<D> {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeInt> {
        self.0.div(other.0)
    }
}

impl<const D: usize> Div<&SafeDec<D>> for SafeDec<D> {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeInt> {
        self.0.div(other.0.clone())
    }
}

impl<const D: usize> Div<SafeDec<D>> for &SafeDec<D> {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeInt> {
        self.0.clone().div(other.0)
    }
}

impl<const D: usize> Div<&SafeDec<D>> for &SafeDec<D> {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeInt> {
        self.0.clone().div(other.0.clone())
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
    let c = a + b;
    assert_eq!(c.0, SafeInt::from(123456 + 654321));
    assert_eq!(c.to_string().as_str(), "777.777");
    assert_eq!(c, SafeDec::from_raw(777777));
}

#[test]
fn test_safe_dec_sub() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = a - b;
    assert_eq!(c.0, SafeInt::from(123456 - 654321));
    assert_eq!(c.to_string().as_str(), "-530.865");
    assert_eq!(c, SafeDec::from_raw(-530865));
}

#[test]
fn test_safe_dec_mul() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = a * b;
    assert_eq!(c.0, SafeInt::from(123456u64 * 654321u64));
    assert_eq!(c.to_string().as_str(), "80779853.376");
    assert_eq!(c, SafeDec::from_raw(80779853376u64));
}

#[test]
fn test_safe_dec_div() {
    let a = "123.456".parse::<SafeDec<3>>().unwrap();
    let b = "654.321".parse::<SafeDec<3>>().unwrap();
    let c = b / a;
    assert_eq!(c, Some(SafeInt::from(654321 / 123456)));
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
