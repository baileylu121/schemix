use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("Input {}!", args.input);
    println!("Output {}!", args.output);
}
