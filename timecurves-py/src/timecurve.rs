use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::{projection::ClassicalMDS, timecurve::TimecurveSet};

use crate::input::PyInputData;

#[pyclass(name = "timecurves")]
pub struct PyTimecurves {
    pub timecurves: TimecurveSet,
}

#[pymethods]

impl PyTimecurves {
    // Might be useful for debugging
    fn print_timelabels(&self) -> PyResult<()> {
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
#[pyo3(signature =(input_data,algorithm = "mds"))]
pub fn timecurves_from_data(input_data: &PyInputData, algorithm: &str) -> PyResult<PyTimecurves> {
    let mut _algorithm = match &algorithm.to_lowercase()[..] {
        "mds" => ClassicalMDS::new(),
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unknown algorithm '{}'.",
                algorithm
            )))
        }
    };
    let a = TimecurveSet::new(&input_data.inputdata, _algorithm);
    match a {
        Ok(v) => Ok(PyTimecurves { timecurves: v }),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
