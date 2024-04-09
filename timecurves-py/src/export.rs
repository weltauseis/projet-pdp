use pyo3::prelude::*;

use crate::timecurve::PyTimecurves;

const TIKZ_DRAWING_SIZE: f64 = 10.;
const VEGA_DRAWING_SIZE: f64 = 400.;

#[pyclass(name = "exporter")]
pub struct PyExporter {
    pub ext: String,
    pub size: Option<f64>,
}

#[pymethods]
impl PyExporter {
    #[new]
    #[pyo3(signature = (str = "tikz",size = None))]
    fn new(str: &str, size: Option<f64>) -> Self {
        PyExporter {
            ext: str.to_string(),
            size,
        }
    }
    fn export(&self, curves: &PyTimecurves) -> PyResult<String> {
        Ok(match self.ext.as_str() {
            "tikz" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::TikzExporter::new(
                    self.size.unwrap_or(TIKZ_DRAWING_SIZE),
                ),
                &curves.timecurves,
            ),
            "csv" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::CSVExporter::new(),
                &curves.timecurves,
            ),
            "svg" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::SVGExporter::new(),
                &curves.timecurves,
            ),
            "vegalite" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::VegaLiteExporter::new(
                    self.size.unwrap_or(VEGA_DRAWING_SIZE) as u64,
                ),
                &curves.timecurves,
            ),
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Unknown export format.",
                ))
            }
        })
    }
}

// Use to convert a PyObject to a MyPyClass in runtime
// fn convert_pyobject_to_timecurve(pytc: PyObject) -> PyResult<PyTimecurve> {
//     Python::with_gil(|py| -> PyResult<PyTimecurve> { pytc.extract::<PyTimecurve>(py) })
// }
