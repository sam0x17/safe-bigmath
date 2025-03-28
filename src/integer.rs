use core::{cmp::Ordering, fmt::Display, ops::*, str::FromStr};
use quoth::Parsable;
use rug::{
    Integer,
    ops::{NegAssign, Pow},
};

use crate::parsing::ParsedSafeInt;

#[derive(Clone, Debug, Eq, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeInt(Integer);

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

pub static ONE: SafeInt = SafeInt(unsafe { Integer::from_raw(*Integer::ONE.as_raw()) });
pub static NEG_ONE: SafeInt = SafeInt(unsafe { Integer::from_raw(*Integer::NEG_ONE.as_raw()) });

impl SafeInt {
    pub const ZERO: SafeInt = SafeInt(Integer::ZERO);

    #[inline(always)]
    pub const fn raw(&self) -> &Integer {
        &self.0
    }

    #[inline(always)]
    pub const fn from_raw(value: Integer) -> SafeInt {
        SafeInt(value)
    }

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[inline(always)]
    pub const fn is_even(&self) -> bool {
        self.0.is_even()
    }

    #[inline(always)]
    pub const fn is_odd(&self) -> bool {
        self.0.is_odd()
    }

    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline(always)]
    pub fn abs(self) -> SafeInt {
        SafeInt(self.0.abs())
    }

    #[inline(always)]
    pub fn pow(self, exp: u32) -> SafeInt {
        SafeInt(self.0.pow(exp))
    }

    #[inline(always)]
    pub fn div_rem(self, other: SafeInt) -> (SafeInt, SafeInt) {
        let (div, rem) = self.0.div_rem(other.0);
        (SafeInt(div), SafeInt(rem))
    }

    #[inline(always)]
    pub fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    #[inline(always)]
    pub fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    #[inline(always)]
    pub fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    #[inline(always)]
    pub fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    #[inline(always)]
    pub fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    #[inline(always)]
    pub fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    #[inline(always)]
    pub fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    #[inline(always)]
    pub fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    #[inline(always)]
    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    #[inline(always)]
    pub fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    #[inline(always)]
    pub fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    #[inline(always)]
    pub fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    #[inline(always)]
    pub fn ceil_div(&self, b: SafeInt) -> Option<SafeInt> {
        let one = SafeInt::from(1);
        Some(((self - one.clone()) / b)? + one)
    }
}

impl Neg for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn neg(self) -> SafeInt {
        SafeInt(-self.0)
    }
}

impl NegAssign for SafeInt {
    #[inline(always)]
    fn neg_assign(&mut self) {
        self.0.neg_assign();
    }
}

impl Neg for &SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn neg(self) -> SafeInt {
        SafeInt(-self.0.clone())
    }
}

impl NegAssign for &mut SafeInt {
    #[inline(always)]
    fn neg_assign(&mut self) {
        self.0.neg_assign();
    }
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident) => {
        impl $trait for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(self.0.$method(other.0))
            }
        }

        impl $trait<&SafeInt> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(self.0.clone().$method(&other.0))
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

        impl $trait<SafeInt> for u8 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for u16 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for u32 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for u64 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for u128 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for i8 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for i16 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for i32 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for i64 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for i128 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for usize {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<SafeInt> for isize {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0))
            }
        }

        impl $trait<u8> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u8) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<u16> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u16) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<u32> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u32) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<u64> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u64) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<u128> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u128) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<i8> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i8) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<i16> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i16) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<i32> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i32) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<i64> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i64) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<i128> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i128) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<usize> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: usize) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<isize> for SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: isize) -> SafeInt {
                SafeInt(self.0.$method(other))
            }
        }

        impl $trait<&SafeInt> for u8 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for u16 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for u32 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for u64 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for u128 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for i8 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for i16 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for i32 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for i64 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for i128 {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for usize {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<&SafeInt> for isize {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: &SafeInt) -> SafeInt {
                SafeInt(Integer::from(self).$method(other.0.clone()))
            }
        }

        impl $trait<u8> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u8) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<u16> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u16) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<u32> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u32) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<u64> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u64) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<u128> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: u128) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<i8> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i8) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<i16> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i16) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<i32> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i32) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<i64> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i64) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<i128> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: i128) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<usize> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: usize) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }

        impl $trait<isize> for &SafeInt {
            type Output = SafeInt;

            #[inline(always)]
            fn $method(self, other: isize) -> SafeInt {
                SafeInt(self.0.clone().$method(other))
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident) => {
        impl $trait for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: SafeInt) {
                self.0.$method(other.0);
            }
        }

        impl $trait<&SafeInt> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: &SafeInt) {
                self.0.$method(&other.0);
            }
        }

        impl $trait<u8> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u8) {
                self.0.$method(other);
            }
        }

        impl $trait<u16> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u16) {
                self.0.$method(other);
            }
        }

        impl $trait<u32> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u32) {
                self.0.$method(other);
            }
        }

        impl $trait<u64> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u64) {
                self.0.$method(other);
            }
        }

        impl $trait<u128> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u128) {
                self.0.$method(other);
            }
        }

        impl $trait<i8> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i8) {
                self.0.$method(other);
            }
        }

        impl $trait<i16> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i16) {
                self.0.$method(other);
            }
        }

        impl $trait<i32> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i32) {
                self.0.$method(other);
            }
        }

        impl $trait<i64> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i64) {
                self.0.$method(other);
            }
        }

        impl $trait<i128> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i128) {
                self.0.$method(other);
            }
        }

        impl $trait<usize> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: usize) {
                self.0.$method(other);
            }
        }

        impl $trait<isize> for SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: isize) {
                self.0.$method(other);
            }
        }

        impl $trait<u8> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u8) {
                self.0.$method(other);
            }
        }

        impl $trait<u16> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u16) {
                self.0.$method(other);
            }
        }

        impl $trait<u32> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u32) {
                self.0.$method(other);
            }
        }

        impl $trait<u64> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u64) {
                self.0.$method(other);
            }
        }

        impl $trait<u128> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: u128) {
                self.0.$method(other);
            }
        }

        impl $trait<i8> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i8) {
                self.0.$method(other);
            }
        }

        impl $trait<i16> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i16) {
                self.0.$method(other);
            }
        }

        impl $trait<i32> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i32) {
                self.0.$method(other);
            }
        }

        impl $trait<i64> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i64) {
                self.0.$method(other);
            }
        }

        impl $trait<i128> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: i128) {
                self.0.$method(other);
            }
        }

        impl $trait<usize> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: usize) {
                self.0.$method(other);
            }
        }

        impl $trait<isize> for &mut SafeInt {
            #[inline(always)]
            fn $method(&mut self, other: isize) {
                self.0.$method(other);
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
impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(SubAssign, sub_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(RemAssign, rem_assign);
impl_assign_op!(BitAndAssign, bitand_assign);
impl_assign_op!(BitOrAssign, bitor_assign);
impl_assign_op!(BitXorAssign, bitxor_assign);

impl Div for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.0.div(other.0)))
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
            Some(SafeInt(self.0.div(&other.0)))
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
            Some(SafeInt(self.0.clone().div(other.0)))
        }
    }
}

