use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::projection::{ClassicalMDS, ProjectionAlgorithm};

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
