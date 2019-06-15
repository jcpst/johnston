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

pub fn factors(num: i32) -> Vec<i32> {
    let nums: Vec<_> = (1..=num).collect();
    let mut result: Vec<i32> = Vec::new();
    for n in nums {
        if num % n == 0 {
            result.push(n);
        }
    }

    result
}

pub fn is_prime(num: i32) -> bool {
    let val = Integer::from(num).is_probably_prime(15);
    if let IsPrime::No = val {
        false
    } else {
        true
    }
}

pub fn greatest_prime_factor(num: i32) -> i32 {
    let mut result: i32 = 0;
    for n in factors(num) {
        if is_prime(n) && n > result {
            result = n;
        }
    }

    result
}

pub fn ratio_to_list(ratio: Rational) -> Vec<i32> {
    let (num, den) = ratio.into_numer_denom();
    vec![num.to_i32().unwrap(), den.to_i32().unwrap()]
}

pub fn is_power_of_two(num: i32) -> bool {
    num != 0 && (num & (num - 1)) == 0
}

pub fn calc_cents(ratio: &Rational) -> f32 {
    (1_200f32 / 2f32.log10()) * ratio.to_f32().log10()
}

// TODO: I think this could be part of an `impl` on Rational...
pub fn flatten_ratio(ratio: Rational) -> Rational {
    let one = Rational::from((1, 1));
    let two = Rational::from((2, 1));

    if ratio > two {
        flatten_ratio(ratio / two)
    } else if ratio < one {
        flatten_ratio(ratio * two)
    } else {
        ratio
    }
}

pub fn get_ordinal(ratio: &Rational) -> Ordinal {
    let (num, den) = Rational::from(ratio).into_numer_denom();

    if greatest_prime_factor(num.to_i32().unwrap()) > greatest_prime_factor(den.to_i32().unwrap()) {
        Ordinal::Otonal
    } else {
        Ordinal::Utonal
    }
}

pub fn get_limit(ratio: Rational) -> i32 {
    let (num, den) = ratio.into_numer_denom();
    let greatest_num_prime = num.to_i32().unwrap();
    let greatest_den_prime = den.to_i32().unwrap();

    *vec![greatest_num_prime, greatest_den_prime]
        .iter()
        .max()
        .unwrap()
}

// This is the interval to use on a direction of a lattice walk.
pub fn lattice_relation(prime: usize, ordinal: Ordinal) -> Rational {
    let two = Rational::from((2, 1));
    let p = Rational::from((prime, 1));

    flatten_ratio(match ordinal {
        Ordinal::Otonal => p / two,
        Ordinal::Utonal => two / p,
    })
}

pub fn otonal_step(ratio: &Rational, step: &Rational) -> Rational {
    flatten_ratio(Rational::from(ratio) * step)
}

pub fn utonal_step(ratio: &Rational, step: &Rational) -> Rational {
    flatten_ratio(Rational::from(ratio) * Rational::from(step.recip_ref()))
}

pub fn walk(step: &Rational, times: usize) -> Vec<Rational> {
    let mut ratios = vec![Rational::from((1, 1))];

    for _ in 1..times {
        let last_ratio = ratios.last().cloned().unwrap();
        let next_ratio = flatten_ratio(last_ratio * step);
        ratios.push(next_ratio);
    }

    ratios
}

