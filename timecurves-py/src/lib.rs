use pyo3::prelude::*;
mod export;
pub mod input;
mod timecurve;

#[pymodule]
fn timecurves_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<timecurve::PyTimecurve>()?;
    m.add_class::<export::PyExporter>()?;
    m.add_function(wrap_pyfunction!(input::input_from_filename, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_str, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_dataset, m)?)?;
    Ok(())
}
