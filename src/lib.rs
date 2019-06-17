//! ## Utilities for generating JI pitch lattices.
//!
mod extensions;
use rug::{Rational};
pub use extensions::{IntExt, Ratio};

#[derive(Debug, PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
}

pub fn lattice_relation(prime: usize, ordinal: Ordinal) -> Rational {
    let two = Rational::from((2, 1));
    let p = Rational::from((prime, 1));
    let ratio = match ordinal {
        Ordinal::Otonal => p / two,
        Ordinal::Utonal => two / p,
    };

    ratio.flatten()
}


// TODO: write lattice generating function
// (_could end up on the Ratio trait_)
pub fn gen_lattice() -> Vec<Vec<Rational>> {
    
 
 
    // example result:
    vec![
        vec![Rational::from((1, 1)), Rational::from((3, 2)), Rational::from((9, 8))],
        vec![Rational::from((5, 4)), Rational::from((15, 8))],
        vec![Rational::from((25, 16)), Rational::from((125, 64))]
    ]
}
