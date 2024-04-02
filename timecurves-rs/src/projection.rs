use nalgebra::{DMatrix, DVector};

use crate::error::{TimecurveError, TimecurveErrorKind};

pub trait ProjectionAlgorithm {
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Vec<(f64, f64)>;
    fn dataset_global_test(&self,distance_matrix: &Vec<Vec<f64>>) -> Result<(),TimecurveError>;
    fn matrix_test(&self,distance_matrix: &Vec<Vec<f64>>) -> Result<(),TimecurveError>;
    fn projection_coherence_test(&self,distance_matrix: &Vec<Vec<f64>>) -> Result<(),TimecurveError>;

}

pub struct ClassicalMDS;
impl ClassicalMDS {
    pub fn new() -> Self {
        return ClassicalMDS;
    }
}
// TODO : (FACILE) rajouter la crate log (https://github.com/rust-lang/log) pour remplacer les printf de débug
// TODO : implémenter la gestion d'erreur pour cette fonction
//        par exemple, une matrice non carrée ou un nombre de points différent de la taille de la matrice


impl ProjectionAlgorithm for ClassicalMDS {
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Vec<(f64, f64)> {
        let n = distance_matrix.len();

        let d = DMatrix::from_fn(n, n, |i, j| distance_matrix[i][j]);

        // https://rich-d-wilkinson.github.io/MATH3030/6-1-classical-mds.html
        // https://en.wikipedia.org/wiki/Multidimensional_scaling
        // http://www.normalesup.org/~carpenti/Notes/MDS/MDS-simple.html
        // TODO : à modifier selon la méthode du livre Modern Multidimensional Scaling

        // Given a distance matrix D, the centred inner-product matrix
        // (also called the centred-Gram matrix) is B = HAH 🤯

        // The centering matrix is H = In - (1/n) * (1n * 1nT)
        // where In is the n×n identity matrix, and 1n is an n×1 column vector of ones.

        let identity = DMatrix::from_diagonal_element(n, n, 1.0);
        let matrix_of_ones = DMatrix::from_element(n, n, 1.0);

        let h = identity - (1.0 / n as f64) * matrix_of_ones;

        //println!("H = {:.2}", h); // correct, comparer avec C3 dans https://en.wikipedia.org/wiki/Centering_matrix 🤓

        // TODO : hardcoder h (la matrice de centrage) puisqu'on ne projette les points qu'en 2D

        // A is the matrix of negative square distances divided by two

        let a = DMatrix::from_fn(n, n, |i, j| {
            let v = &d[(i, j)];
            return -0.5 * v * v;
        });

        let b = &h * a * h;

        //println!("B = {:.2}", &b);

        // Determine the m largest eigenvalues λ 1 , λ 2 , . . . , λ m
        // and corresponding eigenvectors e 1 , e 2 , . . . , e m of B
        // (where m is the number of dimensions desired for the output)

        let decomposition = b.symmetric_eigen();

        // create couples of eigenvectors / eigenvalues

        //println!("eigenvectors : {:.2}", &decomposition.eigenvectors); // une colonne <-> un vecteur propre
        //println!("eigenvalues : {:.2}", &decomposition.eigenvalues); // ligne n <-> valeur propre du vecteur colonne n de la matrice au dessus

        let mut couples: Vec<(f64, DVector<f64>)> = Vec::new();

        for i in 0..decomposition.eigenvalues.nrows() {
            couples.push((
                decomposition.eigenvalues[i],
                DVector::from_column_slice(decomposition.eigenvectors.column(i).as_slice()),
            ));
        }

        couples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().reverse());

        while couples.len() > 2 {
            couples.pop();
        }

        // matrice diagonale des m plus grandes valeurs propres
        let mut l_m = DMatrix::from_fn(2, 2, |i, j| if i == j { couples[i].0 } else { 0.0 });

        //println!("Lm = {:.2}", &l_m);

        // matrice des m plus grands vecteurs propres

        let couples_unzipped: (Vec<f64>, Vec<DVector<f64>>) = couples.into_iter().unzip();

        let e_m = DMatrix::from_columns(couples_unzipped.1.as_slice());

        //println!("Em = {:.2}", &e_m);

        // X = Em * Lm^.5 fournit une solution du problème posé.
        // Les coordonnées des n points dans l'espace de dimension m sont les lignes de la matrice solution X
        // (matrice à n lignes et m colonnes).

        l_m.apply(|x| {
            *x = x.sqrt();
        });

        let x_mat = e_m * l_m;

        //println!("X = {:.2}", &x_mat);

        let mut points: Vec<(f64, f64)> = Vec::new();
        for i in 0..x_mat.nrows() {
            // [x_mat[(i, 0)], x_mat[(i, 1)]]
            points.push((x_mat[(i, 0)], x_mat[(i, 1)]));
        }

        return points;
    }
    
    fn dataset_global_test(&self,distance_matrix: &Vec<Vec<f64>>) -> Result<(),TimecurveError> {
        match self.projection_coherence_test(distance_matrix) {
            Ok(_) => {
                match self.projection_coherence_test(distance_matrix) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
            
        }
    }
    
    fn matrix_test(&self,distance_matrix: &Vec<Vec<f64>>) -> Result<(), TimecurveError> {
        let n = distance_matrix.len();
    
        if n == 0 {
            return Err(TimecurveError::new(
                TimecurveErrorKind::NonSquareDistanceMatrix,
                Some("Distance matrix is empty !"),
            ));
        }
    
        for row in distance_matrix {
            if row.len() != n  {
                return Err(TimecurveError::new(
                    TimecurveErrorKind::NonSquareDistanceMatrix,
                    Some(&format!(
                        "Distance matrix has {} rows ≠ {} columns !",
                        n,
                        row.len()
                    )),
                ));
            }
        }
        return Ok(());
    }
    
    fn projection_coherence_test(&self,distance_matrix: &Vec<Vec<f64>>) -> Result<(),TimecurveError> {
        let n_matrix = distance_matrix.len();
        let n_points = self.project(distance_matrix).len();
        if n_matrix != n_points {
            return Err(TimecurveError::new(
                TimecurveErrorKind::NonSquareDistanceMatrix,
                Some(&format!(
                    "Distance matrix has {} rows and columns but the projection has {} points !",
                    n_matrix,
                    n_points,
                )),
            ));
        }
        return Ok(());
    }
    
}
