use crate::intervals::{OCTAVE, TONIC};
use num::rational::Ratio;
use num::rational::Rational32;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch {
    pub cents: f32,
    pub ratio: Rational32,
    pub ordinal: Ordinal,
}

pub trait Rationalize {
    fn to_ratio(&self) -> Rational32;
}

impl Rationalize for (i32, i32) {
    fn to_ratio(&self) -> Rational32 {
        flatten(Ratio::<i32>::new(self.0, self.1))
    }
}

impl Rationalize for Rational32 {
    fn to_ratio(&self) -> Rational32 {
        flatten(*self)
    }
}

impl Rationalize for Pitch {
    fn to_ratio(&self) -> Rational32 {
        flatten(self.ratio)
    }
}

impl ops::Add for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Pitch) -> Pitch {
        Pitch::new(self.ratio * rhs.ratio)
    }
}

impl Pitch {
    pub fn new<T: Rationalize>(interval: T) -> Pitch {
        let ratio = interval.to_ratio();
        Pitch {
            cents: cents(ratio),
            ordinal: ordinal(ratio),
            ratio,
        }
    }

    pub fn walk(&self, times: i32) -> Vec<Pitch> {
        let current_pitch = *self;
        let mut pitches = vec![Pitch::new(TONIC)];

        for _ in 1..times {
            let last_pitch = pitches.last().cloned().unwrap();
            let next_pitch = last_pitch + current_pitch;
            pitches.push(next_pitch)
        }
        pitches
    }
}

fn factors(num: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for n in 1..=num {
        if num % n == 0 {
            result.push(n);
        }
    }
    result
}

fn prime(num: i32) -> bool {
    for n in 2..num {
        if num % n == 0 {
            return false;
        }
    }
    true
}

fn gpf(num: i32) -> i32 {
    let mut result = 0;
    for n in factors(num) {
        if prime(n) && n > result {
            result = n;
        }
    }
    result
}

fn cents(ratio: Rational32) -> f32 {
    let ratio_as_float = *ratio.numer() as f32 / *ratio.denom() as f32;
    (1_200f32 / 2f32.log10()) * ratio_as_float.log10()
}

fn flatten(ratio: Rational32) -> Rational32 {
    let to_ratio = |(n, d)| Ratio::<i32>::new(n, d);
    let tonic = to_ratio(TONIC);
    let octave = to_ratio(OCTAVE);
    let mut r = ratio;
    loop {
        if r > octave {
            r /= octave;
        } else if r < tonic {
            r *= octave;
        } else {
            return r;
        }
    }
}

fn ordinal(ratio: Rational32) -> Ordinal {
    let num = gpf(*ratio.numer());
    let den = gpf(*ratio.denom());
    if num > den {
        Ordinal::Otonal
    } else {
        Ordinal::Utonal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

    fn p(n: i32, d: i32) -> Pitch {
        Pitch::new((n, d))
    }

    #[test]
    fn cents_works() {
        let fifth = Ratio::<i32>::new(3, 2);
        let value = cents(fifth);
        let abs_diff = (value.abs() - value).abs();

        assert!(abs_diff <= f32::EPSILON);
    }

    #[test]
    fn flatten_works() {
        let a = Pitch::new((18, 4));

        assert_eq!(a.ratio, Ratio::<i32>::new(9, 8))
    }

    #[test]
    fn walk_works() {
        let expected = vec![p(1, 1), p(3, 2), p(9, 8)];
        let actual = Pitch::new((3, 2)).walk(3);

        assert_eq!(expected, actual);
    }
}
