use pyo3::prelude::*;
mod export;
mod input;
mod timecurve;

#[pymodule]
fn timecurves_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<export::PyExporter>()?;
    //Input
    m.add_function(wrap_pyfunction!(input::input_from_filename, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_str, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_dataset, m)?)?;
    //Timecurve
    m.add_function(wrap_pyfunction!(timecurve::timecurves_from_data, m)?)?;
    Ok(())
}
