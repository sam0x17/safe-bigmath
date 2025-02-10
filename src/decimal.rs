use core::{cmp::Ordering, ops::*};
use rug::{
    ops::{NegAssign, Pow},
    Float,
};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct SafeDec<const DECIMALS: usize>(Float);

impl<const D: usize> SafeDec<D> {
    #[inline(always)]
    pub const fn from_raw(value: Float) -> Self {
        SafeDec(value)
    }

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        SafeDec(self.0.abs())
    }

    #[inline(always)]
    pub fn pow(self, exp: u32) -> Self {
        SafeDec(self.0.pow(exp))
    }

    #[inline(always)]
    pub fn floor(self) -> Self {
        SafeDec(self.0.floor())
    }

    #[inline(always)]
    pub fn ceil(self) -> Self {
        SafeDec(self.0.ceil())
    }

    #[inline(always)]
    pub fn round(self) -> Self {
        SafeDec(self.0.round())
    }

    #[inline(always)]
    pub fn round_even(self) -> Self {
        SafeDec(self.0.round_even())
    }
}

impl<const D: usize> Neg for SafeDec<D> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        SafeDec(-self.0)
    }
}

impl<const D: usize> NegAssign for SafeDec<D> {
    #[inline(always)]
    fn neg_assign(&mut self) {
        self.0.neg_assign();
    }
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident) => {
        impl<const D: usize> $trait for SafeDec<D> {
            type Output = Self;

            #[inline(always)]
            fn $method(self, other: Self) -> Self {
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
            type Output = Self;

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
    };
}

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident) => {
        impl<const D: usize> $trait for SafeDec<D> {
            #[inline(always)]
            fn $method(&mut self, other: SafeDec<D>) {
                self.0.$method(other.0);
            }
        }

        impl<const D: usize> $trait<&SafeDec<D>> for SafeDec<D> {
            #[inline(always)]
            fn $method(&mut self, other: &SafeDec<D>) {
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

impl<const D: usize> Div for SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeDec(self.0.div(other.0)))
        }
    }
}

impl<const D: usize> Div<&SafeDec<D>> for SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: &SafeDec<D>) -> Option<SafeDec<D>> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeDec(self.0.div(&other.0)))
        }
    }
}

impl<const D: usize> Div<SafeDec<D>> for &SafeDec<D> {
    type Output = Option<SafeDec<D>>;

    #[inline(always)]
    fn div(self, other: SafeDec<D>) -> Option<SafeDec<D>> {
        if other.0.is_zero() {
            None
        } else {
            Some(SafeDec(self.0.clone().div(other.0)))
        }
    }
}

impl<const D: usize, T: PartialEq<Float>> PartialEq<T> for SafeDec<D> {
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        *other == self.0
    }
}

impl<const D: usize, T: PartialOrd<Float>> PartialOrd<T> for SafeDec<D> {
    #[inline(always)]
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        match other.partial_cmp(&self.0) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            equal => equal,
        }
    }
}

impl<const D: usize, T: Into<Float>> From<T> for SafeDec<D> {
    #[inline(always)]
    fn from(value: T) -> SafeDec<D> {
        SafeDec(value.into())
    }
}

#[test]
fn general() {
    let a = SafeDec::<10>::from(10.0);
    let b = SafeDec::from(20);
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
    assert_eq!((SafeDec::from(10) / SafeDec::from(3)).unwrap(), 3);
    assert_eq!(SafeDec::from(10) / SafeDec::from(0), None);
    assert!(SafeDec::from(10) != 20);
    assert!(SafeDec::from(37984739847983497938479797988798789783u128).is_odd());
    assert!(
        SafeDec::from_raw(
            Float::from_str_radix(
                "3798473984798349793847979798879878978334738744739847983749837",
                10
            )
            .unwrap()
        ) > 10
    );
}
