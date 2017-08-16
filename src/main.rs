use std::env;

fn main() {
    // args panics on invalid Unicode,
    // use args_os instead
    let args: Vec<String> = env::args().collect();
    // :? is a debug formatter
    // by default, just {} placeholders
    println!("{:?}", args);
}
