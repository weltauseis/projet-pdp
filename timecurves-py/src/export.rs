use pyo3::{exceptions::PyValueError, prelude::*};
use timecurves_rs::timecurve::Timecurve;

use crate::timecurve::PyTimecurve;

const TIKZ_POINT_SIZE: f64 = 0.1;
const TIKZ_CURVE_SIZE: f64 = 1.0;

#[pyclass(name = "exporter")]
pub struct PyExporter {
    pub ext: String,
}

#[pymethods]
impl PyExporter {
    #[new]
    fn new(str: Option<&str>) -> Self {
        let ext = str.unwrap_or("csv");
        PyExporter {
            ext: ext.to_string(),
        }
    }
    fn export(&self, pytc: Vec<PyObject>) -> PyResult<String> {
        let pycurves: Vec<PyResult<PyTimecurve>> = pytc
            .iter()
            .map(|tc| convert_pyobject_to_timecurve(tc.clone()))
            .collect();
        let mut curves: Vec<Timecurve> = Vec::new();
        for c in pycurves.iter() {
            match c {
                Ok(v) => {
                    curves.push(v.clone().timecurve);
                }
                Err(_) => return Err(PyValueError::new_err("Invalid timecurve")),
            }
        }
        Ok(match self.ext.as_str() {
            "tikz" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::TikzExporter::new(TIKZ_POINT_SIZE, TIKZ_CURVE_SIZE),
                &curves,
            ),
            _ => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::CSVExporter::new(),
                &curves,
            ),
        })
    }
}

fn convert_pyobject_to_timecurve(pytc: PyObject) -> PyResult<PyTimecurve> {
    Python::with_gil(|py| -> PyResult<PyTimecurve> { pytc.extract::<PyTimecurve>(py) })
}
