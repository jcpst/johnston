use clap::{AppSettings, ArgEnum, Clap};
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

#[derive(ArgEnum, Debug)]
enum OutputType {
    Debug,
    Json,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Follow the pitches down the direction of a lattice branch.
    Walk(SubOpts),
    /// Sort the Pitches in ascending order.
    Scale(SubOpts),
}

#[derive(Clap, Debug)]
struct SubOpts {
    #[clap(short, long)]
    dimension: i32,
    #[clap(short, long)]
    times: usize,
    #[clap(short, long, arg_enum)]
    output: OutputType,
}

fn main() {
    let command = Options::parse().subcmd;

    match command {
        SubCommand::Walk(args) => {
            let lattice_dimension = LatticeDimension::new(args.dimension);

            match args.output {
                OutputType::Json => {
                    let pitches: Vec<Pitch> = lattice_dimension.take(args.times).collect();
                    println!("{}", serde_json::to_string(&pitches).unwrap());
                }

                OutputType::Debug => {
                    for pitch in lattice_dimension.take(args.times) {
                        println!("{:?}", pitch);
                    }
                }
            }
        }

        SubCommand::Scale(args) => {
            let lattice = Lattice::new(args.dimension, args.times).scale();

            match args.output {
                OutputType::Json => {
                    println!("{}", serde_json::to_string(&lattice).unwrap());
                }

                OutputType::Debug => {
                    for pitch in lattice.pitches {
                        println!("{:?}", pitch)
                    }
                }
            }
        }
    }
}
