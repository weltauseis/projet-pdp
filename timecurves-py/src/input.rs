use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::input::{Dataset, InputData};

#[pyclass(name = "inputdata")]
pub struct PyInputData {
    pub inputdata: InputData,
}
#[pymethods]
impl PyInputData {}

#[pyfunction]
pub fn input_from_dataset(
    dmatrix: Vec<Vec<f64>>,
    datasets: Vec<(String, Vec<String>)>,
) -> PyResult<PyInputData> {
    let datasets = datasets
        .iter()
        .map(|(name, timelabels)| Dataset {
            name: name.clone(),
            timelabels: timelabels.clone(),
        })
        .collect();

    Ok(PyInputData {
        inputdata: InputData::from(dmatrix, datasets),
    })
}
#[pyfunction]
pub fn input_from_filename(filename: &str) -> PyResult<PyInputData> {
    match InputData::from_filename(filename) {
        Ok(v) => Ok(PyInputData { inputdata: v }),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
#[pyfunction]
pub fn input_from_str(string: &str) -> PyResult<PyInputData> {
    match InputData::from_str(string) {
        Ok(v) => Ok(PyInputData { inputdata: v }),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
