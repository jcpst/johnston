use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::math::gcd;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ratio {
    pub numerator: i32,
    pub denominator: i32,
}

pub trait Rational {
    fn to_ratio(&self) -> Ratio;
}

impl Rational for (i32, i32) {
    fn to_ratio(&self) -> Ratio {
        let (numerator, denominator) = *self;

        if denominator == 0 {
            panic!("Zero is an invalid denominator!");
        }

        let gcd = gcd(numerator, denominator);
        Ratio {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

impl Ratio {
    pub fn new<T: Rational>(rational: T) -> Self {
        rational.to_ratio()
    }
}

impl Mul for Ratio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new((numerator, denominator))
    }
}

impl Div for Ratio {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.numerator == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;
        Self::new((numerator, denominator))
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Ratio) {
        *self = *self * rhs;
    }
}

impl DivAssign for Ratio {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}
