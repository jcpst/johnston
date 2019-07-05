mod extensions;
pub use extensions::{IntExt, Ratio};
use rug::Rational;

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

pub fn gen_lattice(_args: &[usize], steps: usize) -> Vec<Vec<Rational>> {
    // args  - &[3, 5]
    // steps - 3
    let r = |n: i32, d: i32| Rational::from((n, d));
    let mut result = vec![];

    for x in _args {
        result.push(r(*x as i32, 1).walk(steps));
        result.push(r(*x as i32, 1).recip().flatten().walk(steps));
    }

    result
    /*
    // example result:
    vec![
        vec![r(1, 1), r(3, 2), r(9, 8)],
        vec![r(5, 4), r(15, 8)],
        vec![r(25, 16), r(125, 64)],
    ]
     */
}
