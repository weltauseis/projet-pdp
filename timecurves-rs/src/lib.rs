use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};

/// Represents a dataset from the input file.
/// Composed of a name and a list of timepoints.
#[derive(Serialize, Deserialize)]
pub struct Dataset {
    name: String,
    timelabels: Vec<String>,
}

/// Represents all data from an input file.
/// Composed of a distance matrix and a list of datasets.
#[derive(Serialize, Deserialize)]
pub struct InputData {
    distancematrix: Vec<Vec<f64>>,
    data: Vec<Dataset>,
}

impl InputData {
    /// Returns a new Nalgebra Matrix constructed from the input data distance matrix.
    pub fn nalgebra_distance_matrix(&self) -> DMatrix<f64> {
        let n = self.distancematrix.len();
        return DMatrix::from_fn(n, n, |i, j| self.distancematrix[i][j]);
    }
}
