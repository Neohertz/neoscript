use clap::Parser;
use std::{env, fs};

mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    let contents = fs::read_to_string(args.path).expect("Failed to read file.");

    parser::parse(&contents);
}
