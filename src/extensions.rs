use crate::constants::{OCTAVE, TONIC};
use crate::Ordinal;
use crate::Pitch;
use rug::integer::IsPrime;
use rug::{Integer, Rational};

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
        let (num, den) = Rational::from(self).into_numer_denom();
        let gpf_num = num.to_i32().unwrap().gpf();
        let gpf_den = den.to_i32().unwrap().gpf();

        if gpf_num > gpf_den {
            Ordinal::Otonal
        } else {
            Ordinal::Utonal
        }
    }

    fn invert_ordinal(self) -> Rational {
        self.recip().flatten()
    }

    fn limit(&self) -> i32 {
        let (num, den) = Rational::from(self).into_numer_denom();

        *vec![num.to_i32().unwrap(), den.to_i32().unwrap()]
            .iter()
            .max()
            .unwrap()
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
