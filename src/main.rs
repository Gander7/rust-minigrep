use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}