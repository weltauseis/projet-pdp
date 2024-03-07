// this module is responsible for parsing the input file

use std::error::Error;

use serde::{Deserialize, Serialize};

/// `InputData` represents the parsed input file.
///
/// It is composed of a distance matrix and a list of datasets. The structure of `Input`
/// directly corresponds to the JSON structure of the input file, which allows for easy parsing
/// with the `serde_json` library.
///
/// # Structure
/// - `distance_matrix`: A two-dimensional array representing the distances between all points of all datasets.
/// - `datasets`: A list of datasets. Each dataset is represented as a separate entity.
///
#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub distancematrix: Vec<Vec<f64>>,
    pub data: Vec<Dataset>,
}

/// `Dataset` represents a single dataset from the input file.
///
/// Each `Dataset` is composed of a unique name and a list of timepoints.
///
/// # Structure
/// - `name`: A unique identifier for the dataset.
/// - `timepoints`: A list of timepoints associated with the dataset. Each timepoint is date and time in string format.
///
///  The structure of `Dataset` directly corresponds to the JSON structure of the input file, which allows for easy parsing
/// with the `serde_json` library.
#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub timelabels: Vec<String>,
}

impl InputData {
    /// Creates a new `InputData` object from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `string` - A JSON string representing the input data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed `InputData` object or an error if parsing fails.
    pub fn from_str(string: &str) -> Result<Self, Box<dyn Error>> {
        let input: Self = serde_json::from_str(string)?;
        Ok(input)
    }

    /// Creates a new `InputData` object by parsing the contents of a JSON file.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path to the input file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed `InputData` object or an error if parsing fails.
    pub fn from_filename(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::read_to_string(filename)?;
        let input: Self = serde_json::from_str(&file)?;
        Ok(input)
    }
}
