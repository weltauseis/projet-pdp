use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::{projection::ClassicalMDS, timecurve::TimecurveSet};

use crate::input::PyInputData;

#[pyclass(name = "timecurves")]
pub struct PyTimecurves {
    pub timecurves: TimecurveSet,
}

#[pymethods]

impl PyTimecurves {
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
#[pyfunction]
pub fn timecurves_from_data(input_data: &PyInputData) -> PyResult<PyTimecurves> {
    let a = TimecurveSet::new(&input_data.inputdata, ClassicalMDS::new());
    match a {
        Ok(v) => Ok(PyTimecurves { timecurves: v }),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
