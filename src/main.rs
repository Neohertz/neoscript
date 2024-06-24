use clap::Parser;

use std::{collections::HashMap, fs};

mod lexer;

mod ast;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

pub type GlobalMethod = dyn Fn() -> ();

fn main() {
    let args: Args = Args::parse();

    let contents = fs::read_to_string(args.path).expect("Failed to read file.");

    // Global Methods

    let mut global_functions: HashMap<String, Box<GlobalMethod>> = HashMap::new();

    //FIXME: not sure if im gonna keep this.
    global_functions.insert(
        "syscall".to_string(),
        Box::new(|| println!("woah, you tried to make a syscall!")),
    );

    ast::parser::create(lexer::tokenize(&contents));
}
