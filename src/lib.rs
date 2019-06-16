//! ## Utilities for generating JI pitch lattices.
//!
mod intext;
use rug::{Rational};
use std::f32;
use intext::intext::IntExt;

#[derive(Debug, PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

pub trait RationalExt {
    fn cents(&self) -> f32;
    fn to_list(&self) -> Vec<i32>;
    fn ordinal(&self) -> Ordinal;
    fn limit(&self) -> i32;
    fn walk(&self, times: usize) -> Vec<Rational>;
    fn flatten(self) -> Rational;
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

pub fn lattice_relation(prime: usize, ordinal: Ordinal) -> Rational {
    let two = Rational::from((2, 1));
    let p = Rational::from((prime, 1));
    let ratio = match ordinal {
        Ordinal::Otonal => p / two,
        Ordinal::Utonal => two / p,
    };

    ratio.flatten()
}

pub fn otonal_step(ratio: &Rational, step: &Rational) -> Rational {
    (Rational::from(ratio) * step).flatten()
}

pub fn utonal_step(ratio: &Rational, step: &Rational) -> Rational {
    (Rational::from(ratio) * Rational::from(step.recip_ref())).flatten()
}
