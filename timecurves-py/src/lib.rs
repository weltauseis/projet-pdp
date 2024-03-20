#[warn(unused_imports)]
use crate::input::Dataset;
use crate::input::InputData;
use pyo3::prelude::*;
use pyo3::Python;
use timecurves_rs::*;

#[derive(Clone)]
#[pyclass]
struct PyInputData {
    distancematrix: Vec<Vec<f64>>,
    data: Vec<PyDataset>,
}
#[pymethods]
impl PyInputData {
    #[staticmethod]
    fn new() -> Self {
        PyInputData {
            distancematrix: Vec::new(),
            data: Vec::new(),
        }
    }
    #[getter]
    pub fn get_dmatrix(&self) -> Vec<Vec<f64>> {
        self.distancematrix.clone()
    }
    #[getter]
    pub fn get_datasets(&self) -> Vec<PyDataset> {
        self.data.clone()
    }
}

#[derive(Clone)]
#[pyclass]
struct PyDataset {
    name: String,
    timelabels: Vec<String>,
}

#[pymethods]
impl PyDataset {
    #[staticmethod]
    fn new() -> Self {
        PyDataset {
            name: String::new(),
            timelabels: Vec::new(),
        }
    }
    #[getter]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    #[getter]
    pub fn get_timelabels(&self) -> Vec<String> {
        self.timelabels.clone()
    }
}
#[pyfunction]
fn input_data(filename: &str) -> PyResult<PyInputData> {
    let input = match InputData::from_filename(filename) {
        Ok(v) => parse_data_to_py_data(v),
        Err(e) => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Error: {}",
                e
            )))
        }
    };
    Ok(input)
}

// Parsing
fn parse_to_py_set(dataset: Dataset) -> PyDataset {
    let mut pyds = PyDataset::new();
    pyds.name = dataset.name;
    pyds.timelabels = dataset.timelabels;
    return pyds;
}
fn parse_data_to_py_data(input: InputData) -> PyInputData {
    let mut pyds = PyInputData::new();
    pyds.distancematrix = input.distancematrix;
    for dataset in input.data {
        pyds.data.push(parse_to_py_set(dataset));
    }
    return pyds;
}

/// A Python module implemented in Rust.gi
#[pymodule]
fn timecurves_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInputData>()?;
    m.add_class::<PyDataset>()?;
    m.add_function(wrap_pyfunction!(input_data, m)?)?;
    Ok(())
}
