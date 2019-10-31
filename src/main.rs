use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cfg = parse_config(&args);

    println!("Searching for {}", cfg.query);
    println!("In File: {}", cfg.filename);

    let content = fs::read_to_string(cfg.filename)
        .expect("Something went wrong reading the file");

    println!("With Text:\n{}", content);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
