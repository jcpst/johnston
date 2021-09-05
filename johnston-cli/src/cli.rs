use clap::{AppSettings, ArgEnum, Clap};
use comfy_table::Table;
use johnston::pitch::Pitch;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(ArgEnum, Debug)]
pub enum OutputType {
    Debug,
    Json,
    Table,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Follow the pitches down the direction of a lattice branch.
    Walk(SubOpts),
    /// Sort the Pitches in ascending order.
    Scale(SubOpts),
}

#[derive(Clap, Debug)]
pub struct SubOpts {
    #[clap(short, long)]
    pub dimension: i32,
    #[clap(short, long)]
    pub times: usize,
    #[clap(short, long, arg_enum, default_value = "json")]
    pub output: OutputType,
}

impl OutputType {
    pub fn print(self, pitches: Vec<Pitch>) {
        match self {
            OutputType::Debug => {
                for pitch in pitches {
                    println!("{:?}", pitch)
                }
            }

            OutputType::Json => {
                println!("{}", serde_json::to_string(&pitches).unwrap());
            }

            OutputType::Table => {
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
    }
}
