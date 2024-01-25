use std::{fs::File, ops::Mul, path::PathBuf};

use nalgebra::DMatrix;
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

// http://www.normalesup.org/~carpenti/Notes/MDS/MDS-simple.html
// Source : Borg, I., Groenen, P., Modern Multidimensional Scaling : Theory and Applications, Second Edition, Springer, 2005.

//On effectue une opération de double centrage : B = -0.5 * J * P2 *J
//où J = In - 1/n 1n, In est la matrice unité d'ordre n, 1n est la matrice n x n dont tous les coefficients sont égaux à 1.

fn double_centering(matrix: &Vec<Vec<f64>>) -> DMatrix<f64> {
    let n = matrix.len();
    let matrix_i_n = DMatrix::from_diagonal_element(n, n, 1.0);
    let matrix_1_n = DMatrix::from_element(n, n, 1.0);

    let mut matrix_squared = DMatrix::from_element(n, n, 0.0);
    for i in 0..n {
        for j in 0..n {
            matrix_squared[(i, j)] = matrix[i][j] * matrix[i][j];
        }
    }
    let matrix_j = matrix_i_n - matrix_1_n.mul(1.0 / n as f64);
    return matrix_j.clone().mul(-0.5) * matrix_squared * matrix_j;
}

fn main() {
    let cli = Cli::parse();

    println!("Input file : {}", cli.input.as_path().display());
    println!("Output file : {}", cli.output.as_path().display());

    let f = File::open(cli.input).unwrap();

    let d: InputStruct = serde_json::from_reader(f).unwrap();

    print!("{}", double_centering(&d.distancematrix));

    println!("Distance Matrix :\n{:?}", d.distancematrix);
}
