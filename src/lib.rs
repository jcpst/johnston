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

// flat-map : built in for iterators
// range-inc : built in
// [x] factors
// [x] primep - is_prime
// [x] max-prime - greatest_prime_factor
// [x] ratio-to-list
// [x] power-of-twop - is_power_of_two
// invert-ratio : Rational::recip()
// [x] get-ordinal
// [x] get-limit
// lattice-relation
// partial
// walk
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

pub fn get_ordinal(ratio: Rational) -> Ordinal {
    let (num, den) = ratio.into_numer_denom();

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

    *vec![greatest_num_prime, greatest_den_prime].iter().max().unwrap()
}

pub fn otonal_step(ratio: &Rational, step: &Rational) -> Rational {
    flatten_ratio(Rational::from(ratio) * step)
}

pub fn utonal_step(ratio: &Rational, step: &Rational) -> Rational {
    flatten_ratio(Rational::from(ratio) * Rational::from(step.recip_ref()))
}
/*
/// Makes a list of ratios in one direction of a lattice.
///
/// # Examples
///
/// ```
/// let fifth = Rational::from((3, 2));
/// let result = walk(&utonal_step, 5, &fifth);
/// assert_eq!(result, vec![(1, 1), (4, 3), (16, 9), (32, 27), (128, 81)]);
/// ```
pub fn walk(action: &Step, times: usize, step: &Rational) -> Vec<Rational> {
    let mut ratios = vec![Rational::from((1, 1))];

    for _ in 1..times {
        let next_ratio = action(&ratios.last().unwrap(), step);
        ratios.push(next_ratio);
    }

    ratios
}
*/
