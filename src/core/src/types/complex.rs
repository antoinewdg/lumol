// Lumol, an extensible molecular simulation engine
// Copyright (C) 2015-2016 Lumol's contributors — BSD license

//! Complex type
use std::ops::{Add, Sub, Neg, Mul, Div};
use std::f64;

use types::{Zero, One};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Complex number, with double precision real and imaginary parts.
///
/// `Complex` implements all the usual arithmetic operations:
///
/// ```
/// # use lumol::types::Complex;
///
/// let w = Complex::cartesian(-1.0, 0.5);
/// let z = Complex::cartesian(4.0, 2.0);
///
/// // Addition
/// let c = w + z;
/// assert_eq!(c, Complex::cartesian(3.0, 2.5));
///
/// // Subtraction
/// let c = w - z;
/// assert_eq!(c, Complex::cartesian(-5.0, -1.5));
///
/// // Multiplication
/// let c = w * z;
/// assert_eq!(c, Complex::cartesian(-5.0, 0.0));
///
/// let c = 42.0 * w;
/// assert_eq!(c, Complex::cartesian(-42.0, 21.0));
///
/// // Division
/// let c = z / 2.0;
/// assert_eq!(c, Complex::cartesian(2.0, 1.0));
/// ```
pub struct Complex {
    /// Real part of the complex
    real: f64,
    /// Imaginary part of the complex
    imag: f64,
}

impl Complex {
    /// Create a new `Complex` from a norm `r` and a phase `phi` in radians.
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// # use std::f64;
    /// let z = Complex::polar(3.0, f64::consts::PI);
    /// assert_eq!(z.norm(), 3.0);
    /// ```
    pub fn polar(r: f64, phi: f64) -> Complex {
        Complex{
            real: r * f64::cos(phi),
            imag: r * f64::sin(phi)
        }
    }

    /// Create a complex from Cartesian coordinates
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let z = Complex::cartesian(3.0, -2.0);
    /// assert_eq!(z.real(), 3.0);
    /// assert_eq!(z.imag(), -2.0);
    /// ```
    pub fn cartesian(x: f64, y: f64) -> Complex {
        Complex{
            real: x,
            imag: y,
        }
    }

    /// Get the real part of the complex
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let z = Complex::cartesian(3.0, -2.0);
    /// assert_eq!(z.real(), 3.0);
    /// ```
    #[inline]
    pub fn real(&self) -> f64 {
        self.real
    }

    /// Get the imaginary part of the complex
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let z = Complex::cartesian(3.0, -2.0);
    /// assert_eq!(z.imag(), -2.0);
    /// ```
    #[inline]
    pub fn imag(&self) -> f64 {
        self.imag
    }

    /// Get the phase of the complex in the [-π, π) interval
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let z = Complex::polar(2.0, 0.3);
    /// assert_eq!(z.phase(), 0.3);
    /// ```
    #[inline]
    pub fn phase(&self) -> f64 {
        f64::atan2(self.imag, self.real)
    }

    /// Get the norm of the complex
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// # use std::f64;
    /// let z = Complex::polar(2.0, 0.3);
    /// assert_eq!(z.norm(), 2.0);
    ///
    /// let z = Complex::cartesian(2.0, 1.0);
    /// assert_eq!(z.norm(), f64::sqrt(5.0));
    /// ```
    #[inline]
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.norm2())
    }

    /// Get the square of the norm if this complex
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let z = Complex::cartesian(2.0, 1.0);
    /// assert_eq!(z.norm2(), 5.0);
    /// ```
    #[inline]
    pub fn norm2(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    /// Get the conjugate of the complex
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let z = Complex::cartesian(2.0, 1.0);
    /// assert_eq!(z.conj(), Complex::cartesian(2.0, -1.0));
    /// ```
    #[inline]
    pub fn conj(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag
        }
    }

    /// Get only the imaginary part of the multiplication.
    /// # Examples
    /// ```
    /// # use lumol::types::Complex;
    /// let a = Complex::cartesian(3.0, -2.0);
    /// let b = Complex::cartesian(1.5, -3.0);
    ///
    /// assert_eq!(a.imag_mul(b), (a*b).imag());
    /// assert_eq!(b.imag_mul(a), (a*b).imag());
    /// ```
    #[inline]
    pub fn imag_mul(self, other: Complex) -> f64 {
        self.real() * other.imag() + self.imag() * other.real()
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        let x = self.real() + other.real();
        let y = self.imag() + other.imag();
        return Complex::cartesian(x, y);
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        let x = self.real() - other.real();
        let y = self.imag() - other.imag();
        return Complex::cartesian(x, y);
    }
}

