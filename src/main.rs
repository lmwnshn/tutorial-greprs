use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // args panics on invalid Unicode,
    // use args_os instead
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    // note that ! denotes macro in Rust
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // expect = panic if something bad happens
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    // no & = passing owner = invalid to refer to contents after this call
    // only & = passing immutable reference, hence we need mut
    f.read_to_string(&mut contents).expect("error reading file");

    println!("Text:\n{}", contents);
}

struct Config {
    // note ownership matters here too!
    // this says Config owns its strings
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // todo: clone is inefficient, own iterator instead
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
