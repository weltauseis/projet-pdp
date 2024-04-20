use pyo3::prelude::*;
//mod export;
//mod input;
mod timecurve;

#[pymodule]
fn timecurves_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    /* m.add_class::<export::PyExporter>()?;
    //Input
    m.add_function(wrap_pyfunction!(input::input_from_filename, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_str, m)?)?;
    m.add_function(wrap_pyfunction!(input::input_from_dataset, m)?)?;
    //Timecurve
    m.add_function(wrap_pyfunction!(timecurve::timecurves_from_data, m)?)?; */

    m.add_class::<timecurve::PyPosition>()?;
    Ok(())
}
