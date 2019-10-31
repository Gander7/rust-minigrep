use std::env;
use std::process; 

use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", cfg.query);
    println!("In File: {}", cfg.filename);

    if let Err(e) = minigrep::run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}