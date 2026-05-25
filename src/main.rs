use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::search;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguements");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}

fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for Query: {}",config.query);
    println!("Searching in File: {}",config.file_path);


    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}