use num::rational::Ratio;
use num::rational::Rational32;

#[derive(PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pitch {
    pub cents: f32,
    pub ratio: Rational32,
}

#[derive(Debug, PartialEq)]
pub struct LatticeDimension {
    pub limit: i32,
    pub otonal: Vec<Pitch>,
    pub utonal: Vec<Pitch>,
}

#[derive(Debug, PartialEq)]
pub struct Lattice {
    pub dimensions: Vec<LatticeDimension>,
}

impl Pitch {
    pub fn new(interval: (i32, i32)) -> Pitch {
        Pitch::from_ratio(Ratio::<i32>::new(interval.0, interval.1))
    }

    pub fn from_ratio(ratio: Rational32) -> Pitch {
        Pitch {
            cents: cents(ratio),
            ratio: ratio,
        }
    }

    fn flatten(self) -> Pitch {
        let tonic = Ratio::<i32>::new(1, 1);
        let octave = Ratio::<i32>::new(2, 1);

        if self.ratio > octave {
            Pitch::from_ratio(self.ratio / octave).flatten()
        } else if self.ratio < tonic {
            Pitch::from_ratio(self.ratio * octave).flatten()
        } else {
            self
        }
    }

    fn walk(&self, times: i32) -> Vec<Pitch> {
        let mut pitches = vec![Pitch::new((1, 1))];

        for _ in 1..times {
            let last_pitch = pitches.last().cloned().unwrap();
            let next_pitch = Pitch::from_ratio(last_pitch.ratio * self.ratio).flatten();

            pitches.push(next_pitch)
        }

        pitches
    }
}

impl LatticeDimension {
    pub fn new(dimension: i32, steps: i32) -> LatticeDimension {
        LatticeDimension {
            limit: dimension,
            otonal: Pitch::new((dimension, 1)).walk(steps),
            utonal: Pitch::from_ratio(Ratio::<i32>::new(dimension, 1).recip())
                .flatten()
                .walk(steps),
        }
    }
}

impl Lattice {
    pub fn new(dimensions: &[i32], steps: i32) -> Lattice {
        Lattice {
            dimensions: dimensions
                .iter()
                .map(|dimension| LatticeDimension::new(*dimension, steps))
                .collect(),
        }
    }

    pub fn scale(self) -> Vec<Pitch> {
        let mut pitches = Vec::<Pitch>::new();

        for dimension in self.dimensions {
            for pitch in dimension.otonal {
                pitches.push(pitch);
            }
            for pitch in dimension.utonal {
                pitches.push(pitch);
            }
        }

        pitches.sort_unstable_by(|a, b| a.cents.partial_cmp(&b.cents).unwrap());
        pitches.dedup_by(|a, b| a.cents == b.cents);

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

fn power_of_two(num: i32) -> bool {
    num != 0 && (num & (num - 1)) == 0
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

    // Helpers
    // =========================================================================
    fn p(n: i32, d: i32) -> Pitch {
        Pitch::new((n, d))
    }

    #[test]
    fn factors_works() {
        let expected = vec![1, 2, 17, 34];
        let actual = factors(34);
        assert_eq!(expected, actual);
    }

    #[test]
    fn prime_works() {
        assert!(prime(31));
    }

    #[test]
    fn power_of_two_works() {
        assert!(power_of_two(1024));
    }

    #[test]
    fn gpf_works() {
        assert_eq!(gpf(2), 2);
        assert_eq!(gpf(15), 5);
        assert_eq!(gpf(34), 17);
        assert_eq!(gpf(49), 7);
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

        assert_eq!(a.flatten().ratio, Ratio::<i32>::new(9, 8))
    }

    #[test]
    fn walk_works() {
        let expected = vec![p(1, 1), p(3, 2), p(9, 8)];
        let actual = Pitch::new((3, 2)).walk(3);

        assert_eq!(expected, actual);
    }
}
