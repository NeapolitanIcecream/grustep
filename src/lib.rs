use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}
