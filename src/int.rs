use core::{
    cmp::Ordering,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use rug::{ops::NegAssign, Integer};

#[derive(Clone, Debug, Eq, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeInt(Integer);

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

impl Add for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn add(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0 + other.0)
    }
}

impl AddAssign for SafeInt {
    #[inline(always)]
    fn add_assign(&mut self, other: SafeInt) {
        self.0 += other.0;
    }
}

impl Sub for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn sub(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0 - other.0)
    }
}

impl SubAssign for SafeInt {
    #[inline(always)]
    fn sub_assign(&mut self, other: SafeInt) {
        self.0 -= other.0;
    }
}

impl Mul for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn mul(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0 * other.0)
    }
}

impl MulAssign for SafeInt {
    #[inline(always)]
    fn mul_assign(&mut self, other: SafeInt) {
        self.0 *= other.0;
    }
}

impl Rem for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn rem(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0 % other.0)
    }
}

impl RemAssign for SafeInt {
    #[inline(always)]
    fn rem_assign(&mut self, other: SafeInt) {
        self.0 %= other.0;
    }
}

impl<T: Into<Integer>> From<T> for SafeInt {
    #[inline(always)]
    fn from(value: T) -> Self {
        SafeInt(value.into())
    }
}

impl<T: Into<Integer>> Add<T> for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn add(self, other: T) -> SafeInt {
        SafeInt(self.0 + other.into())
    }
}

impl<T: Into<Integer>> AddAssign<T> for SafeInt {
    #[inline(always)]
    fn add_assign(&mut self, other: T) {
        self.0 += other.into();
    }
}

impl<T: Into<Integer>> Sub<T> for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn sub(self, other: T) -> SafeInt {
        SafeInt(self.0 - other.into())
    }
}

impl<T: Into<Integer>> SubAssign<T> for SafeInt {
    #[inline(always)]
    fn sub_assign(&mut self, other: T) {
        self.0 -= other.into();
    }
}

impl<T: Into<Integer>> Mul<T> for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn mul(self, other: T) -> SafeInt {
        SafeInt(self.0 * other.into())
    }
}

impl<T: Into<Integer>> MulAssign<T> for SafeInt {
    #[inline(always)]
    fn mul_assign(&mut self, other: T) {
        self.0 *= other.into();
    }
}

impl<T: Into<Integer>> Rem<T> for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn rem(self, other: T) -> SafeInt {
        SafeInt(self.0 % other.into())
    }
}

impl<T: Into<Integer>> RemAssign<T> for SafeInt {
    #[inline(always)]
    fn rem_assign(&mut self, other: T) {
        self.0 %= other.into();
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

#[test]
fn test_int() {
    let a = SafeInt::from(10);
    let b = SafeInt::from(20);
    let c = a.clone() + b;
    let d = a.clone() + c.clone();
    assert_eq!(c, 30);
    assert!(d > a);
    assert!(a < d);
    assert!(a < b);
}
