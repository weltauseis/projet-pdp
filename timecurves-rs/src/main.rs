use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Specifies the input file for generating the curves.
    /// The file must be in the correct JSON format, as per the provided template.
    input: PathBuf,

    /// Specifies the name of the output file where the results will be stored.
    output: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    timelabels: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct InputStruct {
    distancematrix: Vec<Vec<f64>>,
    data: Vec<Data>,
}

fn main() {
    let cli = Cli::parse();

    println!("Input file : {}", cli.input.as_path().display());
    println!("Output file : {}", cli.output.as_path().display());

    let f = File::open(cli.input).unwrap();

    let d: InputStruct = serde_json::from_reader(f).unwrap();

    println!("Distance Matrix :\n{:?}", d.distancematrix);
}
