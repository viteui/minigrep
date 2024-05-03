// in main.rs
use std::{env, fs};

struct Config {
    query: String,
    file_path: String
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("{} In file {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}


fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone();
    let file_path  = args[2].clone();
   Config{
    query,
    file_path
   }
}
