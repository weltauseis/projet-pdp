/// Represents an error that can occur while working with time curves.
#[derive(Debug)]
pub struct TimecurveError {
    /// The kind of error that occurred.
    pub kind: TimecurveErrorKind,
    /// Additional information about the error. Can be `None`.
    pub info: Option<String>,
}

/// Represents the different kinds of errors that can occur while working with time curves.
#[derive(Debug)]
pub enum TimecurveErrorKind {
    /// The distance matrix is malformed. Eg. it is empty, asymmetric, not square, etc.
    MalformedDistanceMatrix,
    /// Tried to evaluate a time curve outside its range.
    EvaluatedOutsideRange,
    /// The time label is not in any valid format.
    InvalidTimeLabel,
    /// An error occured in a python function passed to the library.
    PythonError,
}

impl std::fmt::Display for TimecurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(info) = &self.info {
            write!(
                f,
                "{} ({})",
                match self.kind {
                    TimecurveErrorKind::MalformedDistanceMatrix => "Distance matrix is malformed !",
                    TimecurveErrorKind::EvaluatedOutsideRange =>
                        "Tried to evaluate timecurve outside its range !",
                    TimecurveErrorKind::InvalidTimeLabel => "Timelabel is invalid !",
                    TimecurveErrorKind:: PythonError => "/// An error occured in a python function passed to the library !",
                },
                info
            )?;
        }
        Ok(())
    }
}

impl TimecurveError {
    /// Creates a new `TimecurveError` instance with the specified error kind and optional additional information.
    ///
    /// ### Arguments
    ///
    /// * `kind` - The kind of error.
    /// * `info` - Optional additional information about the error.
    ///
    /// ### Returns
    ///
    /// A new `TimecurveError` instance.
    ///
    /// ### Note
    ///
    /// The `info` parameter is optional and can be `None`. It should not be used to repeat the error kind.
    pub fn new(kind: TimecurveErrorKind, info: Option<&str>) -> Self {
        Self {
            kind,
            info: info.map(String::from),
        }
    }
}
