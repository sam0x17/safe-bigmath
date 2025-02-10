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

impl Add<&SafeInt> for &SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn add(self, other: &SafeInt) -> SafeInt {
        SafeInt(self.0.clone() + &other.0)
    }
}

impl Add<&SafeInt> for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn add(self, other: &SafeInt) -> SafeInt {
        SafeInt(self.0 + &other.0)
    }
}

impl Add<SafeInt> for &SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn add(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0.clone() + other.0)
    }
}

impl AddAssign for SafeInt {
    #[inline(always)]
    fn add_assign(&mut self, other: SafeInt) {
        self.0 += other.0;
    }
}

impl AddAssign<&SafeInt> for SafeInt {
    #[inline(always)]
    fn add_assign(&mut self, other: &SafeInt) {
        self.0 += &other.0;
    }
}

impl Sub for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn sub(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0 - other.0)
    }
}

impl Sub<&SafeInt> for &SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn sub(self, other: &SafeInt) -> SafeInt {
        SafeInt(self.0.clone() - &other.0)
    }
}

impl Sub<&SafeInt> for SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn sub(self, other: &SafeInt) -> SafeInt {
        SafeInt(self.0 - &other.0)
    }
}

impl Sub<SafeInt> for &SafeInt {
    type Output = SafeInt;

    #[inline(always)]
    fn sub(self, other: SafeInt) -> SafeInt {
        SafeInt(self.0.clone() + other.0)
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
}
