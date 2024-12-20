use std::{env, process};

use minigrep_library::{exe::runner::run, models::config::Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("in file {}", config.file_path);

    if let Err(err) = run(config) {
        eprintln!("Aplication Error {}", err);
        process::exit(1);
    }
}
