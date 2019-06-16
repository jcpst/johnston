//! ## Utilities for generating JI pitch lattices.
//!
extern crate rug;
use rug::integer::IsPrime;
use rug::{Integer, Rational};
use std::f32;

#[derive(Debug, PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

pub trait IntExt {
    fn factors(self) -> Vec<i32>;
    fn is_prime(self) -> bool;
    fn is_power_of_two(self) -> bool;
    fn gpf(self) -> i32;
}

pub trait RationalExt {
    fn cents(&self) -> f32;
    fn to_list(&self) -> Vec<i32>;
    fn ordinal(&self) -> Ordinal;
    fn limit(&self) -> i32;
    fn walk(&self, usize) -> Vec<Rational>;
    fn flatten(self) -> Rational;
}

impl IntExt for i32 {
    fn factors(self) -> Vec<i32> {
        let nums: Vec<_> = (1..=self).collect();
        let mut result: Vec<i32> = Vec::new();
        for n in nums {
            if self % n == 0 {
                result.push(n);
            }
        }

        result
    }

    fn is_prime(self) -> bool {
        let val = Integer::from(self).is_probably_prime(15);
        if let IsPrime::No = val {
            false
        } else {
            true
        }
    }

    fn is_power_of_two(self) -> bool {
        self != 0 && (self & (self - 1)) == 0
    }

    fn gpf(self) -> i32 {
        let mut result: i32 = 0;
        for n in self.factors() {
            if n.is_prime() && n > result {
                result = n;
            }
        }

        result
    }
}

impl RationalExt for Rational {
    fn cents(&self) -> f32 {
        (1_200f32 / 2f32.log10()) * self.to_f32().log10()
    }

    fn to_list(&self) -> Vec<i32> {
        let (num, den) = Rational::from(self).into_numer_denom();
        vec![num.to_i32().unwrap(), den.to_i32().unwrap()]
    }

    fn ordinal(&self) -> Ordinal {
        let (num, den) = Rational::from(self).into_numer_denom();
        let gpf_num = num.to_i32().unwrap().gpf();
        let gpf_den = den.to_i32().unwrap().gpf();

        if gpf_num > gpf_den {
            Ordinal::Otonal
        } else {
            Ordinal::Utonal
        }
    }

    fn limit(&self) -> i32 {
        let (num, den) = Rational::from(self).into_numer_denom();

        *vec![num.to_i32().unwrap(), den.to_i32().unwrap()]
            .iter()
            .max()
            .unwrap()
    }

    fn walk(&self, times: usize) -> Vec<Rational> {
        let mut ratios = vec![Rational::from((1, 1))];

        for _ in 1..times {
            let last_ratio = ratios.last().cloned().unwrap();
            let next_ratio = (last_ratio * self).flatten();
            ratios.push(next_ratio);
        }

        ratios
    }

    fn flatten(self) -> Rational {
        let one = Rational::from((1, 1));
        let two = Rational::from((2, 1));

        if self > two {
            (self / two).flatten()
        } else if self < one {
            (self * two).flatten()
        } else {
            self
        }
    }
}

// This is the interval to use on a direction of a lattice walk.
pub fn lattice_relation(prime: usize, ordinal: Ordinal) -> Rational {
    let two = Rational::from((2, 1));
    let p = Rational::from((prime, 1));

    (match ordinal {
        Ordinal::Otonal => p / two,
        Ordinal::Utonal => two / p,
    }).flatten()
}

pub fn otonal_step(ratio: &Rational, step: &Rational) -> Rational {
    (Rational::from(ratio) * step).flatten()
}

pub fn utonal_step(ratio: &Rational, step: &Rational) -> Rational {
    (Rational::from(ratio) * Rational::from(step.recip_ref())).flatten()
}
