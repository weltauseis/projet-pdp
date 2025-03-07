/*
* Copyright (c) 2024, Kevin Jourdain
* Copyright (c) 2024, Thibault Giloux
*
* SPDX-License-Identifier: BSD-3-Clause
*/

use clap::Parser;
use log::{debug, info};
use std::{path::PathBuf, process::exit};
use timecurves_rs::{
    exporters::{CSVExporter, Exporter, SVGExporter, TikzExporter, VegaLiteExporter},
    input::InputData,
    projection::ClassicalMDS,
    timecurve::TimecurveSet,
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
    #[arg(short, long)]
    format: String,
    /// Specifies the size of the output graph, for formats that support it. Unit is cm for Tikz, px for Vega-lite
    #[arg(short, long)]
    size: Option<f64>,
    /// Specifies the thickness of the lines in the output graph, for formats that support it.
    #[arg(long, default_value = "1.0")]
    thickness: f64,
}

fn main() {
    let cmd = CommandLine::parse();

    env_logger::init();

    let filename = cmd.input.display().to_string();

    let input: InputData = match InputData::from_filename(&filename) {
        Ok(v) => v,
        Err(e) => {
            println!("Error while parsing the input file :");
            println!("{}", e);
            exit(1);
        }
    };

    info!("Input file <{}> read.", &cmd.input.display());
    info!("Contains {} datasets :", input.get_datasets().len());
    for dataset in input.get_datasets() {
        info!("  - {}", dataset.get_name());
    }

    let timecurves = match TimecurveSet::new(&input, ClassicalMDS::new()) {
        Ok(curves) => curves,
        Err(e) => {
            println!("Error while creating the timecurves :");
            println!("{}", e);
            exit(1);
        }
    };

    info!("Curves for datasets calculated.");
    for curve in timecurves.get_curves() {
        debug!("Points for dataset '{}' :", curve.get_name());
        for (i, p) in curve.get_points().iter().enumerate() {
            debug!(
                "  {}. - {} : ({:.2}, {:.2})",
                i,
                p.get_label(),
                p.get_pos_x(),
                p.get_pos_y()
            );
        }
    }

    let exporter: Box<dyn Exporter> = match cmd.format.to_lowercase().as_str() {
        "csv" => Box::new(CSVExporter::new()),
        "tikz" => Box::new(TikzExporter::new(cmd.size.unwrap_or(10.0), cmd.thickness)),
        "svg" => Box::new(SVGExporter::new(cmd.thickness)),
        "vegalite" => Box::new(VegaLiteExporter::new(cmd.size.unwrap_or(400.0) as u64)),
        _ => {
            println!("Unknown output format.");
            exit(1);
        }
    };

    let output = exporter.export(&timecurves);

    match std::fs::write(&cmd.output, output) {
        Ok(_) => {
            info!("Export to file <{}> successful.", &cmd.output.display());
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
