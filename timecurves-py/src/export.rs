use pyo3::prelude::*;

use crate::timecurve::PyTimecurves;

const TIKZ_DRAWING_SIZE: f64 = 10.;

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
    fn export(&self, curves: &PyTimecurves) -> PyResult<String> {
        Ok(match self.ext.as_str() {
            "tikz" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::TikzExporter::new(TIKZ_DRAWING_SIZE),
                &curves.timecurves,
            ),
            _ => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::CSVExporter::new(),
                &curves.timecurves,
            ),
        })
    }
}

// Use to convert a PyObject to a MyPyClass in runtime
// fn convert_pyobject_to_timecurve(pytc: PyObject) -> PyResult<PyTimecurve> {
//     Python::with_gil(|py| -> PyResult<PyTimecurve> { pytc.extract::<PyTimecurve>(py) })
// }
