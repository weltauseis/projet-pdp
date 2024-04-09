use crate::{
    error::{TimecurveError, TimecurveErrorKind},
    input::{Dataset, InputData},
    projection::ProjectionAlgorithm,
};

#[derive(Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    pub fn lerp(&self, other: &Position, t: f64) -> Position {
        Position {
            x: (1.0 - t) * self.x + t * other.x,
            y: (1.0 - t) * self.y + t * other.y,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }
}

/// Represents a point on a timecurve.
pub struct TimecurvePoint {
    /// The string label associated with the point.
    label: String,
    /// The unix time value of the point. Is equivalent to the label, but in numerical form.
    t: i64,
    /// The (x, y) position of the point in 2D space.
    pos: Position,
    /// The control point in the direction of the the previous point on the curve.
    c_prev: Option<Position>,
    /// The control point in the direction of the next point on the curve.
    c_next: Option<Position>,
}

impl TimecurvePoint {
    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_t(&self) -> i64 {
        self.t
    }

    pub fn get_pos(&self) -> &Position {
        &self.pos
    }

    pub fn get_c_prev(&self) -> Option<&Position> {
        self.c_prev.as_ref()
    }

    pub fn get_c_next(&self) -> Option<&Position> {
        self.c_next.as_ref()
    }
}

/// Represents a single timecurve.
pub struct Timecurve {
    /// The name of the timecurve.
    name: String,
    /// A list holding the points that make up the timecurve.
    /// If the curve is created from a projection algorithm, the points are sorted chronologically.
    points: Vec<TimecurvePoint>,
}

impl Timecurve {
    /// Creates a new empty timecurve with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the timecurve.
    ///
    /// # Returns
    ///
    /// A new `Timecurve` instance.
    fn new_empty(name: &str) -> Self {
        Timecurve {
            points: Vec::new(),
            name: name.to_owned(),
        }
    }

