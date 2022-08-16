use clap::{CommandFactory, Parser, ValueHint};
use prost_build::Config;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Output directory
    #[clap(short, long, default_value = ".", value_name = "DIR", value_hint = ValueHint::DirPath)]
    out: PathBuf,

    /// Search directory for imports
    #[clap(short = 'I', long, value_name = "DIR",value_hint = ValueHint::DirPath)]
    include: Vec<PathBuf>,

    #[clap(value_parser)]
    proto: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if args.proto.is_empty() {
        Args::command().print_help().unwrap();
        return
    }

    Config::new()
        .out_dir(&args.out)
        .compile_protos(&args.proto, &args.include)
        .unwrap();
    ()
}
