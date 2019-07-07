mod constants;
mod extensions;
pub use constants::OCTAVE;
pub use extensions::{IntExt, Ratio};
use rug::Rational;

#[derive(Debug, PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pitch {
    pub cents: f32,
    pub ratio: Rational,
}

#[derive(Debug, PartialEq)]
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
    fn new(dimension: usize, steps: usize) -> LatticeDimension {
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

pub fn lattice_relation(prime: usize, ordinal: Ordinal) -> Rational {
    let octave = Rational::from(OCTAVE);
    let p = Rational::from((prime, 1));
    let ratio = match ordinal {
        Ordinal::Otonal => p / octave,
        Ordinal::Utonal => octave / p,
    };

    ratio.flatten()
}

pub fn gen_lattice(dimensions: &[usize], steps: usize) -> Vec<LatticeDimension> {
    let mut result = vec![];

    for dimension in dimensions {
        result.push(LatticeDimension::new(*dimension, steps));
    }

    result
}
