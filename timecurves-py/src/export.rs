use pyo3::prelude::*;
use timecurves_rs::exporters::{CSVExporter, Exporter};

use crate::timecurve::PyTimecurveSet;

const TIKZ_DRAWING_SIZE: f64 = 10.;
const LINE_THICKNESS: f64 = 1.0;
const VEGA_DRAWING_SIZE: f64 = 400.;

#[pyfunction]
pub fn export_to_csv(set: &PyTimecurveSet) -> String {
    Exporter::export(&CSVExporter::new(), &set.inner)
}

#[pyfunction]
#[pyo3(signature = (set, size = TIKZ_DRAWING_SIZE, thickness = LINE_THICKNESS))]
pub fn export_to_tikz(set: &PyTimecurveSet, size: f64, thickness: f64) -> String {
    Exporter::export(
        &timecurves_rs::exporters::TikzExporter::new(size, thickness),
        &set.inner,
    )
}

#[pyfunction]
#[pyo3(signature = (set, thickness = LINE_THICKNESS))]
pub fn export_to_svg(set: &PyTimecurveSet, thickness: f64) -> String {
    Exporter::export(
        &timecurves_rs::exporters::SVGExporter::new(thickness),
        &set.inner,
    )
}

#[pyfunction]
#[pyo3(signature = (set, size = VEGA_DRAWING_SIZE))]
pub fn export_to_vegalite(set: &PyTimecurveSet, size: f64) -> String {
    Exporter::export(
        &timecurves_rs::exporters::VegaLiteExporter::new(size as u64),
        &set.inner,
    )
}
