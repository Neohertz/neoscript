use std::{env, io};

fn main() {

    for arg in env::args() {
        println!("{arg}");
        program(&arg);
    }

}

fn program(arg: &String) {
    match arg.to_lowercase().trim() {
        "-" => println!("Hello!"),
        _ => {}
    }
}