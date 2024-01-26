use std::{fs::File,ops::Mul, path::PathBuf};

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




fn double_centering(matrix : &Vec<Vec<f64>>) ->DMatrix<f64>{
    let n = matrix.len();
    let matrix_i_n = DMatrix::from_diagonal_element(n, n, 1.0);
    let matrix_1_n = DMatrix::from_element(n, n, 1.0);

    let mut matrix_squared = DMatrix::from_element(n, n, 0.0);
    for i in 0..n{
        for j in 0..n{
            matrix_squared[(i,j)] = matrix[i][j]*matrix[i][j];
        }
    }
    let matrix_j = matrix_i_n - matrix_1_n.mul(1.0/n as f64);
    return matrix_j.clone().mul(-0.5) * matrix_squared * matrix_j
}


    fn eigen2(matrix : DMatrix<f64>) -> DMatrix<f64>{
        let mut list = Vec::new();
        // Calcul des valeurs propres
        //eigen en fonction a des valeurs un peu différentes que celui fait à la main
        matrix.symmetric_eigen().eigenvalues.iter().for_each(|x| list.push(*x));

        list.sort_by(|a,b| a.partial_cmp(b).unwrap());
        
        let n = list.len();
        let mut lm = DMatrix::from_element(n,n, 0.0);
        for i in 0..n-2{
            lm[(i,i)] = 0.0;
        }
        lm[(n-2,n-2)] = list[n-2];
        lm[(n-1,n-1)] = list[n-1];
        
        return lm.map(|x| x.sqrt());
    }


    fn eigen_vector2(matrix : DMatrix<f64>) -> DMatrix<f64> {
        // Calcul des vecteurs propres
        let eigendecomp = matrix.symmetric_eigen();
        let vectors = eigendecomp.eigenvectors;
    
        vectors
    }
/*
url : https://www.normalesup.org/~carpenti/Notes/MDS/MDS-simple.html

On part d'une matrice n x n de distances mutuelles entre objets : P
On calcule P2, matrice n x n des carrés des distances précédentes
On effectue une opération de double centrage : B = -0.5 * J * P2 *J
où J = In - 1/n 1n, In est la matrice unité d'ordre n, 1n est la matrice n x n dont tous les coefficients sont égaux à 1.
On calcule les valeurs propres et vecteurs propres (unitaires) de la matrice B. On conserve les m plus grandes valeurs propres. Soit Lm la matrice diagonale de ces m valeurs propres, et Em la matrice des vecteurs propres unitaires correspondants. Soit Lm^.5 la matrice des racines carrées de ces valeurs propres.
X = Em * Lm^.5 fournit une solution du problème posé. Les coordonnées des n points dans l'espace de dimension n sont les lignes de la matrice solution X (matrice à n lignes et m colonnes). */
fn main() {
    let cli = Cli::parse();

    println!("Input file : {}", cli.input.as_path().display());
    println!("Output file : {}", cli.output.as_path().display());

    let f = File::open(cli.input).unwrap();

    let d: InputStruct = serde_json::from_reader(f).unwrap();

    let em = eigen_vector2(double_centering(&d.distancematrix));


    let lm = eigen2(double_centering(&d.distancematrix));

    let x = em * lm;
    
    for i in 0..3{
        println!("{:?} : {}:::{}",d.data[0].timelabels[i],x[(i,1)],x[(i,2)]);
    }
}
