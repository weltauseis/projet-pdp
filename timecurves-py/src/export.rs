use pyo3::prelude::*;
use timecurves_rs::timecurve::Timecurve;

use crate::timecurve::PyTimecurve;

#[pyclass]
pub struct CSVExporter {
    pub inner: timecurves_rs::exporters::CSVExporter,
}

#[pymethods]
impl CSVExporter {
    #[new]
    fn new() -> Self {
        CSVExporter {
            inner: timecurves_rs::exporters::CSVExporter::new(),
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
        timecurves_rs::exporters::Exporter::export(&self.inner, &curves)
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
#[pyfunction]
pub fn export_csv(curves: Vec<PyObject>) -> String {
    let exporter = CSVExporter::new();
    exporter.export(curves)
}
