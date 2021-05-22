use johnston::{lattice::LatticeDimension, pitch::Pitch};

extern crate johnston;

fn main() {
    let lat_dim = LatticeDimension::new(Pitch::new((5, 4)));
    let notes = lat_dim.take(6);

    for note in notes {
        println!("{:?}", note);
    }
}
