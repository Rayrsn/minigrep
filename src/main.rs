use std::{env, process};
use minigrep::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("query: {}\nfilename: {}", config.query, config.filename);
    run(config)
}


