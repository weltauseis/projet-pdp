use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, types::PyList};
use timecurves_rs::{
    error::{TimecurveError, TimecurveErrorKind},
    projection::{ClassicalMDS, ProjectionAlgorithm},
    timecurve::Position,
};

use crate::timecurve::PyPosition;

#[pyclass(name = "ClassicalMDS")]
pub struct PyClassicalMDS {
    inner: ClassicalMDS,
}

impl From<ClassicalMDS> for PyClassicalMDS {
    fn from(p: ClassicalMDS) -> Self {
        PyClassicalMDS { inner: p }
    }
}

#[pymethods]
impl PyClassicalMDS {
    #[new]
    pub fn new() -> Self {
        ClassicalMDS::new().into()
    }

    pub fn project(&self, distance_matrix: Vec<Vec<f64>>) -> PyResult<Vec<PyPosition>> {
        let result = self.inner.project(&distance_matrix);
        match result {
            Ok(points) => Ok(points.into_iter().map(|p| p.into()).collect()),
            Err(e) => Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        }
    }
}

// https://pyo3.rs/v0.21.2/trait-bounds
#[pyclass(name = "UserAlgorithm")]
pub struct PyUserAlgorithm {
    algo: Py<PyAny>,
}

impl ProjectionAlgorithm for PyUserAlgorithm {
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Result<Vec<Position>, TimecurveError> {
        let result: Result<Vec<Position>, TimecurveError> = Python::with_gil(|py| {
            let a: Result<Vec<PyRef<PyPosition>>, PyErr> = self
                .algo
                .bind(py)
                .call_method("project", (PyList::new_bound(py, distance_matrix),), None)
                .unwrap()
                .extract();

            match a {
                Ok(points) => Ok(points
                    .into_iter()
                    .map(|p: PyRef<'_, PyPosition>| p.inner)
                    .collect()),
                Err(e) => Err(TimecurveError::new(
                    TimecurveErrorKind::MalformedDistanceMatrix,
                    None,
                )),
            }
        });

        return result;
    }
}

#[pymethods]
impl PyUserAlgorithm {
    #[new]
    pub fn new(algo: Py<PyAny>) -> Self {
        PyUserAlgorithm { algo }
    }

    pub fn project(&self, distance_matrix: Vec<Vec<f64>>) -> PyResult<Vec<PyPosition>> {
        println!("Project from Python calling Rust !");

        let result = ProjectionAlgorithm::project(self, &distance_matrix);
        match result {
            Ok(points) => Ok(points.into_iter().map(|p| p.into()).collect()),
            Err(e) => Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        }

        /* return Ok(vec![
            Position::new(0.5, 0.69).into(),
            Position::new(0.420, 0.088).into(),
        ]); */
    }
}
