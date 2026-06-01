use std::{error::Error, str::FromStr};
mod conversions;
use clap::Parser;

use crate::conversions::{any_unit::AnyUnit};

#[derive(Parser)]
#[command(name = "converter")]
#[command(about = "A simple unit converter tool", long_about = None)]
struct Cli {
    name: String,
    unit_convert:  f64,
    from: String,
    to_text: String,
    to: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();
    let from_unit = AnyUnit::from_str(&cli.from)?; 
    let to_unit = AnyUnit::from_str(&cli.to)?;     
    let result = from_unit.convert(cli.unit_convert, &to_unit)?;
    println!("{} {} = {:.4} {}", cli.unit_convert, cli.from, result, cli.to);

    Ok(())
}


