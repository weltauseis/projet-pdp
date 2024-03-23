use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::input::{Dataset, InputData};

#[pyclass(name = "inputdata")]
pub struct PyInputData {
    pub inputdata: InputData,
}
#[pymethods]
impl PyInputData {
    #[staticmethod]
    pub fn from(dmatrix: Vec<Vec<f64>>, datasets: Vec<(String, Vec<String>)>) -> PyResult<Self> {
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
    #[staticmethod]
    pub fn from_filename(filename: &str) -> PyResult<Self> {
        match InputData::from_filename(filename) {
            Ok(v) => Ok(PyInputData { inputdata: v }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    #[staticmethod]
    pub fn from_str(string: &str) -> PyResult<Self> {
        match InputData::from_str(string) {
            Ok(v) => Ok(PyInputData { inputdata: v }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}
