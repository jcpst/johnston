use clap::Clap;
use cli::{Options, SubCommand};
use johnston::lattice::{Lattice, LatticeDimension};

mod cli;

fn main() {
    let options = Options::parse();
    let sub_command = options.subcmd;

    match sub_command {
        SubCommand::Walk(args) => {
            let lattice_dimension = LatticeDimension::new(args.dimension);
            let pitches = lattice_dimension.take(args.times).collect::<Vec<_>>();

            args.output.print(pitches);
        }

        SubCommand::Scale(args) => {
            let lattice = Lattice::new(args.dimension, args.times).scale();

            args.output.print(lattice.pitches);
        }
    }
}
