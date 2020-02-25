extern crate johnston;

use johnston::Lattice;

fn main() -> () {
    let lattice = Lattice::new(&[3, 5], 3);
    println!("{:?}", lattice);
    println!("");

    println!("Scale: {:?}", Lattice::new(&[3, 5], 3).scale());
    println!("");

    for dimension in lattice.dimensions {
        println!("limit: {}", dimension.limit);

        println!("  otonal");
        for pitch in &dimension.otonal {
            println!(
                "  {}/{} : {}",
                pitch.ratio.numer(),
                pitch.ratio.denom(),
                pitch.cents
            );
        }

        println!("  utonal");
        for pitch in &dimension.utonal {
            println!(
                "  {}/{} : {}",
                pitch.ratio.numer(),
                pitch.ratio.denom(),
                pitch.cents
            );
        }
    }

    for pitch in Lattice::new(&[3, 5], 3).scale() {
        print!("{} ", pitch.cents);
    }
}
