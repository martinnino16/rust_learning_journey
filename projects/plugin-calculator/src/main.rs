use std::process;
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "calc")]
#[command(version, about, long_about = None)]
struct Config {
    name: String,
    input: String
}

fn main() {
    let config = Config::parse();
    if let Err(e) = plugin_calculator::run(&config.input) {
        eprintln!("Error application {e}");
        process::exit(1);
    }
}
