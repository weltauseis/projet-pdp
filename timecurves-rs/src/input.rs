// this module is responsible for parsing the input file

use std::error::Error;

use serde::{Deserialize, Serialize};

/// Represents the parsed input file.
/// Composed of a distance matrix and a list of datasets.
/// Maps directly to the json structure of the input file for easy parsing with `serde_json`.
#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub distancematrix: Vec<Vec<f64>>,
    pub data: Vec<Dataset>,
}

/// Represents a dataset from the input file.
/// Composed of a name and a list of string timepoints.
#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub timelabels: Vec<String>,
}

impl InputData {
    pub fn from_str(string: &str) -> Result<InputData, Box<dyn Error>> {
        let input: Self = serde_json::from_str(string)?;
        Ok(input)
    }

    pub fn from_filename(filename: &str) -> Result<InputData, Box<dyn Error>> {
        let file = std::fs::read_to_string(filename)?;
        let input: Self = serde_json::from_str(&file)?;
        Ok(input)
    }
}
