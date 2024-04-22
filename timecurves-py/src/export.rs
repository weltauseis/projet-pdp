use pyo3::prelude::*;

use crate::timecurve::PyTimecurveSet;

const TIKZ_DRAWING_SIZE: f64 = 10.;
const LINE_THICKNESS: f64 = 1.0;
const VEGA_DRAWING_SIZE: f64 = 400.;

#[pyclass(name = "Exporter")]
pub struct PyExporter {
    pub ext: String,
    pub size: Option<f64>,
    pub thickness: Option<f64>,
}

#[pymethods]
impl PyExporter {
    #[new]
    #[pyo3(signature = (str = "tikz",size = None, thickness = None))]
    fn new(str: &str, size: Option<f64>, thickness: Option<f64>) -> Self {
        PyExporter {
            ext: str.to_string(),
            size,
            thickness,
        }
    }

    fn export(&self, curves: &PyTimecurveSet) -> PyResult<String> {
        Ok(match self.ext.as_str() {
            "tikz" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::TikzExporter::new(
                    self.size.unwrap_or(TIKZ_DRAWING_SIZE),
                    self.thickness.unwrap_or(LINE_THICKNESS),
                ),
                &curves.inner,
            ),
            "csv" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::CSVExporter::new(),
                &curves.inner,
            ),
            "svg" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::SVGExporter::new(
                    self.thickness.unwrap_or(LINE_THICKNESS),
                ),
                &curves.inner,
            ),
            "vegalite" => timecurves_rs::exporters::Exporter::export(
                &timecurves_rs::exporters::VegaLiteExporter::new(
                    self.size.unwrap_or(VEGA_DRAWING_SIZE) as u64,
                ),
                &curves.inner,
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
