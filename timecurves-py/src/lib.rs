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
    m.add_class::<projection::PyProjAlgorithm>()?;
    // m.add_class::<export::PyExporter>()?;
    //Timecurves
    m.add_class::<timecurve::PyPosition>()?;
    m.add_class::<timecurve::PyTimecurvePoint>()?;
    m.add_class::<timecurve::PyTimecurve>()?;
    m.add_class::<timecurve::PyTimecurveSet>()?;

    Ok(())
}