    /// Creates a new timecurve from a dataset and a list of points.
    ///
    /// # Arguments
    ///
    /// * `dataset` - The dataset from which the timecurve is created.
    /// * `projected_points` - A slice of (x, y) points that make up the timecurve.
    ///   The length should be equal to the number of timelabels in the dataset.
    ///
    /// # Returns
    ///
    /// A new `Timecurve` instance.
    fn new(dataset: &Dataset, projected_points: &[Position]) -> Result<Self, TimecurveError> {
        let mut timecurve = Timecurve::new_empty(&dataset.name);
        for (i, timelabel) in dataset.timelabels.iter().enumerate() {
            timecurve.points.push(TimecurvePoint {
                label: timelabel.to_owned(),
                t: label_to_time(&timelabel)?,
                pos: projected_points[i],
                c_prev: None,
                c_next: None,
            });
        }

        return Ok(timecurve);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_points(&self) -> &[TimecurvePoint] {
        &self.points
    }

    fn compute_control_points(&mut self, sigma: f64) {
        for i in 1..self.points.len() - 1 {
            let current = &self.points[i];
            let previous = &self.points[i - 1];
            let next = &self.points[i + 1];

            // These control points are positioned so that the line joining them is parallel to (pi−1, pi+1).

            let mut line = (previous.pos.x - next.pos.x, previous.pos.y - next.pos.y);
            let norm = (line.0.powi(2) + line.1.powi(2)).sqrt();
            line = (line.0 / norm, line.1 / norm);

            // The distance of ci,1 (resp. ci+1,0) to pi is set to the distance
            // between pi and pi−1 (resp. pi+1) multiplied by a a smoothing parameter σ .

            // distance between the previous and the current point
            let dist_p_c = ((previous.pos.x - current.pos.x).powi(2)
                + (previous.pos.y - current.pos.y).powi(2))
            .sqrt();

            let control_1 = Position::new(
                current.pos.x + line.0 * dist_p_c * sigma,
                current.pos.y + line.1 * dist_p_c * sigma,
            );

            // distance between the current and the next point
            let dist_c_n = ((current.pos.x - next.pos.x).powi(2)
                + (current.pos.y - next.pos.y).powi(2))
            .sqrt();

            // second control point
            let control_2 = Position::new(
                current.pos.x - line.0 * dist_c_n * sigma,
                current.pos.y - line.1 * dist_c_n * sigma,
            );

            self.points[i].c_prev = Some(control_1);

            self.points[i].c_next = Some(control_2);
        }

        // special case for the first and last points
        // the cpoint for the first/last point is calculated based on the line between
        // the first/last point and the next/previous control point
        let len = self.points.len();

        let p0 = &self.points[0].pos;
        let c0 = &self.points[1].get_c_prev().unwrap();
        let mut line0 = (p0.x - c0.x, p0.y - c0.y);
        let norm = (line0.0.powi(2) + line0.1.powi(2)).sqrt();
        line0 = (line0.0 / norm, line0.1 / norm);

        self.points[0].c_next = Some(Position::new(
            p0.x - line0.0 * sigma,
            p0.y - line0.1 * sigma,
        ));

        let p1: &Position = &self.points[len - 1].pos;
        let c1 = &self.points[len - 2].get_c_next().unwrap();
        let mut line1 = (p1.x - c1.x, p1.y - c1.y);
        let norm = (line1.0.powi(2) + line1.1.powi(2)).sqrt();
        line1 = (line1.0 / norm, line1.1 / norm);

        self.points[len - 1].c_prev = Some(Position::new(
            p1.x - line1.0 * sigma,
            p1.y - line1.1 * sigma,
        ));
    }

    pub fn evaluate(&self, u: f64) -> Result<Position, TimecurveError> {
        let t = u.fract();

        let p0_index = u.floor() as usize;
        let p3_index = (u.floor() as usize + 1).clamp(0, self.points.len() - 1);

        println!(
            "\nEvaluating at u = {:.3}(t = {:.3}), between P0 = {} & P3 = {}",
            u, t, p0_index, p3_index
        );

        // if evaluating exactly on a point, return the point
        if t == 0.0 {
            return Ok(self.points[p0_index].get_pos().clone());
        }

        let p0 = &self.points[p0_index].pos;
        let p1 = match &self.points[p0_index].c_next {
            Some(p) => p,
            None => {
                return Err(TimecurveError::new(
                    TimecurveErrorKind::EvaluatedOutsideRange,
                    Some(&format!("c_next is None for point {}", u.floor())),
                ))
            }
        };
        let p2 = match &self.points[p3_index].c_prev {
            Some(p) => p,
            None => {
                return Err(TimecurveError::new(
                    TimecurveErrorKind::EvaluatedOutsideRange,
                    Some(&format!(
                        "c_prev is None for point {}",
                        u.floor() as usize + 1
                    )),
                ))
            }
        };

        let p3 = &self.points[p3_index].pos;

        // Algorithme de Casteljau
        let a = p0.lerp(p1, t);
        let b = p1.lerp(p2, t);
        let c = p2.lerp(p3, t);

        let d = &a.lerp(&b, t);
        let e = &b.lerp(&c, t);

        return Ok(d.lerp(&e, t));
    }

    fn rotate_points_around_origin(&mut self, angle: f64) {
        for p in &mut self.points {
            p.pos = rotate_point_around_origin(angle, p.pos);
            if let Some(c) = p.c_prev {
                p.c_prev = Some(rotate_point_around_origin(angle, c));
            }
            if let Some(c) = p.c_next {
                p.c_next = Some(rotate_point_around_origin(angle, c));
            }
        }
    }

    fn normalise_points(&mut self, y_min: f64, x_min: f64, range: f64) {
        // substract xmin or ymind to bring points into positive range ([0; +inf], [0; +inf])
        // then divide by range to bring them into ([0; 1], [0; 1])
        for p in &mut self.points {
            p.pos.x = (p.pos.x - x_min) / range;
            p.pos.y = (p.pos.y - y_min) / range;

            if let Some(c) = p.c_prev {
                let new_x = (c.x - x_min) / range;
                let new_y = (c.y - y_min) / range;
                p.c_prev = Some(Position::new(new_x, new_y));
            }

            if let Some(c) = p.c_next {
                let new_x = (c.x - x_min) / range;
                let new_y = (c.y - y_min) / range;
                p.c_next = Some(Position::new(new_x, new_y));
            }
        }
    }
}

/// Represents a set of one or more timecurves sharing the same 2D space.
pub struct TimecurveSet {
    /// A vector containing all the timecurves in the set.
    curves: Vec<Timecurve>,
}

impl TimecurveSet {
    pub fn new(
        input_data: &InputData,
        proj_algo: impl ProjectionAlgorithm,
    ) -> Result<Self, TimecurveError> {
        let mut timecurves = TimecurveSet { curves: Vec::new() };
        let projected_points = proj_algo.project(&input_data.distancematrix)?;

        let mut index = 0; // index to keep track of where we are in the projected points
        for dataset in &input_data.data {
            let mut timecurve = Timecurve::new(
                &dataset,
                &projected_points[index..index + dataset.timelabels.len()],
            )?;

            timecurve.points.sort_by_key(|p| p.t);
            timecurve.compute_control_points(0.3);
            timecurves.curves.push(timecurve);

            index += dataset.timelabels.len();
        }
        //Must be in this order if we want the curve to be around the origin
        timecurves.align();
        timecurves.normalise();
        return Ok(timecurves);
    }

