mod intervals;
mod types;
pub use intervals::*;
use rug::Rational;
pub use types::{IntExt, LatticeDimension, Ordinal, Pitch, Ratio};

///  
pub fn lattice_relation(prime: usize, ordinal: Ordinal) -> Rational {
    let octave = Rational::from(OCTAVE);
    let p = Rational::from((prime, 1));
    let ratio = match ordinal {
        Ordinal::Otonal => p / octave,
        Ordinal::Utonal => octave / p,
    };

    ratio.flatten()
}

/// Generates a multi-dimensional pitch lattice.
/// ### Example
///
/// ```
/// # use johnston::gen_lattice;
/// let lattice = gen_lattice(&[3, 5], 2);
/// ```
pub fn gen_lattice(dimensions: &[usize], steps: usize) -> Vec<LatticeDimension> {
    let mut result = vec![];

    for dimension in dimensions {
        result.push(LatticeDimension::new(*dimension, steps));
    }

    result
}
