extern crate johnston;
use johnston::*;
use serde_json::Result;

fn main() -> Result<()> {
    let lattice = gen_lattice(&[3], 5);
    let result = serde_json::to_string(&lattice)?;
    println!("{}", result);

    Ok(())
}
