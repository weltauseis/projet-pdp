use pyo3::prelude::*;
//mod export;
mod input;
mod projection;
mod timecurve;

#[pymodule]
fn timecurves_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //Input
    m.add_class::<input::PyDataset>()?;
    m.add_class::<input::PyInputData>()?;
    m.add_function(wrap_pyfunction!(input::input_from_filename, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_str, m)?)?;
    // Projection
    m.add_class::<projection::PyClassicalMDS>()?;
    /* m.add_class::<export::PyExporter>()?;
    //Timecurve
    m.add_function(wrap_pyfunction!(timecurve::timecurves_from_data, m)?)?; */

    m.add_class::<timecurve::PyPosition>()?;
    Ok(())
}
