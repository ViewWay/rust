use crate::cmp::PartialEq;
use crate::fmt::Debug;
use crate::ops::{Add, Sub};

/// The complex number type.
/// This promises to be equivalent to the C type on each platform.
#[derive(Debug, PartialEq)]
#[unstable(feature = "complex_numbers", issue = "154023")]
pub struct Complex<T> {
    re: T,
    im: T,
}

#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T> Complex<T> {
    /// Constructs a new instance of type `Complex`.
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Complex<T::Output>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output { re: self.re + other.re, im: self.im + other.im }
    }
}

#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T: Add<Output = T>> Add<T> for Complex<T> {
    type Output = Complex<T::Output>;

    fn add(self, other: T) -> Self::Output {
        Self::Output { re: self.re + other, im: self.im }
    }
}

#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Complex<T::Output>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output { re: self.re - other.re, im: self.im - other.im }
    }
}

#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T: Sub<Output = T>> Sub<T> for Complex<T> {
    type Output = Complex<T::Output>;

    fn sub(self, other: T) -> Self::Output {
        Self::Output { re: self.re - other, im: self.im }
    }
}
