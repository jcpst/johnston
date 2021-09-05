use clap::{AppSettings, Clap};
use johnston::{
    lattice::{Lattice, LatticeDimension},
    pitch::Pitch,
};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Follow the pitches down the direction of a lattice branch.
    Walk { dimension: i32, times: usize },
    /// Sort the Pitches in ascending order.
    Scale { dimension: i32, times: usize },
}

#[derive(Clap, Debug)]
struct SubOpts {
    #[clap(short, long)]
    dimension: i32,
    #[clap(short, long)]
    times: usize,
}

fn main() {
    let opts = Options::parse();

    match opts.subcmd {
        SubCommand::Walk { dimension, times } => {
            let lattice_dimension = LatticeDimension::new(dimension);

            for pitch in lattice_dimension.take(times) {
                println!("{:?}", pitch);
            }
        }

        SubCommand::Scale { dimension, times } => {
            let lattice = Lattice::new(dimension, times).scale();

            for pitch in lattice.pitches {
                println!("{:?}", pitch)
            }
        }
    }
}
