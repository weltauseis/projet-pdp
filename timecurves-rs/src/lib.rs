use nalgebra::{DMatrix, Vector2};
use rand::Rng;
use serde::{Deserialize, Serialize};

// INPUT DATA --------------------------------------------------

/// Represents a dataset from the input file.
/// Composed of a name and a list of timepoints.
#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub timelabels: Vec<String>,
}

/// Represents all data from an input file.
/// Composed of a distance matrix and a list of datasets.
#[derive(Serialize, Deserialize)]
pub struct InputData {
    distancematrix: Vec<Vec<f64>>,
    pub data: Vec<Dataset>,
}

impl InputData {
    /// Returns a new Nalgebra DMatrix constructed from the input data distance matrix.
    pub fn nalgebra_distance_matrix(&self) -> DMatrix<f64> {
        let n = self.distancematrix.len();
        return DMatrix::from_fn(n, n, |i, j| self.distancematrix[i][j]);
    }
}

// PROJECTION ALGORITHMS -----------------------------------------------

// interface d'un algorithme de projection
pub trait ProjectionAlgorithm {
    fn project(&self, distance_matrix: DMatrix<f64>) -> Vec<Vector2<f64>>;
}

// différents algorithmes

pub struct ClassicalMDS;
impl ClassicalMDS {
    // fonction new inutile ici mais je pars du principe qu'une struct d'algo peut avoir des paramètres, ex :
    // pub struct Algo {a: f64, b:f64} et
    // fn new(alpha: f64, beta: f64) -> Self { return Algo { a: alpha, b: beta }}
    pub fn new() -> Self {
        return ClassicalMDS;
    }
}
impl ProjectionAlgorithm for ClassicalMDS {
    fn project(&self, _distance_matrix: DMatrix<f64>) -> Vec<Vector2<f64>> {
        todo!()
    }
}

pub struct RandomMDS {
    min: f64,
    max: f64,
}
impl RandomMDS {
    pub fn new(_min: f64, _max: f64) -> Self {
        return RandomMDS {
            min: _min,
            max: _max,
        };
    }
}
impl ProjectionAlgorithm for RandomMDS {
    fn project(&self, distance_matrix: DMatrix<f64>) -> Vec<Vector2<f64>> {
        let n = distance_matrix.ncols();

        let mut positions = Vec::new();

        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x: f64 = rng.gen_range(self.min..self.max);
            let y: f64 = rng.gen_range(self.min..self.max);

            positions.push(Vector2::new(x, y));
        }

        return positions;
    }
}
