use pyo3::prelude::*;
use timecurves_rs::timecurve::Position;

#[pyclass(name = "position")]
pub struct PyPosition {
    inner: Position,
}

impl From<Position> for PyPosition {
    fn from(p: Position) -> Self {
        PyPosition { inner: p }
    }
}

#[pymethods]
impl PyPosition {
    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        PyPosition::from(Position::new(x, y))
    }

    pub fn get_x(&self) -> f64 {
        self.inner.get_x()
    }

    pub fn get_y(&self) -> f64 {
        self.inner.get_y()
    }
}

/* #[pyclass(name = "timecurves")]
pub struct PyTimecurves {
    pub timecurves: TimecurveSet,
}

#[pymethods]

impl PyTimecurves {
    // Might be useful for debugging
    fn print_timelabels(&self) -> PyResult<()> {
        for tc in self.timecurves.curves.iter() {
            println!("Curve for dataset '{}' :", tc.name);
            for (i, p) in tc.points.iter().enumerate() {
                println!("  {}. - {} : ({:.2}, {:.2})", i, p.label, p.pos.0, p.pos.1);
            }
        }
        Ok(())
    }
}
#[pyfunction]
#[pyo3(signature =(input_data,algorithm = "mds"))]
pub fn timecurves_from_data(input_data: &PyInputData, algorithm: &str) -> PyResult<PyTimecurves> {
    let mut _algorithm = match &algorithm.to_lowercase()[..] {
        "mds" => ClassicalMDS::new(),
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unknown algorithm '{}'.",
                algorithm
            )))
        }
    };
    let a = TimecurveSet::new(&input_data.inputdata, _algorithm);
    match a {
        Ok(v) => Ok(PyTimecurves { timecurves: v }),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
 */