impl Neg for Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Complex {
        Complex{
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        let x = self.real() * other.real() - self.imag() * other.imag();
        let y = self.real() * other.imag() + self.imag() * other.real();
        Complex::cartesian(x, y)
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: f64) -> Complex {
        Complex::cartesian(self.real() * other, self.imag() * other)
    }
}

impl Mul<Complex> for f64 {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex::cartesian(self * other.real(), self * other.imag())
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let r = other.norm2();
        let x = self.real() * other.real() + self.imag() * other.imag();
        let y = - self.real() * other.imag() + self.imag() * other.real();

        Complex::cartesian(x / r, y / r)
    }
}

impl Div<f64> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: f64) -> Complex {
        let norm = self.norm() / other;
        let phase = self.phase();
        Complex::polar(norm, phase)
    }
}

impl Zero for Complex {
    fn zero() -> Complex {
        Complex::cartesian(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.norm2() == 0.0
    }
}

impl One for Complex {
    fn one() -> Complex {
        Complex::cartesian(1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts;

    use approx::ApproxEq;
    impl ApproxEq for Complex {
        type Epsilon = <f64 as ApproxEq>::Epsilon;

        fn default_epsilon() -> Self::Epsilon {
            f64::default_epsilon()
        }

        fn default_max_relative() -> Self::Epsilon {
            f64::default_max_relative()
        }

        fn default_max_ulps() -> u32 {
            f64::default_max_ulps()
        }

        fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
            f64::relative_eq(&self.real, &other.real, epsilon, max_relative) &&
            f64::relative_eq(&self.imag, &other.imag, epsilon, max_relative)
        }

        fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
            f64::ulps_eq(&self.real, &other.real, epsilon, max_ulps) &&
            f64::ulps_eq(&self.imag, &other.imag, epsilon, max_ulps)
        }
    }

    #[test]
    fn norm() {
        let c = Complex::polar(3.0, 5.0);
        assert_ulps_eq!(c.norm(), 3.0);

        let c = Complex::polar(-3.0, 0.0);
        assert_ulps_eq!(c.norm(), 3.0);
    }

    #[test]
    fn phase() {
        // Phase is between 0 and 2π
        for &phase in &[-consts::PI, -3.1, -1.5, 0.0, 0.1, 2.0, 3.1] {
            let c = Complex::polar(1.0, phase);
            assert_ulps_eq!(c.phase(), phase);
        }

        let c = Complex::polar(1.0, -8.0);
        assert_ulps_eq!(c.phase(), -8.0 + 2.0 * consts::PI);

        let c = Complex::polar(1.0, 12.0);
        assert_ulps_eq!(c.phase(), 12.0 - 4.0 * consts::PI, max_ulps=10);

        let c = Complex::polar(1.0, consts::PI);
        assert_eq!(c.phase(), consts::PI);
        let c = Complex::polar(1.0, -consts::PI);
        assert_eq!(c.phase(), -consts::PI);

        let c = Complex::polar(3.0, 0.0);
        assert_eq!(c.phase(), 0.0);

        let c = Complex::polar(-3.0, 0.0);
        assert_eq!(c.phase(), -consts::PI);
    }