impl Div<u8> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u8) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<u16> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u16) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<u32> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u32) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<u64> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u64) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<u128> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u128) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<i8> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i8) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<i16> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i16) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<i32> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i32) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<i64> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i64) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<i128> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i128) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<usize> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: usize) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<isize> for SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: isize) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.div(other)))
        }
    }
}

impl Div<SafeInt> for u8 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for u16 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for u32 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for u64 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for u128 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for i8 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for i16 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for i32 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for i64 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for i128 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for usize {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
        }
    }
}

impl Div<SafeInt> for isize {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0)))
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
            Some(SafeInt(self.0.clone().div(&other.0)))
        }
    }
}

impl Div<u8> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u8) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<u16> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u16) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<u32> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u32) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<u64> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u64) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<u128> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: u128) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<i8> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i8) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<i16> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i16) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<i32> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i32) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<i64> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i64) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<i128> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: i128) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<usize> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: usize) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<isize> for &SafeInt {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: isize) -> Option<SafeInt> {
        if other == 0 {
            None
        } else {
            Some(SafeInt(self.0.clone().div(other)))
        }
    }
}

impl Div<&SafeInt> for u8 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for u16 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for u32 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for u64 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for u128 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for i8 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for i16 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for i32 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for i64 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for i128 {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for usize {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl Div<&SafeInt> for isize {
    type Output = Option<SafeInt>;

    #[inline(always)]
    fn div(self, other: &SafeInt) -> Option<SafeInt> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeInt(self.div(other.0.clone())))
        }
    }
}

impl<T: PartialEq<Integer>> PartialEq<T> for SafeInt {
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        *other == self.0
    }
}

impl<T: PartialOrd<Integer>> PartialOrd<T> for SafeInt {
    #[inline(always)]
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        match other.partial_cmp(&self.0) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            equal => equal,
        }
    }
}

impl<T: Into<Integer>> From<T> for SafeInt {
    #[inline(always)]
    fn from(value: T) -> SafeInt {
        SafeInt(value.into())
    }
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
    assert!(SafeInt::from(10) != 20);
    assert!(SafeInt::from(37984739847983497938479797988798789783u128).is_odd());
    assert!(
        SafeInt::from_raw(
            Integer::from_str_radix(
                "3798473984798349793847979798879878978334738744739847983749837",
                10
            )
            .unwrap()
        ) > 10
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
    assert_eq!(&SafeInt::from(28249) / SafeInt::ZERO, None);
}

#[test]
fn test_zero() {
    assert_eq!(SafeInt::ZERO, 0);
    assert_eq!(SafeInt::ZERO.0, Integer::ZERO);
    assert!(SafeInt::ZERO.is_zero());
}

#[test]
fn test_one() {
    assert_eq!(ONE, 1);
    assert_eq!(ONE.0, *Integer::ONE);
    let one = ONE.clone();
    assert_eq!(one, 1);
}
