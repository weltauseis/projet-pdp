use std::error::Error;

use serde::{Deserialize, Serialize};

/// Structure representing a single dataset.
///
/// Each `Dataset` is composed of a unique name and a list of timepoints.
///
/// # Structure
/// - `name`: A unique identifier for the dataset.
/// - `timepoints`: A list of timepoints associated with the dataset. Each timepoint should be either an ISO 8601 date and time string, or a simple number.
///
///  The structure of `Dataset` directly corresponds to the JSON structure of the input file, which allows for easy parsing
/// thanks to the `serde_json` library.
#[derive(Serialize, Deserialize, Clone)]
pub struct Dataset {
    name: String,
    timelabels: Vec<String>,
}

impl Dataset {
    /// Returns the name of the dataset.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the list of time point labels associated with the dataset.
    pub fn get_timelabels(&self) -> &Vec<String> {
        &self.timelabels
    }

    /// Creates a new `Dataset` object with the given name and list of time point labels.
    pub fn new(name: &str, timelabels: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            timelabels,
        }
    }
}

/// Structure representing the parsed input file.
///
/// It is composed of a distance matrix and a list of datasets. The structure of `Input`
/// directly corresponds to the JSON structure of the input file, which allows for easy parsing
/// thanks to the `serde_json` library.
///
/// # Structure
/// - `distance_matrix`: A two-dimensional array representing the distances between all points of all datasets.
/// - `datasets`: A list of datasets. Each dataset is represented as a separate entity.
///
#[derive(Serialize, Deserialize)]
pub struct InputData {
    distancematrix: Vec<Vec<f64>>,
    data: Vec<Dataset>,
}

impl InputData {
    /// Creates a new `InputData` object from a JSON string.
    ///
    /// ### Arguments
    ///
    /// * `string` - A JSON string representing the input data.
    ///
    /// ### Returns
    ///
    /// Returns a `Result` containing the parsed `InputData` object or an error if parsing fails.
    pub fn from_str(string: &str) -> Result<Self, Box<dyn Error>> {
        let input: Self = serde_json::from_str(string)?;
        Ok(input)
    }

    /// Creates a new `InputData` object by parsing the contents of a JSON file.
    ///
    /// ### Arguments
    ///
    /// * `filename` - The path to the input file.
    ///
    /// ### Returns
    ///
    /// Returns a `Result` containing the parsed `InputData` object or an error if parsing fails.
    pub fn from_filename(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::read_to_string(filename)?;
        let input: Self = serde_json::from_str(&file)?;
        Ok(input)
    }

    /// Creates a new `InputData` object from a distance matrix and a list of datasets.
    ///
    /// ### Arguments
    ///
    /// * `dmatrix` - A 2D vector representing the distance matrix.
    /// * `datasets` - A vector of `Dataset` objects.
    ///
    /// ### Returns
    ///
    /// Returns a new `InputData` object initialized with the given distance matrix and datasets.
    pub fn from(dmatrix: Vec<Vec<f64>>, datasets: Vec<Dataset>) -> Self {
        InputData {
            distancematrix: dmatrix,
            data: datasets,
        }
    }

    /// Returns a reference to the distance matrix.
    pub fn get_distance_matrix(&self) -> &Vec<Vec<f64>> {
        &self.distancematrix
    }

    /// Returns a reference to the list of datasets.
    pub fn get_datasets(&self) -> &Vec<Dataset> {
        &self.data
    }
}
