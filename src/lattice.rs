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

#[derive(Clone, Debug, PartialEq)]
pub struct LatticeDimension {
    pub connector: Pitch,
    pub current: Pitch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lattice {
    pub root: Pitch,
    pub dimensions: Vec<LatticeDimension>,
}

impl LatticeDimension {
    pub fn new(connector: Pitch) -> LatticeDimension {
        LatticeDimension {
            connector,
            current: Pitch::new(1),
        }
    }
}

impl Iterator for LatticeDimension {
    type Item = Pitch;

    fn next(&mut self) -> Option<Pitch> {
        let next_pitch = self.connector + self.current;
        self.current = next_pitch;

        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use crate::pitch::Ordinal;

    use super::*;

    #[test]
    fn dimensions_are_cool() {
        let mut dim = LatticeDimension::new(Pitch::new((3, 2)));

        println!("{:?}", dim.next());
        println!("{:?}", dim.next());
        println!("{:?}", dim.next());
        println!("{:?}", dim.next());

        assert_eq!(3, dim.connector.limit);
        assert_eq!(Ordinal::Otonal, dim.connector.ordinal);
    }
}
