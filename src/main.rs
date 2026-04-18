use std::{
    fs::File,
    io::{Read, Write},
};

use askama::Template;
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
    let (exprs, errors) = result.into_output_errors();

    if !errors.is_empty() {
        schemix::error::print_errors(&contents, errors);
        std::process::exit(1);
    }

    let exprs = exprs.unwrap_or_default();

    let mut output_file = File::create(args.output).wrap_err("Could not create schemix output file")?;

    for expr in &exprs {
        let rendered = expr.render()?;
        output_file
            .write_all(rendered.as_bytes())
            .wrap_err("Could not write to schemix output file")?;
    }

    Ok(())
}