    #[test]
    fn conj() {
        let c = Complex::cartesian(3.0, 5.0);
        assert_eq!(c.conj(), Complex::cartesian(3.0, -5.0));

        let c = Complex::cartesian(3.0, -5.0);
        assert_eq!(c.conj(), Complex::cartesian(3.0, 5.0));

        let c = Complex::cartesian(3.0, 0.0);
        assert_eq!(c.conj(), Complex::cartesian(3.0, 0.0));
    }

    #[test]
    fn cartesian() {
        let c = Complex::polar(1.0, 0.0);
        assert_ulps_eq!(c.real(), 1.0);
        assert_ulps_eq!(c.imag(), 0.0);

        let c = Complex::polar(1.0, consts::PI);
        assert_ulps_eq!(c.real(), -1.0);
        assert_ulps_eq!(c.imag(), 0.0);

        let c = Complex::polar(1.0, consts::FRAC_PI_2);
        assert_ulps_eq!(c.real(), 0.0);
        assert_ulps_eq!(c.imag(), 1.0);

        let c = Complex::polar(1.0, consts::FRAC_PI_4);
        assert_ulps_eq!(c.real(), consts::FRAC_1_SQRT_2);
        assert_ulps_eq!(c.imag(), consts::FRAC_1_SQRT_2);

        let c = Complex::cartesian(consts::FRAC_1_SQRT_2, consts::FRAC_1_SQRT_2);
        assert_ulps_eq!(c.norm(), 1.0);
        assert_ulps_eq!(c.phase(), consts::FRAC_PI_4);
    }

    #[test]
    fn add() {
        let a = Complex::polar(2.0, 0.2);
        let b = Complex::polar(1.0, 0.5);
        let c = a + b;

        assert_eq!(c.real(), a.real() + b.real());
        assert_eq!(c.imag(), a.imag() + b.imag());
    }

    #[test]
    fn sub() {
        let a = Complex::polar(2.0, 0.2);
        let b = Complex::polar(1.0, 0.5);
        let c = a - b;

        assert_eq!(c.real(), a.real() - b.real());
        assert_eq!(c.imag(), a.imag() - b.imag());
    }

    #[test]
    fn neg() {
        let a = Complex::polar(2.0, 0.2);
        let c = -a;

        assert_ulps_eq!(c.norm(), a.norm());
        assert_ulps_eq!(c.phase(), a.phase() - consts::PI);

        assert_ulps_eq!(c.real(), - a.real());
        assert_ulps_eq!(c.imag(), - a.imag());
    }

    #[test]
    fn mul() {
        let a = Complex::polar(2.0, 1.4);
        let b = Complex::polar(1.0, 0.5);
        let c = a * b;

        assert_eq!(c.norm(), a.norm() * b.norm());
        assert_eq!(c.phase(), a.phase() + b.phase());

        let c = 3.0 * a;
        assert_eq!(c.norm(), 3.0 * a.norm());
        assert_eq!(c.phase(), a.phase());

        let c = a * 3.0;
        assert_eq!(c.norm(), 3.0 * a.norm());
        assert_eq!(c.phase(), a.phase());

        let c = -2.0 * a;
        assert_eq!(c.norm(), 2.0 * a.norm());
        assert_ulps_eq!(c.phase(), a.phase() - consts::PI);
    }

    #[test]
    fn div() {
        let a = Complex::polar(2.0, 0.2);
        let b = Complex::polar(1.0, 0.5);
        let c = a / b;

        assert_eq!(c.norm(), a.norm() / b.norm());
        assert_eq!(c.phase(), a.phase() - b.phase());

        let c = a / 3.0;
        assert_eq!(c.norm(), a.norm()/3.0);
        assert_ulps_eq!(c.phase(), a.phase());

        let c = a / (-2.0);
        assert_eq!(c.norm(), a.norm()/2.0);
        assert_ulps_eq!(c.phase(), a.phase() - consts::PI);
    }
}
