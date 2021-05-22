use johnston::{lattice::LatticeDimension, pitch::Pitch};

extern crate johnston;

fn main() {
    let lat_dim = LatticeDimension::new(Pitch::new((7, 4)));
    let notes = lat_dim.take(10);

    for note in notes {
        println!("{:?}", note);
    }
}
