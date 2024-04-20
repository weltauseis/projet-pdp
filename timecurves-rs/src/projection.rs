use nalgebra::{DMatrix, DVector};

use crate::{
    error::{TimecurveError, TimecurveErrorKind},
    timecurve::Position,
};

/// Trait representing a projection algorithm.
pub trait ProjectionAlgorithm {
    /// Projects points described by a distance matrix onto a 2D space.
    ///
    /// ### Arguments
    ///
    /// * `distance_matrix` - A reference to a vector of rows representing the distance matrix.
    ///
    /// ### Returns
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
    /// use timecurves_rs::projection::{ProjectionAlgorithm, ClassicalMDS};
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
    ///             println!("({},{})", point.get_x(), point.get_y());
    ///         }
    ///     }
    ///     Err(e) => {
    ///         println!("Error while computing the projection :");
    ///         println!("{}", e);
    ///     }
    /// }
    /// ```
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Result<Vec<Position>, TimecurveError>;
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
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Result<Vec<Position>, TimecurveError> {
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

        let mut points = Vec::new();
        for i in 0..x_mat.nrows() {
            // [x_mat[(i, 0)], x_mat[(i, 1)]]
            points.push(Position::new(x_mat[(i, 0)], x_mat[(i, 1)]));
        }

        return Ok(points);
    }
}

#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn classical_mds_preserves_distances() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 3.0],
            vec![2.0, 3.0, 0.0],
        ];

        let classical_mds = ClassicalMDS::new();

        let points = classical_mds.project(&distance_matrix).unwrap();

        let epsilon: f64 = 10e-3;

        // dist a <-> b
        let dist_a_b = ((points[0].get_x() - points[1].get_x()).powf(2.0)
            + (points[0].get_y() - points[1].get_y()).powf(2.0))
        .sqrt();
        assert!(dist_a_b < 1.0 + epsilon && dist_a_b > 1.0 - epsilon);

        // dist a <-> c
        let dist_a_c = ((points[0].get_x() - points[2].get_x()).powf(2.0)
            + (points[0].get_y() - points[2].get_y()).powf(2.0))
        .sqrt();
        assert!(dist_a_c < 2.0 + epsilon && dist_a_c > 2.0 - epsilon);

        // dist b <-> c
        let dist_b_c = ((points[1].get_x() - points[2].get_x()).powf(2.0)
            + (points[1].get_y() - points[2].get_y()).powf(2.0))
        .sqrt();
        assert!(dist_b_c < 3.0 + epsilon && dist_b_c > 3.0 - epsilon);
    }

    #[test]
    fn classical_mds_projects_the_right_number_of_points() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 3.0],
            vec![2.0, 3.0, 0.0],
        ];

        let classical_mds = ClassicalMDS::new();

        let points = classical_mds.project(&distance_matrix).unwrap();

        assert_eq!(points.len(), 3);

        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 1.0, 2.0, 3.0],
            vec![1.0, 0.0, 3.0, 4.0],
            vec![2.0, 3.0, 0.0, 5.0],
            vec![3.0, 4.0, 5.0, 0.0],
        ];

        let points = classical_mds.project(&distance_matrix).unwrap();

        assert_eq!(points.len(), 4);

        let input = InputData::from_filename(&format!(
            "{}/tests/psfr100points.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let points = classical_mds.project(&input.distancematrix).unwrap();

        assert_eq!(points.len(), 100);
    }
}
