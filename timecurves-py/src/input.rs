use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::input::{Dataset, InputData};

#[pyclass(name = "Dataset")]
#[derive(Clone)]
pub struct PyDataset {
    inner: Dataset,
}

impl From<Dataset> for PyDataset {
    fn from(d: Dataset) -> Self {
        PyDataset { inner: d }
    }
}

#[pymethods]
impl PyDataset {
    #[new]
    pub fn new(name: &str, timelabels: Vec<String>) -> Self {
        PyDataset {
            inner: Dataset {
                name: name.to_string(),
                timelabels,
            },
        }
    }
}

#[pyclass(name = "InputData")]
pub struct PyInputData {
    inner: InputData,
}

impl From<InputData> for PyInputData {
    fn from(d: InputData) -> Self {
        PyInputData { inner: d }
    }
}

#[pymethods]
impl PyInputData {
    #[new]
    pub fn new(distancematrix: Vec<Vec<f64>>, data: Vec<PyDataset>) -> Self {
        PyInputData {
            inner: InputData {
                distancematrix,
                data: data.into_iter().map(|d| d.inner).collect(),
            },
        }
    }
}

// functions to allow for "multiple constructors"
#[pyfunction]
pub fn input_from_filename(filename: &str) -> PyResult<PyInputData> {
    match InputData::from_filename(filename) {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

#[pyfunction]
pub fn input_from_str(string: &str) -> PyResult<PyInputData> {
    match InputData::from_str(string) {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
