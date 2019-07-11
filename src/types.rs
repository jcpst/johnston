use crate::constants::{OCTAVE, TONIC};
use rug::integer::IsPrime;
use rug::{Integer, Rational};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pitch {
    pub cents: f32,
    pub ratio: Rational,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LatticeDimension {
    pub limit: usize,
    pub otonal: Vec<Pitch>,
    pub utonal: Vec<Pitch>,
}

impl Pitch {
    pub fn new(ratio: Rational) -> Pitch {
        Pitch {
            cents: ratio.cents(),
            ratio,
        }
    }
}

impl LatticeDimension {
    pub fn new(dimension: usize, steps: usize) -> LatticeDimension {
        LatticeDimension {
            limit: dimension,
            otonal: Rational::from((dimension as i32, 1)).walk(steps),
            utonal: Rational::from((dimension as i32, 1))
                .recip()
                .flatten()
                .walk(steps),
        }
    }
}

pub trait IntExt {
    fn factors(self) -> Vec<i32>;
    fn is_prime(self) -> bool;
    fn is_power_of_two(self) -> bool;
    fn gpf(self) -> i32;
}

pub trait Ratio {
    fn cents(&self) -> f32;
    fn to_list(&self) -> Vec<i32>;
    fn ordinal(&self) -> Ordinal;
    fn invert_ordinal(self) -> Rational;
    fn limit(&self) -> i32;
    fn walk(&self, times: usize) -> Vec<Pitch>;
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

impl Ratio for Rational {
    fn cents(&self) -> f32 {
        (1_200f32 / 2f32.log10()) * self.to_f32().log10()
    }

    fn to_list(&self) -> Vec<i32> {
        let (num, den) = Rational::from(self).into_numer_denom();
        vec![num.to_i32().unwrap(), den.to_i32().unwrap()]
    }

    fn ordinal(&self) -> Ordinal {
        let items = self.to_list();
        let num = items[0].gpf();
        let den = items[1].gpf();

        if num > den {
            Ordinal::Otonal
        } else {
            Ordinal::Utonal
        }
    }

    fn invert_ordinal(self) -> Rational {
        self.recip().flatten()
    }

    fn limit(&self) -> i32 {
        *self.to_list().iter().max().unwrap()
    }

    fn walk(&self, times: usize) -> Vec<Pitch> {
        let tonic = Rational::from(TONIC);
        let mut pitches = vec![Pitch::new(tonic)];

        for _ in 1..times {
            let last_pitch = pitches.last().cloned().unwrap();
            let next_pitch = (last_pitch.ratio * self).flatten();

            pitches.push(Pitch::new(next_pitch));
        }

        pitches
    }

    fn flatten(self) -> Rational {
        let tonic = TONIC;
        let octave = Rational::from(OCTAVE);

        if self > octave {
            (self / octave).flatten()
        } else if self < tonic {
            (self * octave).flatten()
        } else {
            self
        }
    }
}
