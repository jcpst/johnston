use crate::intervals::{OCTAVE, TONIC};
use num::rational::Ratio;
use num::rational::Rational32;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch {
    pub cents: f32,
    pub ratio: Rational32,
}

pub trait PitchInit {
    fn init(&self) -> Pitch;
}

impl PitchInit for (i32, i32) {
    fn init(&self) -> Pitch {
        let ratio = Ratio::<i32>::new(self.0, self.1);
        Pitch {
            cents: cents(ratio),
            ratio: flatten(ratio),
        }
    }
}

impl PitchInit for Rational32 {
    fn init(&self) -> Pitch {
        Pitch {
            cents: cents(*self),
            ratio: flatten(*self),
        }
    }
}

impl PitchInit for Pitch {
    fn init(&self) -> Pitch {
        Pitch {
            cents: self.cents,
            ratio: flatten(self.ratio),
        }
    }
}

impl ops::Add for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Pitch) -> Pitch {
        let ratio = flatten(self.ratio * rhs.ratio);
        let cents = cents(ratio);

        Pitch { cents, ratio }
    }
}

impl Pitch {
    pub fn new<T: PitchInit>(ratio: T) -> Pitch {
        ratio.init()
    }

    pub fn walk(&self, times: i32) -> Vec<Pitch> {
        let mut pitches = vec![Pitch::new((1, 1))];
        for _ in 1..times {
            let last_pitch = pitches.last().cloned().unwrap();
            let next_pitch = last_pitch + *self;
            pitches.push(next_pitch)
        }
        pitches
    }
}

fn cents(ratio: Rational32) -> f32 {
    let ratio_as_float = *ratio.numer() as f32 / *ratio.denom() as f32;
    (1_200f32 / 2f32.log10()) * ratio_as_float.log10()
}

fn interval_ratio(interval: (i32, i32)) -> Rational32 {
    Ratio::<i32>::new(interval.0, interval.1)
}

fn flatten(ratio: Rational32) -> Rational32 {
    let tonic = interval_ratio(TONIC);
    let octave = interval_ratio(OCTAVE);
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
