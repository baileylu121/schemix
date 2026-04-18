use std::{fs::File, io::Read};

use chumsky::Parser as ChumskyParser;
use clap::Parser;
use color_eyre::eyre::Result;
use eyre::Context;

/// Nix modules options DSL
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the input schemix file
    #[arg(short, long)]
    input: String,

    /// Path to place the generated nix options in
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    color_eyre::install()?;

    let mut input = File::open(args.input).wrap_err("Could not open schemix input file")?;

    let mut contents = String::new();
    input
        .read_to_string(&mut contents)
        .wrap_err("Could not read schemix input file")?;

    let result = schemix::parser().parse(&contents);
    
    if result.has_errors() {
        schemix::parser::print_errors(&contents, result.errors().collect());
        std::process::exit(1);
    }

    println!("{:?}", result.into_result());

    // let mut output = File::create(args.output).wrap_err("Could not create schemix output file")?;

    Ok(())
}
