use core::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use rug::{
    ops::{NegAssign, Pow},
    Integer,
};

#[derive(Clone, Debug, Eq, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeInt(Integer);

impl SafeInt {
    pub const ZERO: SafeInt = SafeInt(Integer::ZERO);

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
    };
}

impl_binary_op!(Add, add);
impl_binary_op!(Sub, sub);
impl_binary_op!(Mul, mul);
impl_binary_op!(Rem, rem);
impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(SubAssign, sub_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(RemAssign, rem_assign);

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
}
