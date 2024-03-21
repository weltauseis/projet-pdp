use pyo3::prelude::*;
use timecurves_rs::{projection::ClassicalMDS, timecurve::Timecurve};

use crate::input::PyInputData;

#[pyclass(name = "timecurve")]
#[derive(Clone)]
pub struct PyTimecurve {
    pub timecurve: Timecurve,
}

#[pymethods]

impl PyTimecurve {
    #[new]
    pub fn new_empty(name: &str) -> Self {
        PyTimecurve {
            timecurve: Timecurve::new_empty(name),
        }
    }
    #[warn(private_interfaces)]
    pub fn from_input_data(&mut self, input_data: &PyInputData) -> Vec<Self> {
        let a = Timecurve::from_input_data(&input_data.inputdata, ClassicalMDS::new());
        match a {
            Ok(v) => v
                .into_iter()
                .map(|tc| PyTimecurve { timecurve: tc })
                .collect(),
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    pub fn print(&self) {
        println!("Curve for dataset '{}' :", self.timecurve.name);
        for (i, p) in self.timecurve.points.iter().enumerate() {
            println!("  {}. - {} : ({:.2}, {:.2})", i, p.label, p.pos.0, p.pos.1);
        }
    }
}
