/*
* Copyright (c) 2024, Kevin Jourdain
* Copyright (c) 2024, Thibault Giloux
*
* SPDX-License-Identifier: BSD-3-Clause
*/

use pyo3::prelude::*;
mod export;
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
    //Timecurves
    m.add_class::<timecurve::PyPosition>()?;
    m.add_class::<timecurve::PyTimecurvePoint>()?;
    m.add_class::<timecurve::PyTimecurve>()?;
    m.add_class::<timecurve::PyTimecurveSet>()?;
    // export
    m.add_function(wrap_pyfunction!(export::export_to_csv, m)?)?;
    m.add_function(wrap_pyfunction!(export::export_to_tikz, m)?)?;
    m.add_function(wrap_pyfunction!(export::export_to_svg, m)?)?;
    m.add_function(wrap_pyfunction!(export::export_to_vegalite, m)?)?;
    Ok(())
}
