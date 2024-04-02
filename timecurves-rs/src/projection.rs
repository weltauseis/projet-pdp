use nalgebra::{DMatrix, DVector};

use crate::error::{TimecurveError, TimecurveErrorKind};

/// Trait representing a projection algorithm.
pub trait ProjectionAlgorithm {
    /// Projects points described by a distance matrix onto a 2D space.
    ///
    /// # Arguments
    ///
    /// * `distance_matrix` - A reference to a vector of rows representing the distance matrix.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples (x,y) representing the projected points.
    ///
    /// # Errors
    ///
    /// Returns a `TimecurveError` if the projection fails.
    ///
    /// # Example
    ///
    /// ```
    /// use timecurves_rs::projection::ProjectionAlgorithm;
    ///
    /// let distance_matrix : Vec<Vec<f64>> = vec![
    ///     vec![0.0, 1.0, 2.0],
    ///     vec![1.0, 0.0, 3.0],
    ///     vec![2.0, 3.0, 0.0]
    /// ];
    ///
    /// let mds = ClassicalMDS::new();
    ///
    /// let result = mds.project(&distance_matrix);
    ///
    /// match result {
    ///     Ok(points) => {
    ///         for point in points {
    ///             println!("({},{})", point.0, point.1);
    ///         }
    ///     }
    ///     Err(e) => {
    ///         println!("Error while computing the projection :");
    ///         println!("{}", e);
    ///     }
    /// }
    /// ```
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Result<Vec<(f64, f64)>, TimecurveError>;
}

/// Structure representing the classical Multidimensional Scaling (MDS) algorithm.
pub struct ClassicalMDS;
impl ClassicalMDS {
    /// Creates a new instance of the classical MDS algorithm.
    /// Takes no arguments.
    pub fn new() -> Self {
        return ClassicalMDS;
    }
}

impl ProjectionAlgorithm for ClassicalMDS {
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Result<Vec<(f64, f64)>, TimecurveError> {
        let n = distance_matrix.len();
        let m = match distance_matrix.get(0) {
            Some(row) => row.len(),
            None => {
                return Err(TimecurveError::new(
                    TimecurveErrorKind::MalformedDistanceMatrix,
                    Some("Matrix is empty"),
                ))
            }
        };

        if n != m {
            return Err(TimecurveError::new(
                TimecurveErrorKind::MalformedDistanceMatrix,
                Some(&format!("Has {} rows != {} columns", n, m)),
            ));
        }

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

        // TODO : hardcoder h (la matrice de centrage) puisqu'on ne projette les points qu'en 2D

        // A is the matrix of negative square distances divided by two
        let a = DMatrix::from_fn(n, n, |i, j| {
            let v = &d[(i, j)];
            return -0.5 * v * v;
        });

        let b = &h * a * h;

        // Determine the m largest eigenvalues λ 1 , λ 2 , . . . , λ m
        // and corresponding eigenvectors e 1 , e 2 , . . . , e m of B
        // (where m is the number of dimensions desired for the output)
        let decomposition = b.symmetric_eigen();
        // une colonne <-> un vecteur propre
        // ligne n <-> valeur propre du vecteur colonne n de la matrice au dessus

        // create couples of eigenvectors / eigenvalues
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

        // matrice des m plus grands vecteurs propres
        let couples_unzipped: (Vec<f64>, Vec<DVector<f64>>) = couples.into_iter().unzip();

        let e_m = DMatrix::from_columns(couples_unzipped.1.as_slice());

        // X = Em * Lm^.5 fournit une solution du problème posé.
        // Les coordonnées des n points dans l'espace de dimension m sont les lignes de la matrice solution X
        // (matrice à n lignes et m colonnes).
        l_m.apply(|x| {
            *x = x.sqrt();
        });

        let x_mat = e_m * l_m;

        let mut points: Vec<(f64, f64)> = Vec::new();
        for i in 0..x_mat.nrows() {
            // [x_mat[(i, 0)], x_mat[(i, 1)]]
            points.push((x_mat[(i, 0)], x_mat[(i, 1)]));
        }

        return Ok(points);
    }
}
