use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In File: {}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With Text:\n{}", content);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
