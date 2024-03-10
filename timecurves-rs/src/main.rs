use std::{path::PathBuf, process::exit};

use clap::Parser;

use timecurves_rs::{
    exporters::{csv_exporter::CSVExporter, Exporter},
    input::InputData,
    projection::ClassicalMDS,
    timecurve::Timecurve,
};

#[derive(Parser)]
struct CommandLine {
    /// Specifies the input file for generating the curves.
    /// The file must be in the correct JSON format, as per the provided template.
    input: PathBuf,
    /// Specifies the output file for the generated curves.
    /// The file will be in the format specified by the --format option.
    output: PathBuf,
    /// Specifies the format of the output file.
    #[arg(short, long, default_value = "csv")]
    format: String,
    /// Print additional debug information to the standard output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cmd = CommandLine::parse();

    let filename = cmd.input.display().to_string();

    let input: InputData = match InputData::from_filename(&filename) {
        Ok(v) => v,
        Err(e) => {
            println!("Error while parsing the input file :");
            println!("{}", e);
            exit(1);
        }
    };

    if cmd.verbose {
        println!("Input file <{}> read.", &cmd.input.display());
        println!("Contains {} datasets :", input.data.len());
        for dataset in &input.data {
            println!("  - {}", dataset.name);
        }
    }

    let timecurves = Timecurve::from_input_data(&input, ClassicalMDS::new());

    if cmd.verbose {
        println!("Curves for datasets calculated.");

        for curve in &timecurves {
            println!("Curve for dataset '{}' :", curve.name);
            for (i, p) in curve.points.iter().enumerate() {
                println!("  {}. - {} : ({:.2}, {:.2})", i, p.label, p.pos.0, p.pos.1);
            }
        }
    }

    let exporter: Box<dyn Exporter> = match cmd.format.to_lowercase().as_str() {
        "csv" => Box::new(CSVExporter::new()),
        _ => {
            println!("Unknown output format.");
            exit(1);
        }
    };

    let output = exporter.export(&timecurves);

    match std::fs::write(&cmd.output, output) {
        Ok(_) => {
            if cmd.verbose {
                println!("Export to file <{}> successful.", &cmd.output.display());
            }
        }
        Err(e) => {
            println!(
                "Error while exporting to file <{}> :",
                &cmd.output.display()
            );
            println!("{}", e);
            exit(1);
        }
    }

    exit(0);
}
