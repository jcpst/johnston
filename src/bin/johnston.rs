extern crate johnston;
use johnston::*;

fn main() {
    let x = gen_lattice(&[3], 5);
    println!("{:#?}", x);
}
