use crate::{
    error::{TimecurveError, TimecurveErrorKind},
    input::InputData,
    projection::ProjectionAlgorithm,
};

pub struct TimecurvePoint {
    pub label: String,
    // t <-> timelabel sous forme de nombre pour le format de fichier input par défaut
    pub t: i64,
    pub pos: (f64, f64),
    // le point de contrôle en commun avec le point precedent
    pub c_prev: Option<(f64, f64)>,
    // le point de contrôle en commun avec le point suivant
    pub c_next: Option<(f64, f64)>,
}

pub struct Timecurve {
    pub points: Vec<TimecurvePoint>,
    pub name: String,
}

impl Timecurve {
    pub fn new_empty(name: &str) -> Self {
        Timecurve {
            points: Vec::new(),
            name: String::from(name),
        }
    }
}

impl Timecurve {
    // TODO : implémenter la gestion d'erreurs pour cette fonction
    // par exemple valider l'input, il me semble qu'une matrice de distance vide ou non carrée
    // passe le parsing
    fn new(
        dataset: &crate::input::Dataset,
        projected_points: &Vec<(f64, f64)>,
    ) -> Result<Self, TimecurveError> {
        let mut i = 0;
        let mut timecurve = Timecurve::new_empty(&dataset.name);
        for timelabel in &dataset.timelabels {
            timecurve.points.push(TimecurvePoint {
                label: String::from(timelabel),
                t: label_to_time(&timelabel)?,
                pos: projected_points[i].clone(),
                // TODO : calcul des control points avec méthode variable comme l'algo
                // de projection
                c_prev: None,
                c_next: None,
            });

            i = i + 1;
        }
        return Ok(timecurve);
    }

    fn compute_control_points(&mut self, sigma: f64) {
        for i in 1..self.points.len() - 1 {
            let current = &self.points[i];
            let previous = &self.points[i - 1];
            let next = &self.points[i + 1];

            // These control points are positioned so that the line joining them is parallel to (pi−1, pi+1).

            let mut line = (previous.pos.0 - next.pos.0, previous.pos.1 - next.pos.1);
            let norm = (line.0.powi(2) + line.1.powi(2)).sqrt();
            line = (line.0 / norm, line.1 / norm);

            // The distance of ci,1 (resp. ci+1,0) to pi is set to the distance
            // between pi and pi−1 (resp. pi+1) multiplied by a a smoothing parameter σ .

            // distance between the previous and the current point
            let dist_p_c = ((previous.pos.0 - current.pos.0).powi(2)
                + (previous.pos.1 - current.pos.1).powi(2))
            .sqrt();

            // first control point
            let control_1 = (
                current.pos.0 + line.0 * dist_p_c * sigma,
                current.pos.1 + line.1 * dist_p_c * sigma,
            );

            // distance between the current and the next point
            let dist_c_n = ((current.pos.0 - next.pos.0).powi(2)
                + (current.pos.1 - next.pos.1).powi(2))
            .sqrt();

            // second control point
            let control_2 = (
                current.pos.0 - line.0 * dist_c_n * sigma,
                current.pos.1 - line.1 * dist_c_n * sigma,
            );

            self.points[i].c_prev = Some(control_1);

            self.points[i].c_next = Some(control_2);
        }

        // special case for the first and last points
        // the cpoint for the first/last point is calculated based on the line between
        // the first/last point and the next/previous control point
        let len = self.points.len();

        let p0: &(f64, f64) = &self.points[0].pos;
        let c0 = &self.points[1].c_prev.unwrap();
        let mut line0 = (p0.0 - c0.0, p0.1 - c0.1);
        let norm = (line0.0.powi(2) + line0.1.powi(2)).sqrt();
        line0 = (line0.0 / norm, line0.1 / norm);

        self.points[0].c_next = Some((p0.0 - line0.0 * sigma, p0.1 - line0.1 * sigma));

        let p1: &(f64, f64) = &self.points[len - 1].pos;
        let c1 = &self.points[len - 2].c_next.unwrap();
        let mut line1 = (p1.0 - c1.0, p1.1 - c1.1);
        let norm = (line1.0.powi(2) + line1.1.powi(2)).sqrt();
        line1 = (line1.0 / norm, line1.1 / norm);

        self.points[len - 1].c_prev = Some((p1.0 - line1.0 * sigma, p1.1 - line1.1 * sigma));
    }

