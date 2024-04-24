use crate::{
    error::{TimecurveError, TimecurveErrorKind},
    input::{Dataset, InputData},
    projection::ProjectionAlgorithm,
};

use palette::{Darken, Hsv, IntoColor, Mix, Srgb};

#[derive(Clone, Copy)]
/// Represents a position in 2D space.
pub struct Position {
    // The x coordinate of the position.
    x: f64,
    // The y coordinate of the position.
    y: f64,
}

impl Position {
    /// Creates a new `Position` with the given x and y coordinates.
    ///
    /// ### Arguments
    ///
    /// * `x` - The x coordinate.
    /// * `y` - The y coordinate.
    ///
    /// ### Returns
    ///
    /// A new `Position` with the given coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    /// Performs linear interpolation between two positions.
    ///
    /// ### Arguments
    ///
    /// * `other` - The other position to interpolate with.
    /// * `t` - The interpolation parameter, ranging from 0.0 to 1.0.
    ///
    /// ### Returns
    ///
    /// The interpolated position.
    fn lerp(&self, other: &Position, t: f64) -> Position {
        Position {
            x: (1.0 - t) * self.x + t * other.x,
            y: (1.0 - t) * self.y + t * other.y,
        }
    }

    /// Returns the x coordinate of the position.
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Returns the y coordinate of the position.
    pub fn get_y(&self) -> f64 {
        self.y
    }
}

/// Represents a point on a timecurve.
#[derive(Clone)]
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
    /// The color of the point, for visualization purposes
    color: (u8, u8, u8),
}

impl TimecurvePoint {
    /// Returns the label of the timecurve point.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Returns the timestamp of the timecurve point.
    pub fn get_t(&self) -> i64 {
        self.t
    }

    /// Returns a reference to the position of the timecurve point.
    pub fn get_pos(&self) -> &Position {
        &self.pos
    }

    /// Returns an optional reference to the previous control point of the timecurve point.
    pub fn get_c_prev(&self) -> Option<&Position> {
        self.c_prev.as_ref()
    }

    /// Returns an optional reference to the next control point of the timecurve point.
    pub fn get_c_next(&self) -> Option<&Position> {
        self.c_next.as_ref()
    }

    /// Returns the x-coordinate of the position of the timecurve point.
    pub fn get_pos_x(&self) -> f64 {
        self.pos.get_x()
    }

    /// Returns the y-coordinate of the position of the timecurve point.
    pub fn get_pos_y(&self) -> f64 {
        self.pos.get_y()
    }

    /// Returns the color of the point, as a RGB tuple.
    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }
}

