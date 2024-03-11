
use crate::{
    error::{TimeCurveErrorKind, TimecurveError},
    input::InputData,
    projection::ProjectionAlgorithm,
};
use std::f64::consts::PI;


pub struct TimecurvePoint {
    pub label: String,
    // t <-> timelabel sous forme de nombre pour le format de fichier input par défaut
    pub t: Option<u64>,
    pub pos: (f64, f64),
    // le point de contrôle en commun avec le point precedent
    pub c_prev: Option<(f64, f64)>,
    // le point de contrôle en commun avec le point suivant
    pub c_next: Option<(f64, f64)>,
}

impl TimecurvePoint {

    pub fn time_label_to_unix_time(&self) -> u64 {
        //"2023-08-03T19:28:26Z" to 1691083706
        let datetime = chrono::DateTime::parse_from_rfc3339(&self.label)
            .map_err(|_| TimecurveError::new(TimeCurveErrorKind::InvalidTimeLabel, "Invalid time label"));
        datetime.unwrap().timestamp() as u64
    }
    
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
    pub fn from_input_data(
        input_data: &InputData,
        proj_algo: impl ProjectionAlgorithm,
    ) -> Vec<Self> {
        let mut timecurves: Vec<Timecurve> = Vec::new();

        let projected_points = proj_algo.project(&input_data.distancematrix);
        let mut i = 0;
        for dataset in &input_data.data {
            let mut timecurve = Timecurve::new_empty(&dataset.name);
            for timelabel in &dataset.timelabels {
                timecurve.points.push(TimecurvePoint {
                    label: String::from(timelabel),
                    t: None,
                    pos: projected_points[i].clone(),
                    // TODO : calcul des control points avec méthode variable comme l'algo
                    // de projection
                    c_prev: None,
                    c_next: None,
                });

                i = i + 1;
            }

            timecurve.compute_control_points(0.3);

            timecurve.orient();
            timecurves.push(timecurve);
        }
        timecurves
    }

    pub fn compute_control_points(&mut self, sigma: f64) {
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
                    TimeCurveErrorKind::EvaluatedOutsideRange,
                    &format!("c_next is None for point {}", u.floor()),
                ))
            }
        };
        let p2 = match &self.points[p3_index].c_prev {
            Some(p) => p,
            None => {
                return Err(TimecurveError::new(
                    TimeCurveErrorKind::EvaluatedOutsideRange,
                    &format!("c_prev is None for point {}", u.floor() as usize + 1),
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

    pub fn orient(&mut self) -> () {
        //align the first and the last point
        let angle =
            self.get_rotation_angle(self.points[0].pos, self.points[self.points.len() - 1].pos);
        for i in 0..self.points.len() {
            self.points[i].pos = self.rotate_point(angle, self.points[i].pos);
            self.points[i].c_prev = match self.points[i].c_prev {
                Some(p) => Some(self.rotate_point(angle, p)),
                None => None,
            };
            self.points[i].c_next = match self.points[i].c_next {
                Some(p) => Some(self.rotate_point(angle, p)),
                None => None,
            };
        }
    }
    fn get_rotation_angle(&self, p0: (f64, f64), p1: (f64, f64)) -> f64 {
        let angle = -((p1.1 - p0.1).abs() / (p1.0 - p0.0)).atan();
        println!("Angle : {}", angle);
        return angle + PI;
    }
    fn rotate_point(&self, angle: f64, p: (f64, f64)) -> (f64, f64) {
        let x = p.0;
        let y = p.1;

        let x_rot = x * angle.cos() - y * angle.sin();
        let y_rot = x * angle.sin() + y * angle.cos();

        (x_rot, y_rot)
    }
}

fn lerp(a: &(f64, f64), b: &(f64, f64), t: f64) -> (f64, f64) {
    ((1.0 - t) * a.0 + t * b.0, (1.0 - t) * a.1 + t * b.1)
}

// TODO : implémenter une méthode de Timecurve pour normaliser les points
// par exemple centrer les points sur (0,0) et faire en sorte qu'ils aillent de -1 à 1 ou je sais pas
// parce que pour l'instant la position des points dépend de la matrice de distance,
// c.a.d que si les distance sont genre 0.3, 0.6, etc la courbe finale sera toute petite
// alors que si les distances sont 300, 567, etc la courbe sera énorme

// TODO : implémenter une méthode de timecurve qui tourne les points autour de l'origine en fonction du temps t
// pour que le sens de lecture s'aligne sur gauche -> droite