    pub fn evaluate(&self, u: f64) -> Result<(f64, f64), TimecurveError> {
        let t = u.fract();

        let p0_index = u.floor() as usize;
        let p3_index = (u.floor() as usize + 1).clamp(0, self.points.len() - 1);

        println!(
            "\nEvaluating at u = {:.3}(t = {:.3}), between P0 = {} & P3 = {}",
            u, t, p0_index, p3_index
        );

        // if evaluating exactly on a point, return the point
        if t == 0.0 {
            return Ok(self.points[p0_index].pos);
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
        let a = lerp(p0, p1, t);
        let b = lerp(p1, p2, t);
        let c = lerp(p2, p3, t);

        let d = lerp(&a, &b, t);
        let e = lerp(&b, &c, t);

        return Ok(lerp(&d, &e, t));
    }

    fn rotate_points(&mut self, angle: f64) {
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
            p.pos.0 = (p.pos.0 - x_min) / range;
            p.pos.1 = (p.pos.1 - y_min) / range;

            if let Some(c) = p.c_prev {
                let new_x = (c.0 - x_min) / range;
                let new_y = (c.1 - y_min) / range;
                p.c_prev = Some((new_x, new_y));
            }

            if let Some(c) = p.c_next {
                let new_x = (c.0 - x_min) / range;
                let new_y = (c.1 - y_min) / range;
                p.c_next = Some((new_x, new_y));
            }
        }
    }
}

pub struct TimecurveSet {
    pub curves: Vec<Timecurve>,
}

impl TimecurveSet {
    pub fn new(
        input_data: &InputData,
        proj_algo: impl ProjectionAlgorithm,
    ) -> Result<Self, TimecurveError> {
        let mut timecurves = TimecurveSet { curves: Vec::new() };
        let projected_points = proj_algo.project(&input_data.distancematrix);
        let mut index = 0; // index to keep track of where we are in the projected points
        for dataset in &input_data.data {
            let mut timecurve = Timecurve::new(
                &dataset,
                &projected_points[index..index + dataset.timelabels.len()].to_vec(),
            )?;

            timecurve.points.sort_by_key(|p| p.t);
            timecurve.compute_control_points(0.3);
            timecurves.curves.push(timecurve);

            index += dataset.timelabels.len();
        }
        timecurves.normalise();
        timecurves.orient();
        return Ok(timecurves);
    }

    fn orient(&mut self) {
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
        let angle = -(p1.pos.1 - p0.pos.1).atan2(p1.pos.0 - p0.pos.0);

        // rotate all points around the origin by that angle
        for curve in &mut self.curves {
            curve.rotate_points(angle);
        }
    }

    fn normalise(&mut self) {
        // normalise in range [0, 1]
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for curve in &self.curves {
            x_min = curve.points.iter().fold(x_min, |acc, p| acc.min(p.pos.0));
            x_max = curve.points.iter().fold(x_max, |acc, p| acc.max(p.pos.0));
            y_min = curve.points.iter().fold(y_min, |acc, p| acc.min(p.pos.1));
            y_max = curve.points.iter().fold(y_max, |acc, p| acc.max(p.pos.1));
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

fn lerp(a: &(f64, f64), b: &(f64, f64), t: f64) -> (f64, f64) {
    ((1.0 - t) * a.0 + t * b.0, (1.0 - t) * a.1 + t * b.1)
}

fn rotate_point_around_origin(angle: f64, p: (f64, f64)) -> (f64, f64) {
    let x = p.0;
    let y = p.1;

    let x_prime = x * angle.cos() - y * angle.sin();
    let y_prime = x * angle.sin() + y * angle.cos();

    (x_prime, y_prime)
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

    // label is a raw time number
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

// TODO : implémenter une méthode de Timecurve pour normaliser les points
// par exemple centrer les points sur (0,0) et faire en sorte qu'ils aillent de -1 à 1 ou je sais pas
// parce que pour l'instant la position des points dépend de la matrice de distance,
// c.a.d que si les distance sont genre 0.3, 0.6, etc la courbe finale sera toute petite
// alors que si les distances sont 300, 567, etc la courbe sera énorme

// TODO : implémenter une méthode de timecurve qui tourne les points autour de l'origine en fonction du temps t
// pour que le sens de lecture s'aligne sur gauche -> droite
