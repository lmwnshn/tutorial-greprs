use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


// note use of pub = public
pub struct Config {
    // note ownership matters here too!
    // this says Config owns its strings
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    // Result<,> from prelude
    // 'static is lifetime of program
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        // there are argparse-esque crates
        if args.len() < 3 {
            return Err("not enough args");
        }

        // todo: clone is inefficient, own iterator instead
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// if only real search algorithms were so simple
// see ripgrep
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // function signature is rather long
    // and the code duplication is kind of sad
    // next chapter is functional things,
    // though Rust doesn't have HKT
    // it does have ATC but I don't fully grok that yet
    // I think I should take 312..

    // query is shadowed
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}
