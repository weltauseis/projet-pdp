use clap::Parser;
use std::{fs::File, path::PathBuf};

use timecurves_rs::InputData;

#[derive(Parser)]
struct Cli {
    /// Specifies the input file for generating the curves.
    /// The file must be in the correct JSON format, as per the provided template.
    input: PathBuf,

    /// Specifies the name of the output file where the results will be stored.
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Input file : {}", cli.input.as_path().display());
    println!("Output file : {}", cli.output.as_path().display());

    let f = File::open(cli.input).unwrap();

    let input_data: InputData = serde_json::from_reader(f).unwrap();

    println!(
        "Distance matrix : {:.1}",
        input_data.nalgebra_distance_matrix()
    );
}
