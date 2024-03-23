use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::{projection::ClassicalMDS, timecurve::Timecurve};

use crate::input::PyInputData;

#[pyclass(name = "timecurve")]
#[derive(Clone)]
pub struct PyTimecurve {
    pub timecurve: Timecurve,
}

#[pymethods]

impl PyTimecurve {
    #[staticmethod]
    pub fn from_data(input_data: &PyInputData) -> PyResult<Vec<Self>> {
        let a = Timecurve::from_input_data(&input_data.inputdata, ClassicalMDS::new());
        match a {
            Ok(v) => Ok(v
                .into_iter()
                .map(|tc| PyTimecurve { timecurve: tc })
                .collect()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    fn print(&self) -> PyResult<()> {
        if self.timecurve.points.is_empty() {
            return Err(PyValueError::new_err("Timecurve is empty"));
        }
        println!("Curve for dataset '{}' :", self.timecurve.name);
        for (i, p) in self.timecurve.points.iter().enumerate() {
            println!("  {}. - {} : ({:.2}, {:.2})", i, p.label, p.pos.0, p.pos.1);
        }
        Ok(())
    }
}
