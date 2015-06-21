//! [Complex numbers][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Complex_number

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// A complex number.
pub trait Complex: Add<Output=Self> +
                   Div<Output=Self> +
                   Mul<Output=Self> +
                   Sub<Output=Self> +
                   Copy + Debug + PartialEq {

    type Real: Add<Output=Self::Real> +
               Div<Output=Self::Real> +
               Mul<Output=Self::Real> +
               Sub<Output=Self::Real> +
               Copy + Debug + PartialEq;

    fn new(Self::Real, Self::Real) -> Self;
    fn re(&self) -> Self::Real;
    fn im(&self) -> Self::Real;
}

/// A complex number with 32-bit parts.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct c32(pub f32, pub f32);

/// A complex number with 64-bit parts.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct c64(pub f64, pub f64);

macro_rules! implement(
    ($complex:ident, $real:ty) => (
        impl Complex for $complex {
            type Real = $real;

            #[inline(always)]
            fn new(re: Self::Real, im: Self::Real) -> $complex {
                $complex(re, im)
            }

            #[inline(always)]
            fn re(&self) -> Self::Real {
                self.0
            }

            #[inline(always)]
            fn im(&self) -> Self::Real {
                self.1
            }
        }

        impl Add for $complex {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Complex::new(self.re() + rhs.re(), self.im() + rhs.im())
            }
        }

        impl Add<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: $real) -> Self {
                Complex::new(self.re() + rhs, self.im())
            }
        }

        impl Div for $complex {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                let denominator = rhs.re() * rhs.re() + rhs.im() * rhs.im();
                Complex::new((self.re() * rhs.re() + self.im() * rhs.im()) / denominator,
                             (self.im() * rhs.re() - self.re() * rhs.im()) / denominator)
            }
        }

        impl Div<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: $real) -> Self {
                Complex::new(self.re() / rhs, self.im() / rhs)
            }
        }

        impl Mul for $complex {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                Complex::new(self.re() * rhs.re() - self.im() * rhs.im(),
                             self.im() * rhs.re() + self.re() * rhs.im())
            }
        }

        impl Mul<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: $real) -> Self {
                Complex::new(self.re() * rhs, self.im() * rhs)
            }
        }

        impl Sub for $complex {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Complex::new(self.re() - rhs.re(), self.im() - rhs.im())
            }
        }

        impl Sub<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: $real) -> Self {
                Complex::new(self.re() - rhs, self.im())
            }
        }
    );
);

implement!(c32, f32);
implement!(c64, f64);

#[cfg(test)]
mod tests {
    use {Complex, c64};

    #[test]
    fn add() {
        assert_eq!(c64(-4.0, 7.0) + c64(5.0, -10.0), c64(1.0, -3.0));
    }

    #[test]
    fn div() {
        assert_eq!(c64(3.0, -1.0) / c64(2.0, 7.0), c64(-1.0 / 53.0, -23.0 / 53.0));
        assert_eq!(c64(3.0, 0.0) / c64(9.0, -1.0), c64(27.0 / 82.0, 3.0 / 82.0));
        assert_eq!(c64(0.0, 8.0) / c64(1.0, 2.0), c64(16.0 / 5.0, 8.0 / 5.0));
        assert_eq!(c64(6.0, -9.0) / c64(0.0, 2.0), c64(-9.0 / 2.0, -3.0));
    }

    #[test]
    fn mul() {
        assert_eq!(c64(0.0, 7.0) * c64(-5.0, 2.0), c64(-14.0, -35.0));
        assert_eq!(c64(1.0, -5.0) * c64(-9.0, 2.0), c64(1.0, 47.0));
        assert_eq!(c64(4.0, 1.0) * c64(2.0, 3.0), c64(5.0, 14.0));
        assert_eq!(c64(1.0, -8.0) * c64(1.0, 8.0), c64(65.0, 0.0));
    }

    #[test]
    fn sub() {
        assert_eq!(c64(4.0, 12.0) - c64(3.0, -15.0), c64(1.0, 27.0));
        assert_eq!(c64(0.0, 5.0) - c64(-9.0, 1.0), c64(9.0, 4.0));
    }

    #[test]
    fn size_of() {
        use std::mem::size_of;
        assert_eq!(size_of::<c64>(), 2 * size_of::<f64>());
        assert_eq!(size_of::<[c64; 42]>(), size_of::<[f64; 2 * 42]>());
    }

    #[test]
    fn scalar_arguments() {
        assert_eq!(go(c64(0.0, 0.0), c64(0.0, 0.0), c64(1.0, 1.0)), c64(0.0, 0.0));

        fn go<C>(a: C, b: C, c: C) -> C where C: Complex {
            (a + b) * (a - b) / c
        }
    }

    #[test]
    fn vector_arguments() {
        assert_eq!(go(&[c64(0.0, 0.0)], &[c64(0.0, 0.0)], &[c64(1.0, 1.0)]), &[c64(0.0, 0.0)]);

        fn go<C>(a: &[C], b: &[C], c: &[C]) -> Vec<C> where C: Complex {
            a.iter().zip(b).zip(c).map(|((&a, &b), &c)| (a + b) * (a - b) / c).collect()
        }
    }
}
