use pyo3::prelude::*;
use timecurves_rs::{
    projection::ProjectionAlgorithm,
    timecurve::{Position, Timecurve, TimecurvePoint, TimecurveSet},
};

use crate::{input::PyInputData, projection::PyClassicalMDS};

#[pyclass(name = "Position")]
pub struct PyPosition {
    pub inner: Position,
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

#[pyclass(name = "TimecurvePoint")]
pub struct PyTimecurvePoint {
    inner: TimecurvePoint,
}

impl From<TimecurvePoint> for PyTimecurvePoint {
    fn from(p: TimecurvePoint) -> Self {
        PyTimecurvePoint { inner: p }
    }
}

#[pymethods]
impl PyTimecurvePoint {
    pub fn get_label(&self) -> &str {
        &self.inner.get_label()
    }

    pub fn get_t(&self) -> i64 {
        self.inner.get_t()
    }

    pub fn get_pos(&self) -> PyPosition {
        self.inner.get_pos().clone().into()
    }

    pub fn get_c_prev(&self) -> Option<PyPosition> {
        self.inner.get_c_prev().map(|p| p.clone().into())
    }

    pub fn get_c_next(&self) -> Option<PyPosition> {
        self.inner.get_c_next().map(|p| p.clone().into())
    }

    pub fn get_pos_x(&self) -> f64 {
        self.inner.get_pos().get_x()
    }

    pub fn get_pos_y(&self) -> f64 {
        self.inner.get_pos().get_y()
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.inner.get_color()
    }
}

#[pyclass(name = "Timecurve")]
pub struct PyTimecurve {
    inner: Timecurve,
}

impl From<Timecurve> for PyTimecurve {
    fn from(tc: Timecurve) -> Self {
        PyTimecurve { inner: tc }
    }
}

#[pymethods]
impl PyTimecurve {
    pub fn get_name(&self) -> &str {
        &self.inner.get_name()
    }

    pub fn get_points(&self) -> Vec<PyTimecurvePoint> {
        self.inner
            .get_points()
            .iter()
            .map(|p| p.clone().into())
            .collect()
    }
}

#[pyclass(name = "TimecurveSet")]
pub struct PyTimecurves {
    pub inner: TimecurveSet,
}

impl From<TimecurveSet> for PyTimecurves {
    fn from(tc: TimecurveSet) -> Self {
        PyTimecurves { inner: tc }
    }
}
/*
#[pymethods]
impl PyTimecurves {
    #[new]
    pub fn new(input_data: &PyInputData, proj_algo: Bound<'_, PyAny>) -> PyResult<Self> {
        let input_data = &input_data.inner;
    }
}
 */
