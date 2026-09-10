use clap::Parser;
use std::fs;
use toml::de::Error;

mod parser;
use parser::{Config, parse_params};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    file: String,
}

fn main() {
    let cli = Cli::parse();

    let file_path = cli.file;

    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    let config: Result<Config, Error> = toml::from_str(&contents);

    match config {
        Ok(c) => {
            let params = parse_params(&c);
            println!("{}", params);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
