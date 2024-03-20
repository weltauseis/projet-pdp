#[warn(unused_imports)]
use crate::input::Dataset;
use crate::input::InputData;
use pyo3::prelude::*;
use pyo3::Python;
use timecurve::PyTimecurve;
use timecurves_rs::*;
mod export;
mod timecurve;
// Add missing import statement for blalux
#[pyclass]
pub struct PyInputData {
    inputdata: InputData,
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

/// A Python module implemented in Rust.gi
#[pymodule]
fn timecurves_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInputData>()?;
    m.add_class::<PyTimecurve>()?;
    m.add_class::<export::PyExporter>()?;
    Ok(())
}
