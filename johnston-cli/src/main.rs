use clap::{AppSettings, ArgEnum, Clap};
use johnston::lattice::{Lattice, LatticeDimension};

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
            let pitches = lattice_dimension.take(args.times).collect::<Vec<_>>();

            match args.output {
                OutputType::Json => print::json(pitches),
                OutputType::Debug => print::debug(pitches),
                OutputType::Table => print::table(pitches),
            }
        }

        SubCommand::Scale(args) => {
            let lattice = Lattice::new(args.dimension, args.times).scale();

            match args.output {
                OutputType::Json => print::json(lattice.pitches),
                OutputType::Debug => print::debug(lattice.pitches),
                OutputType::Table => print::table(lattice.pitches),
            }
        }
    }
}

mod print {
    use comfy_table::Table;
    use johnston::pitch::Pitch;

    pub fn json(pitches: Vec<Pitch>) {
        println!("{}", serde_json::to_string(&pitches).unwrap());
    }

    pub fn debug(pitches: Vec<Pitch>) {
        for pitch in pitches {
            println!("{:?}", pitch)
        }
    }

    pub fn table(pitches: Vec<Pitch>) {
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
}
