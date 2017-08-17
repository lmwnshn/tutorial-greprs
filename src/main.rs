use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // args panics on invalid Unicode,
    // use args_os instead
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

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

impl Config {

    // Result<,> from prelude
    // 'static is lifetime of program
    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough args");
        }

        // todo: clone is inefficient, own iterator instead
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