/// Represents a single timecurve.
#[derive(Clone)]
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
    /// ### Arguments
    ///
    /// * `name` - The name of the timecurve.
    ///
    /// ### Returns
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
    /// ### Arguments
    ///
    /// * `dataset` - The dataset from which the timecurve is created.
    /// * `projected_points` - A slice of (x, y) points that make up the timecurve.
    ///   The length should be equal to the number of timelabels in the dataset.
    ///
    /// ### Returns
    ///
    /// A new `Timecurve` instance.
    fn new(dataset: &Dataset, projected_points: &[Position]) -> Result<Self, TimecurveError> {
        let mut timecurve = Timecurve::new_empty(dataset.get_name());
        for (i, timelabel) in dataset.get_timelabels().iter().enumerate() {
            timecurve.points.push(TimecurvePoint {
                label: timelabel.to_owned(),
                t: label_to_time(&timelabel)?,
                pos: projected_points[i],
                c_prev: None,
                c_next: None,
                color: (0, 0, 0),
            });
        }

        return Ok(timecurve);
    }

    /// Returns the name of the timecurve.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a slice over the points of the timecurve.
    pub fn get_points(&self) -> &[TimecurvePoint] {
        &self.points
    }

    /// Computes the control points for the timecurve.
    ///
    /// ##### Arguments
    ///
    /// * `sigma` - The smoothing parameter for the control points. For more information, see the paper.
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

    /// Evaluates the timecurve at a given point along the curve.
    /// This could be useful for custom exporters that don't natively support bezier curves.
    /// Or to draw the curve using mesh lines.
    ///
    /// ### Arguments
    ///
    /// * `u` - The parametric value at which to evaluate the timecurve. Should be in the range \[0, n\].
    ///
    /// ### Returns
    ///
    /// The position of the timecurve at the given parametric value.
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

    /// Rotates all points of the timecurve around the origin by a given angle.
    /// This is useful for aligning the timecurves.
    ///
    /// ### Arguments
    ///
    /// * `angle` - The angle of rotation, in radians.
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

    /// Normalises the points of the timecurve to the range \[0, 1\].
    /// Arguments are needed so that we can normalise all timecurves contained in a set the same way.
    ///
    /// ### Arguments
    ///
    /// * `min` - The minimum value of the x and y coordinates of all points in the timecurve set.
    /// * `range` - The range of the x and y coordinates of all points in the timecurve set.
    fn normalise_points(&mut self, min: Position, range: f64) {
        // substract xmin or ymind to bring points into positive range ([0; +inf], [0; +inf])
        // then divide by range to bring them into ([0; 1], [0; 1])
        for p in &mut self.points {
            p.pos.x = (p.pos.x - min.x) / range;
            p.pos.y = (p.pos.y - min.y) / range;

            if let Some(c) = p.c_prev {
                let new_x = (c.x - min.x) / range;
                let new_y = (c.y - min.y) / range;
                p.c_prev = Some(Position::new(new_x, new_y));
            }

            if let Some(c) = p.c_next {
                let new_x = (c.x - min.x) / range;
                let new_y = (c.y - min.y) / range;
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
    /// Creates a new `TimecurveSet` from an `InputData` instance and a projection algorithm.
    /// The timecurves are aligned and normalised in the process, and the points are sorted chronologically.
    ///
    /// ### Arguments
    /// * `input_data` - The input data containing the datasets and distance matrix.
    /// * `proj_algo` - The projection algorithm to use to project the points.
    ///
    /// ### Returns
    /// A new `TimecurveSet` instance.
    pub fn new(
        input_data: &InputData,
        proj_algo: impl ProjectionAlgorithm,
    ) -> Result<Self, TimecurveError> {
        let mut timecurves = TimecurveSet { curves: Vec::new() };
        let projected_points = proj_algo.project(&input_data.get_distance_matrix())?;

        let mut index = 0; // index to keep track of where we are in the projected points
        for dataset in input_data.get_datasets() {
            let mut timecurve = Timecurve::new(
                &dataset,
                &projected_points[index..index + dataset.get_timelabels().len()],
            )?;

            timecurve.points.sort_by_key(|p| p.t);
            timecurve.compute_control_points(0.3);
            timecurves.curves.push(timecurve);

            index += dataset.get_timelabels().len();
        }
        //Must be in this order if we want the curve to be around the origin
        timecurves.align();
        timecurves.normalise();
        timecurves.update_colors();
        return Ok(timecurves);
    }

    /// Returns a slice over the timecurves in the set.
    pub fn get_curves(&self) -> &[Timecurve] {
        &self.curves
    }

    /// Aligns the timecurves in the set so that the first and last points of the first curve are aligned horizontally.
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

    /// Normalises all timecurves in the set so that their points are in the range \[0, 1\].
    fn normalise(&mut self) {
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
        assert!(
            !range.is_infinite(),
            "Overflow in normalisation, range is infinite."
        );
        for curve in &mut self.curves {
            curve.normalise_points(Position::new(x_min, y_min), range);
        }
    }

    /// Updates the colors of the points in the timecurves.
    fn update_colors(&mut self) {
        for (i, curve) in self.curves.iter_mut().enumerate() {
            let oldest = curve.points.first().unwrap().t as f32;
            let newest = curve.points.last().unwrap().t as f32;

            for point in curve.points.iter_mut() {
                point.color = curve_color_lerp(i, (point.t as f32 - oldest) / (newest - oldest))
            }
        }
    }
}

/// Utility function that calculates the new position of a point after a rotation around the origin.
///
/// ### Arguments
///
/// * `angle` - The angle of rotation, in radians.
/// * `p` - The point before transformation.
///
/// ### Returns
///
/// The transformed point.
fn rotate_point_around_origin(angle: f64, p: Position) -> Position {
    let x = p.x;
    let y = p.y;

    let x_prime = x * angle.cos() - y * angle.sin();
    let y_prime = x * angle.sin() + y * angle.cos();

    Position::new(x_prime, y_prime)
}

/// Utility function that converts a label to a unix timestamp.
///
/// ### Arguments
///
/// * `label` - The label to convert to a timestamp. Should be an ISO 8601 date or a number.
///            If it is a number, it is assumed to be a unix timestamp.
///
/// ### Returns
/// The unix timestamp corresponding to the label.
///
/// ### Note
/// Please note that the label is assumed to be in UTC time. If it is not, the timestamp will be incorrect.
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

/// Utility function that linearly interpolates between two colors.
///
/// ### Arguments
///
/// * `curve_id` - The id of the curve. Used to determine the color.
/// * `u` - The interpolation factor. Should be between 0.0 and 1.0.
///
/// ### Returns
///
/// A RGB tuple of three u8 values representing the interpolated color.
pub fn curve_color_lerp(curve_id: usize, u: f32) -> (u8, u8, u8) {
    static COLORS: [(u8, u8, u8); 3] = [
        (255, 105, 22), // orange
        (34, 130, 251), // blue
        (149, 221, 60), // green
    ];

    let color_id = curve_id % COLORS.len();

    let r = COLORS[color_id].0;
    let g = COLORS[color_id].1;
    let b = COLORS[color_id].2;

    let start_color: Hsv =
        Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0).into_color();
    let end_color = Hsv::from(start_color).darken(0.7);

    let color = start_color.mix(end_color, u);

    let srgb: Srgb = color.into_color();

    return (
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    );
}

#[cfg(test)]
mod tests {
    use std::f64::{MAX, MIN};

    use super::*;

    #[test]
    fn random_dommain_test_normalise_points() {
        let mut timecurve = Timecurve::new_empty("test");
        for i in 0..100 {
            let x = rand::random::<f64>() * 1000.0;
            let y = rand::random::<f64>() * 1000.0;
            timecurve.points.push(TimecurvePoint {
                label: i.to_string(),
                t: i,
                pos: Position::new(x, y),
                c_prev: None,
                c_next: None,
                color: (0, 0, 0),
            });
        }
        let mut set = TimecurveSet {
            curves: vec![timecurve],
        };
        set.normalise();
        for curve in set.curves {
            for p in curve.points {
                assert!(p.pos.x <= 1.0 && p.pos.x >= -1.0);
                assert!(p.pos.y <= 1.0 && p.pos.y >= -1.0);
            }
        }
    }

    #[test]
    fn limit_dommain_test_normalise_points() {
        //test with points at the limit of the domain
        let mut timecurve = Timecurve::new_empty("test");
        let x = [
            (MAX / 2.0, MAX / 2.0),
            (MAX / 2.0, 0.0),
            (0.0, MAX / 2.0),
            (0.0, 0.0),
            (MIN / 2.0, MIN / 2.0),
            (MIN / 2.0, 0.0),
            (0.0, MIN / 2.0),
        ];
        for i in 0..x.len() {
            timecurve.points.push(TimecurvePoint {
                label: i.to_string(),
                t: i as i64,
                pos: Position::new(x[i].0, x[i].1),
                c_prev: None,
                c_next: None,
                color: (0, 0, 0),
            });
        }
        let mut set = TimecurveSet {
            curves: vec![timecurve],
        };
        set.normalise();
        for curve in set.curves {
            for p in curve.points {
                assert!(p.pos.x <= 1.0 && p.pos.x >= 0.0);
                assert!(p.pos.y <= 1.0 && p.pos.y >= 0.0);
            }
        }
    }

    #[test]
    fn test_label_to_time() {
        let label = "2021-01-01T00:00:00Z";
        let time = label_to_time(label).unwrap();
        assert_eq!(time, 1609459200);

        let label = "2021-01-01 00:00:00.000";
        let time = label_to_time(label).unwrap();
        assert_eq!(time, 1609459200);

        let label = "1609459200";
        let time = label_to_time(label).unwrap();
        assert_eq!(time, 1609459200);

        let label = "not a valid label";
        let time = label_to_time(label);
        assert!(time.is_err());
    }

    #[test]
    fn test_rotate_point_around_origin() {
        const EPSILON: f64 = 1e-6; // tolerance for floating point comparisons around zero
        let p = Position::new(1.0, 0.0);
        let angle = std::f64::consts::PI / 2.0;
        let new_p = rotate_point_around_origin(angle, p);
        assert!(new_p.get_x().abs() < EPSILON);
        assert_eq!(new_p.get_y(), 1.0);

        let p = Position::new(1.0, 0.0);
        let angle = std::f64::consts::PI * 1.5;
        let new_p = rotate_point_around_origin(angle, p);
        assert!(new_p.get_x().abs() < EPSILON);
        assert_eq!(new_p.get_y(), -1.0);
    }

    #[test]
    fn test_timecurve_compute_control_points() {
        //Control points do exist
        let mut timecurve = Timecurve::new_empty("test");
        let x = [(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        for i in 0..x.len() {
            timecurve.points.push(TimecurvePoint {
                label: i.to_string(),
                t: i as i64,
                pos: Position::new(x[i].0, x[i].1),
                c_prev: None,
                c_next: None,
                color: (0, 0, 0),
            });
        }
        timecurve.compute_control_points(0.3);
        for p in &timecurve.points {
            //first and last point have only one control point
            if p.t == 0 {
                assert!(p.c_next.is_some());
                assert!(p.c_prev.is_none());
            } else if p.t == timecurve.points.len() as i64 - 1 {
                assert!(p.c_prev.is_some());
                assert!(p.c_next.is_none());
            } else {
                assert!(p.c_prev.is_some());
                assert!(p.c_next.is_some());
            }
        }
    }

    #[test]
    fn test_timecurve_align() {
        //test with point are align on the y axis
        let mut timecurve = Timecurve::new_empty("test");
        let x = [(0.0, 0.0), (1.0, 1.0), (2.0, 3.0)];
        for i in 0..x.len() {
            timecurve.points.push(TimecurvePoint {
                label: i.to_string(),
                t: i as i64,
                pos: Position::new(x[i].0, x[i].1),
                c_prev: None,
                c_next: None,
                color: (0, 0, 0),
            });
        }
        let mut set = TimecurveSet {
            curves: vec![timecurve],
        };
        set.align();
        for curve in set.curves {
            let p0 = curve.points.first().unwrap();
            let p1 = curve.points.last().unwrap();
            assert_eq!(p0.pos.y, p1.pos.y);
        }
    }
    //new timecurve
    #[test]
    fn test_timecurve_new() {
        let dataset = Dataset::new(
            "test",
            vec!["0".to_string(), "1".to_string(), "2".to_string()],
        );
        let projected_points = vec![
            Position::new(0.0, 0.0),
            Position::new(1.0, 1.0),
            Position::new(2.0, 3.0),
        ];
        let timecurve = Timecurve::new(&dataset, &projected_points).unwrap();
        assert_eq!(timecurve.get_name(), "test");
        let points = timecurve.get_points();
        assert_eq!(points.len(), 3);
        assert_eq!(points[0].get_label(), "0");
        assert_eq!(points[0].get_t(), 0);
        assert_eq!(points[0].get_pos_x(), 0.0);
        assert_eq!(points[0].get_pos_y(), 0.0);
        assert!(points[0].get_c_prev().is_none());
        assert!(points[0].get_c_next().is_none());
        assert_eq!(points[0].get_color(), (0, 0, 0));
        assert_eq!(points[0].get_pos().get_x(), 0.0);
        assert_eq!(points[0].get_pos().get_y(), 0.0);
    }
}
