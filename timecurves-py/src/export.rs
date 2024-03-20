use pyo3::prelude::*;
use timecurves_rs::timecurve::Timecurve;

use crate::timecurve::PyTimecurve;

const TIKZ_POINT_SIZE: f64 = 0.1;
const TIKZ_CURVE_SIZE: f64 = 1.0;

#[pyclass]
pub struct PyExporter {
    pub ext: String,
}

#[pymethods]
impl PyExporter {
    #[new]
    fn new(str: Option<&str>) -> Self {
        let ext = str.unwrap_or("csv");
        match ext {
            "tikz" => PyExporter {
                ext: "tikz".to_string(),
            },
            _ => PyExporter {
                ext: "csv".to_string(),
            },
        }
    }
    fn export(&self, pytc: Vec<PyObject>) -> String {
        let curves: Vec<Option<PyTimecurve>> = pytc
            .iter()
            .map(|pytc| convert_pyobject_to_timecurve(pytc.clone()))
            .collect();
        let curves: Vec<Timecurve> = curves
            .iter()
            .filter(|tc| tc.is_some())
            .map(|tc| tc.clone().unwrap().timecurve.clone())
            .collect();
        match self.ext.as_str() {
            "tikz" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::TikzExporter::new(TIKZ_POINT_SIZE, TIKZ_CURVE_SIZE),
                &curves,
            ),
            _ => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::CSVExporter::new(),
                &curves,
            ),
        }
    }
}

fn convert_pyobject_to_timecurve(pytc: PyObject) -> Option<PyTimecurve> {
    Python::with_gil(|py| -> Option<PyTimecurve> {
        let a = pytc.extract::<PyTimecurve>(py);
        match &a {
            Ok(v) => Some(v.clone()),
            Err(_) => None,
        };
        a.ok()
    })
}
