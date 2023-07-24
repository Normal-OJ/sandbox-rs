use std::fs;
use std::path::Path;

use clap::Parser;

use libnoj_rs::Env;
use libnoj_rs::run;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "")]
    env_path: String,
    #[arg(short, long, default_value = "")]
    dl_path: String,
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
    let environment = parse_env_from_file(&args.env_path);
    run(args.dl_path, environment)
}