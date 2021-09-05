use clap::{AppSettings, Clap};
use johnston::{lattice::LatticeDimension, pitch::Pitch};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Walk { dimension: i32, times: usize },
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
            let lat_dim = LatticeDimension::new(Pitch::new(dimension));
            let notes = lat_dim.take(times);

            for note in notes {
                println!("{:?}", note);
            }
        }
        SubCommand::Scale { dimension, times } => {
            let mut pitches = Vec::<Pitch>::new();
            let otonal = LatticeDimension::new(Pitch::new(dimension));
            let utonal = LatticeDimension::new(Pitch::new((1, dimension)));

            for o in otonal.take(times) {
                pitches.push(o);
            }

            for u in utonal.take(times) {
                pitches.push(u);
            }

            pitches.sort_unstable_by(|a, b| a.cents.partial_cmp(&b.cents).unwrap());
            pitches.dedup_by(|a, b| a.cents == b.cents);

            for pitch in pitches {
                println!("{:?}", pitch)
            }
        }
    }
}
