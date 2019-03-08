use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a filename")
        };

        return Ok(Config { query, filename });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Reading file.");

    let contents = fs::read_to_string(config.filename)?;

    println!("Scanning file.");

    for line in search(&config.query, &contents) {
        println!("{}", line);
    };

    return Ok(());
}

fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
    return contents.lines().filter(|line| line.contains(query)).collect();
}
