use clap::{arg, Parser};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, help="Prints debug information")]
    pub verbose: bool,

    #[arg(required=true, trailing_var_arg=true, help="Input files to process")]
    pub files: Vec<String>,
}
