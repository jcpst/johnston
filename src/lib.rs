mod pitch;

use num::rational::Ratio;
use pitch::Pitch;

#[derive(PartialEq)]
pub enum Ordinal {
    Otonal,
    Utonal,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn p(n: i32, d: i32) -> Pitch {
        Pitch::new((n, d))
    }

    #[test]
    fn walk_works() {
        let expected = vec![p(1, 1), p(3, 2), p(9, 8)];
        let actual = Pitch::new((3, 2)).walk(3);

        assert_eq!(expected, actual);
    }
}
