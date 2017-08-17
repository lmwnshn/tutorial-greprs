extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    // args panics on invalid Unicode,
    // use args_os instead
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(
            &mut stderr,
            "Problem parsing args: {}",
            err
        ).expect("Couldn't write to stderr");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "Error: {}",
            e
        ).expect("Couldn't write to stderr");
        process::exit(1);
    }
}

