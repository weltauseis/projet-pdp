use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::{
    projection::ClassicalMDS,
    timecurve::{Timecurve, TimecurveSet},
};

use crate::input::PyInputData;

#[pyclass(name = "timecurve")]
pub struct PyTimecurves {
    pub timecurves: TimecurveSet,
}

#[pymethods]

impl PyTimecurves {
    #[staticmethod]
    pub fn from_data(input_data: &PyInputData) -> PyResult<Self> {
        let a = Timecurve::from_input_data(&input_data.inputdata, ClassicalMDS::new());
        match a {
            Ok(v) => Ok(PyTimecurves { timecurves: v }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
    fn print(&self) -> PyResult<()> {
        for tc in self.timecurves.curves.iter() {
            println!("Curve for dataset '{}' :", tc.name);
            for (i, p) in tc.points.iter().enumerate() {
                println!("  {}. - {} : ({:.2}, {:.2})", i, p.label, p.pos.0, p.pos.1);
            }
        }
        Ok(())
    }
}
