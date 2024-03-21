use pyo3::prelude::*;
use timecurves_rs::input::{Dataset, InputData};

#[pyclass(name = "inputdata")]
pub struct PyInputData {
    pub inputdata: InputData,
}
#[pymethods]
impl PyInputData {
    #[new]
    fn new() -> Self {
        PyInputData {
            inputdata: InputData {
                distancematrix: Vec::new(),
                data: Vec::new(),
            },
        }
    }

    fn from_filename(&mut self, filename: &str) -> () {
        self.inputdata = match InputData::from_filename(filename) {
            Ok(v) => v,
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    fn from_str(&mut self, string: &str) -> () {
        self.inputdata = match InputData::from_str(string) {
            Ok(v) => v,
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
    fn from(&mut self, dmatrix: Vec<Vec<f64>>, datasets: Vec<(String, Vec<String>)>) -> () {
        let datasets = datasets
            .iter()
            .map(|(name, timelabels)| Dataset {
                name: name.clone(),
                timelabels: timelabels.clone(),
            })
            .collect();
        self.inputdata = InputData::from(dmatrix, datasets);
    }

    #[getter]
    fn distancematrix(&self) -> Vec<Vec<f64>> {
        self.inputdata.distancematrix.clone()
    }
}
