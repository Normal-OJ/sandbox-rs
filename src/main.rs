mod env;

use std::fs;
use std::path::Path;
use clap::Parser;
use env::Env;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "")]
    file: String,
}

fn parse_env_from_file(file_path: &str) -> Env {
    if let Ok(file_content) = fs::read_to_string(Path::new(file_path)) {
        Env::new(&file_content).unwrap()
    } else {
        Env::new("").unwrap()
    }
}

fn main() {
    let args = Args::parse();

    let environment = parse_env_from_file(&args.file);
}