//! [Complex numbers][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Complex_number

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A number.
pub trait Number: Add<Output=Self> +
                  Div<Output=Self> +
                  Mul<Output=Self> +
                  Neg<Output=Self> +
                  Sub<Output=Self> +
                  Copy + Debug + PartialEq {
}

/// A real number.
pub trait Real: Number {
}

/// A complex number.
pub trait Complex: Number {
    /// A real number.
    type Real: Real;

    /// Create a complex number from a real and an imaginary part.
    fn new(Self::Real, Self::Real) -> Self;

    /// Return the real part.
    fn re(&self) -> Self::Real;

    /// Return the real part as a mutable reference.
    fn re_mut(&mut self) -> &mut Self::Real;

    /// Return the imaginary part.
    fn im(&self) -> Self::Real;

    /// Return the imaginary part as a mutable reference.
    fn im_mut(&mut self) -> &mut Self::Real;

    /// Compute the complex conjugate.
    #[inline(always)]
    fn conj(&self) -> Self {
        Complex::new(self.re(), -self.im())
    }
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
        impl Number for $complex {
        }

        impl Number for $real {
        }

        impl Real for $real {
        }

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
            fn re_mut(&mut self) -> &mut Self::Real {
                &mut self.0
            }

            #[inline(always)]
            fn im(&self) -> Self::Real {
                self.1
            }

            #[inline(always)]
            fn im_mut(&mut self) -> &mut Self::Real {
                &mut self.1
            }
        }

        impl Add for $complex {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                Complex::new(self.re() + rhs.re(), self.im() + rhs.im())
            }
        }

        impl Add<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: $real) -> Self::Output {
                Complex::new(self.re() + rhs, self.im())
            }
        }

        impl Add<$complex> for $real {
            type Output = $complex;

            #[inline(always)]
            fn add(self, rhs: $complex) -> Self::Output {
                Complex::new(self + rhs.re(), rhs.im())
            }
        }

        impl Div for $complex {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                let denominator = rhs.re() * rhs.re() + rhs.im() * rhs.im();
                Complex::new((self.re() * rhs.re() + self.im() * rhs.im()) / denominator,
                             (self.im() * rhs.re() - self.re() * rhs.im()) / denominator)
            }
        }

        impl Div<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: $real) -> Self::Output {
                Complex::new(self.re() / rhs, self.im() / rhs)
            }
        }

        impl Div<$complex> for $real {
            type Output = $complex;

            #[inline(always)]
            fn div(self, rhs: $complex) -> Self::Output {
                let denominator = rhs.re() * rhs.re() + rhs.im() * rhs.im();
                Complex::new((self * rhs.re()) / denominator, (-self * rhs.im()) / denominator)
            }
        }

        impl Mul for $complex {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self::Output {
                Complex::new(self.re() * rhs.re() - self.im() * rhs.im(),
                             self.im() * rhs.re() + self.re() * rhs.im())
            }
        }

        impl Mul<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: $real) -> Self::Output {
                Complex::new(self.re() * rhs, self.im() * rhs)
            }
        }

        impl Mul<$complex> for $real {
            type Output = $complex;

            #[inline(always)]
            fn mul(self, rhs: $complex) -> Self::Output {
                Complex::new(self * rhs.re(), self * rhs.im())
            }
        }

        impl Neg for $complex {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self::Output {
                Complex::new(-self.re(), -self.im())
            }
        }

        impl Sub for $complex {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                Complex::new(self.re() - rhs.re(), self.im() - rhs.im())
            }
        }

        impl Sub<$real> for $complex {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: $real) -> Self::Output {
                Complex::new(self.re() - rhs, self.im())
            }
        }

        impl Sub<$complex> for $real {
            type Output = $complex;

            #[inline(always)]
            fn sub(self, rhs: $complex) -> Self::Output {
                Complex::new(self - rhs.re(), -rhs.im())
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
    fn re_mut() {
        let mut number = c64(69.0, 0.0);
        *number.re_mut() = 42.0;
        assert_eq!(number, c64(42.0, 0.0));
    }

    #[test]
    fn im_mut() {
        let mut number = c64(0.0, 69.0);
        *number.im_mut() = 42.0;
        assert_eq!(number, c64(0.0, 42.0));
    }

    #[test]
    fn conj() {
        assert_eq!(c64(42.0, 69.0).conj(), c64(42.0, -69.0));
    }

    #[test]
    fn add() {
        assert_eq!(c64(-4.0, 7.0) + c64(5.0, -10.0), c64(1.0, -3.0));
        assert_eq!(c64(-4.0, 7.0) + 5.0, c64(1.0, 7.0));
        assert_eq!(5.0 + c64(-4.0, 7.0), c64(1.0, 7.0));
    }

    #[test]
    fn div() {
        assert_eq!(c64(3.0, -1.0) / c64(2.0, 7.0), c64(-1.0 / 53.0, -23.0 / 53.0));
        assert_eq!(c64(3.0, -1.0) / 2.0, c64(1.5, -0.5));
        assert_eq!(2.0 / c64(3.0, -1.0), c64(0.6, 0.2));
    }

    #[test]
    fn mul() {
        assert_eq!(c64(4.0, 1.0) * c64(2.0, 3.0), c64(5.0, 14.0));
        assert_eq!(c64(4.0, 1.0) * 2.0, c64(8.0, 2.0));
        assert_eq!(2.0 * c64(4.0, 1.0), c64(8.0, 2.0));
    }

    #[test]
    fn neg() {
        assert_eq!(-c64(42.0, 69.0), c64(42.0, 69.0) * (-1.0));
    }

    #[test]
    fn sub() {
        assert_eq!(c64(4.0, 12.0) - c64(3.0, -15.0), c64(1.0, 27.0));
        assert_eq!(c64(4.0, 12.0) - 3.0, c64(1.0, 12.0));
        assert_eq!(3.0 - c64(4.0, 12.0), c64(-1.0, -12.0));
    }

    #[test]
    fn size_of() {
        use std::mem::size_of;
        assert_eq!(size_of::<c64>(), 2 * size_of::<f64>());
        assert_eq!(size_of::<[c64; 42]>(), size_of::<[f64; 2 * 42]>());
    }
}
