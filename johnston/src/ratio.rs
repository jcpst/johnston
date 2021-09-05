use crate::math::gcd;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Ratio {
    pub numerator: i32,
    pub denominator: i32,
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lcd = (self.denominator * other.denominator) / gcd(self.denominator, other.denominator);
        let a = self.numerator * (lcd / self.denominator);
        let b = other.numerator * (lcd / other.denominator);
        a.cmp(&b)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
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
