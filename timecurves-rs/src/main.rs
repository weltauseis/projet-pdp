use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Specifies the input file for generating the curves.
    /// The file must be in the correct JSON format, as per the provided template.
    input: PathBuf,

    /// Specifies the name of the output file where the results will be stored.
    output: PathBuf
}

fn main() {
    let cli = Cli::parse();
    
    println!("Input file : {}", cli.input.as_path().display());
    println!("Output file : {}", cli.output.as_path().display());
}
