use crate::{
    intervals::{OCTAVE, TONIC},
    lattice::LatticePosition,
    math::{gpf, power_of_two},
    ratio::Ratio,
};
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch {
    pub cents: f32,
    pub ratio: Ratio,
    pub limit: i32,
    pub ordinal: Ordinal,
}

pub trait Pitchable {
    fn to_pitch(&self) -> Pitch;
}

impl Pitchable for (i32, i32) {
    fn to_pitch(&self) -> Pitch {
        let ratio = Ratio::new(*self);
        Pitch {
            cents: cents(ratio),
            ratio: flatten(ratio),
            limit: limit(ratio),
            ordinal: determine_ordinal(ratio),
        }
    }
}

impl Pitchable for i32 {
    fn to_pitch(&self) -> Pitch {
        let ratio = Ratio::new((*self, 1));
        Pitch {
            cents: cents(ratio),
            ratio: flatten(ratio),
            limit: limit(ratio),
            ordinal: determine_ordinal(ratio),
        }
    }
}

impl Pitchable for Ratio {
    fn to_pitch(&self) -> Pitch {
        Pitch {
            cents: cents(*self),
            ratio: flatten(*self),
            limit: limit(*self),
            ordinal: determine_ordinal(*self),
        }
    }
}

impl Pitchable for Pitch {
    fn to_pitch(&self) -> Pitch {
        Pitch {
            cents: self.cents,
            ratio: flatten(self.ratio),
            limit: self.limit,
            ordinal: self.ordinal,
        }
    }
}

impl Add for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Pitch) -> Pitch {
        Pitch::new(self.ratio * rhs.ratio)
    }
}

impl AddAssign for Pitch {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Pitch {
    pub fn new<T: Pitchable>(pitchable: T) -> Pitch {
        pitchable.to_pitch()
    }

    pub fn to_position(&self) -> LatticePosition {
        if self.ratio == Pitch::new(TONIC).ratio {
            return LatticePosition {
                dimension: 2,
                point: 0,
            };
        }

        // TODO: this fails to find multidimensional pitches
        // Potential fix:
        // - get limit
        // - for each point in the dimension:
        //   - if the point is not the position,
        //     subtract the pitch from the pitch in that position.
        //   - use that pitch as the dimension to move on the lattice.
        //   - if the numerator of the guess is greater than the
        //     numerator on the original pitch, move on to the next
        //     point in the original dimension.

        // TODO: get this algorithm to work with utonal ratios.

        let initial_pitch = Pitch::new(self.limit);
        let mut pitch = initial_pitch;
        let mut point = 1;

        println!("dimension: {}", self.limit);
        while pitch.ratio != self.ratio {
            println!("{:?}", pitch.ratio);
            pitch += initial_pitch;
            point += 1;
        }

        LatticePosition {
            dimension: self.limit,
            point,
        }
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

fn cents(ratio: Ratio) -> f32 {
    let ratio_as_float = ratio.numerator as f32 / ratio.denominator as f32;
    (1_200f32 / 2f32.log10()) * ratio_as_float.log10()
}

fn flatten(ratio: Ratio) -> Ratio {
    let tonic = Ratio::new(TONIC);
    let octave = Ratio::new(OCTAVE);
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

fn limit(ratio: Ratio) -> i32 {
    let gpf_from_numerator = gpf(ratio.numerator);
    let gpf_from_denominator = gpf(ratio.denominator);

    gpf_from_numerator.max(gpf_from_denominator)
}

pub fn determine_ordinal(ratio: Ratio) -> Ordinal {
    if power_of_two(ratio.denominator) {
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
        let fifth = Ratio::new((3, 2));
        let value = cents(fifth);
        let abs_diff = (value.abs() - value).abs();

        assert!(abs_diff <= f32::EPSILON);
    }

    #[test]
    fn flatten_works() {
        let a = p(18, 4);

        assert_eq!(a.ratio, Ratio::new((9, 8)));
    }

    #[test]
    fn walk_works() {
        let expected = vec![p(1, 1), p(3, 2), p(9, 8)];
        let actual = Pitch::new((3, 2)).walk(3);

        assert_eq!(expected, actual);
    }

    macro_rules! pitch_to_position_test {
        ($($name:ident: $value:expr, $expect:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let pitch = Pitch::new($value);
                    let (dimension, point) = $expect;
                    let expected = LatticePosition { dimension, point };
                    let result = pitch.to_position();

                    assert_eq!(expected, result);
                }
            )*
        }
    }

    pitch_to_position_test! {
        position_for_9_8: (9, 8), (3, 2),
        position_for_27_16: (27, 16), (3, 3),
        position_for_5_4: (5, 4), (5, 1),
        position_for_tonic: (1, 1), (2, 0),

        // These tests FAIL

        // utonal - Currently everything is assumed to be ontonal.
        position_for_16_9: (16, 9), (3, -2),

        // multidimensional - The LatticePosition type can't even describe this yet.
        // position_for_15_8: (15, 8), (3, 3),
    }
}
