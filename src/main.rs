use std::{env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("query: {}\nfilename: {}", config.query, config.filename);
    run(config)
}

fn run(config: Config) {
    match fs::read_to_string(config.filename.clone()) {
        Ok(contents) => println!("With text:\n{}", contents),
        Err(e) => println!("Problem reading {}: {}", config.filename, e),
    }
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config { query, filename });
    }
}
