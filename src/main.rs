use std::env;
use std::fs;


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not Enough Arguements");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Config { query, file_path };
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let config   = Config::new(&args);

    println!("Searching for Query: {}",config.query);
    println!("Searching in File: {}",config.file_path);


    let contents = fs::read_to_string(config.file_path)
        .expect("Should Have been able to read the file.");
    println!("With Text:\n{contents}");
}