use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // args panics on invalid Unicode,
    // use args_os instead
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    // note that ! denotes macro in Rust
    println!("Searching for {}", query);
    println!("In file {}", filename);

    // expect = panic if something bad happens
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    // no & = passing owner = invalid to refer to contents after this call
    // only & = passing immutable reference, hence we need mut
    f.read_to_string(&mut contents).expect("error reading file");

    println!("Text:\n{}", contents);
}
