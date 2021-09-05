use clap::{AppSettings, ArgEnum, Clap};
use comfy_table::Table;
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
    Table,
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
                    let pitches = lattice_dimension.take(args.times).collect::<Vec<_>>();
                    println!("{}", serde_json::to_string(&pitches).unwrap());
                }

                OutputType::Debug => {
                    for pitch in lattice_dimension.take(args.times) {
                        println!("{:?}", pitch);
                    }
                }
                OutputType::Table => {
                    print_table(lattice_dimension.take(args.times).collect::<Vec<_>>());
                }
            }
        }

        SubCommand::Scale(args) => {
            let lattice = Lattice::new(args.dimension, args.times).scale();

            match args.output {
                OutputType::Json => {
                    println!("{}", serde_json::to_string(&lattice.pitches).unwrap());
                }

                OutputType::Debug => {
                    for pitch in lattice.pitches {
                        println!("{:?}", pitch)
                    }
                }

                OutputType::Table => {
                    print_table(lattice.pitches);
                }
            }
        }
    }
}

fn print_table(pitches: Vec<Pitch>) {
    let mut table = Table::new();

    table.set_header(vec!["Ratio", "Cents", "Limit", "Ordinal"]);

    for pitch in pitches {
        table.add_row(vec![
            format!("{}/{}", pitch.ratio.numerator, pitch.ratio.denominator),
            format!("{}", pitch.cents),
            format!("{}", pitch.limit),
            format!("{:?}", pitch.ordinal),
        ]);
    }

    println!("{}", table);
}
