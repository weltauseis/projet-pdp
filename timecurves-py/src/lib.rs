use pyo3::prelude::*;
mod export;
pub mod input;
mod timecurve;

#[pymodule]
fn timecurves_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<input::PyInputData>()?;
    m.add_class::<timecurve::PyTimecurve>()?;
    m.add_class::<export::PyExporter>()?;
    // m.add_function(wrap_pyfunction!(timecurve::timecurves_from_data, m)?)?;
    Ok(())
}
