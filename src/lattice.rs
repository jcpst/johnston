use crate::pitch::Pitch;

/*
TODO: New Lattice Structure.
- Lattices are infinite.
- A line on a lattice is increments of the same ratio

This means that any pitch/ratio on the lattic can be calculated
by only know the dimension/limit it sits on, and the position.
positive position is otonal, negative is utonal.

- Dimension 2 and point 0 is tonic.
- Dimension 1 is invalid. Any dimension reduces to a prime...

An interesting problem would be determining position on the
lattice based on the ratio itself.

- determine limit
- find it's position on the dimension

# Modulation

- A scale could be described as a sequence of lattice positions.
- Any position could be treated as the root.
- From there, the same sequence of ratios could be compared
to the new root.
- Then those modulated ratios could be calculated to determine
pitch/Hz based on the original root.

Be able to generate a scale, pick a degree of that scale,
then show what the intervals would be if that degree was the root.

 */

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LatticePosition {
    pub dimension: i32,
    pub point: i32,
}

impl LatticePosition {
    pub fn determine_pitch_from_position(&self) -> Pitch {
        let mut interval = Pitch::new((self.dimension, 1));

        for _ in 1..self.point {
            interval += interval;
        }

        interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! pitch_from_position_test {
        ($($name:ident: $value:expr, $expect:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (dimension, point) = $value;
                    let position = LatticePosition { dimension, point };
                    let expected = Pitch::new($expect);
                    let result = position.determine_pitch_from_position();

                    assert_eq!(expected.ratio, result.ratio);
                }
            )*
        }
    }

    pitch_from_position_test! {
        // test name           position  expected result
        fifth_from_position:   (3, 1),   (3, 2),
        second_from_position:  (3, 2),   (9, 8),
        third_from_position:   (5, 1),   (5, 4),
        seventh_from_position: (7, 1),   (7, 4),
    }
}
