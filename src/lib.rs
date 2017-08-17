use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


// note use of pub = public
pub struct Config {
    // note ownership matters here too!
    // this says Config owns its strings
    pub query: String,
    pub filename: String,
}

impl Config {

    // Result<,> from prelude
    // 'static is lifetime of program
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough args");
        }

        // todo: clone is inefficient, own iterator instead
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// () = unit type
// Box<> is a trait
pub fn run(config: Config) -> Result<(), Box<Error>> {
    // ? = if there is an error, propagate upwards via Result
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    // no & = passing owner = invalid to refer to contents after this call
    // only & = passing immutable reference, hence we need mut
    f.read_to_string(&mut contents)?;

    println!("Text:\n{}", contents);

    Ok(())
}