    pub fn get_curves(&self) -> &[Timecurve] {
        &self.curves
    }

    fn align(&mut self) {
        // for multiple datasets, we align based on the first curve
        // like in the examples in the webpage
        let first_curve = match self.curves.get(0) {
            Some(v) => v,
            None => return,
        };

        // if there are no points to align
        if first_curve.points.len() < 2 {
            return;
        }

        let p0 = first_curve.points.first().unwrap(); // okay to unwrap here because we checked for length
        let p1 = first_curve.points.last().unwrap();

        // find the angle needed for first and last point to be aligned horizontally
        let angle = -(p1.pos.y - p0.pos.y).atan2(p1.pos.x - p0.pos.x);

        // rotate all points around the origin by that angle
        for curve in &mut self.curves {
            curve.rotate_points_around_origin(angle);
        }
    }

    fn normalise(&mut self) {
        // normalise in range [0, 1]
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for curve in &self.curves {
            x_min = curve.points.iter().fold(x_min, |acc, p| acc.min(p.pos.x));
            x_max = curve.points.iter().fold(x_max, |acc, p| acc.max(p.pos.x));
            y_min = curve.points.iter().fold(y_min, |acc, p| acc.min(p.pos.y));
            y_max = curve.points.iter().fold(y_max, |acc, p| acc.max(p.pos.y));
        }

        // we want to scale all points by the same factor
        // on the x and y axis in order to keep the aspect ratio
        // and not distort distances between points
        let max = x_max.max(y_max);
        let min = x_min.min(y_min);

        let range = max - min;

        for curve in &mut self.curves {
            curve.normalise_points(y_min, x_min, range);
        }
    }
}

/// Utility function that calculates the new position of a point after a rotation around the origin.
///
/// # Arguments
///
/// * `angle` - The angle of rotation, in radians.
/// * `p` - The point before transformation.
///
/// # Returns
///
/// The transformed point.
fn rotate_point_around_origin(angle: f64, p: Position) -> Position {
    let x = p.x;
    let y = p.y;

    let x_prime = x * angle.cos() - y * angle.sin();
    let y_prime = x * angle.sin() + y * angle.cos();

    Position::new(x_prime, y_prime)
}

// ATTENTION : ce code utilise NaiveDateTime donc il ne faut pas mélanger les timezone à l'interieur des différents datasets
// c.a.d UNE SEULE TIMEZONE PAR FICHIER
fn label_to_time(label: &str) -> Result<i64, TimecurveError> {
    let mut date;

    // label is a time string
    date = chrono::NaiveDateTime::parse_from_str(label, "%Y-%m-%dT%H:%M:%SZ");
    if let Ok(t) = date {
        return Ok(t.and_utc().timestamp());
    }

    date = chrono::NaiveDateTime::parse_from_str(label, "%Y-%m-%d %H:%M:%S.%f");
    if let Ok(t) = date {
        return Ok(t.and_utc().timestamp());
    }

    // label is a raw number
    let time = label.parse::<i64>();
    if let Ok(t) = time {
        return Ok(t);
    }

    // no parsing method worked
    return Err(TimecurveError::new(
        TimecurveErrorKind::InvalidTimeLabel,
        Some(&format!("Label : \"{}\"", label)),
    ));
}
