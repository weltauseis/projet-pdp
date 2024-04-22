use pyo3::exceptions::PyAttributeError;
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
#[pyclass(name = "ProjectionAlgorithm")]
#[derive(Clone)]
pub struct PyProjAlgorithm {
    algo: Py<PyAny>,
}

impl ProjectionAlgorithm for PyProjAlgorithm {
    fn project(&self, distance_matrix: &Vec<Vec<f64>>) -> Result<Vec<Position>, TimecurveError> {
        return Python::with_gil(|py| {
            let function_result: Result<Bound<'_, PyAny>, PyErr> = self.algo.bind(py).call_method(
                "project",
                (PyList::new_bound(py, distance_matrix),),
                None,
            );

            let extracted: Result<Vec<PyRef<PyPosition>>, PyErr> = match function_result {
                Ok(a) => a.extract(),
                Err(e) => Err(e),
            };

            match extracted {
                Ok(points) => Ok(points
                    .into_iter()
                    .map(|p: PyRef<'_, PyPosition>| p.inner)
                    .collect()),
                Err(e) => Err(TimecurveError::new(
                    TimecurveErrorKind::PythonError,
                    Some(&e.to_string()),
                )),
            }
        });
    }
}

#[pymethods]
impl PyProjAlgorithm {
    #[new]
    pub fn new(algo: Py<PyAny>) -> Self {
        PyProjAlgorithm { algo }
    }

    #[staticmethod]
    pub fn classical_mds() -> PyClassicalMDS {
        ClassicalMDS::new().into()
    }

    pub fn project(&self, distance_matrix: Vec<Vec<f64>>) -> PyResult<Vec<PyPosition>> {
        println!("Project from Python calling Rust !");

        let result = ProjectionAlgorithm::project(self, &distance_matrix);
        match result {
            Ok(points) => Ok(points.into_iter().map(|p| p.into()).collect()),
            Err(e) => Err(PyErr::new::<PyAttributeError, _>(format!("{}", e))),
        }
    }
}
