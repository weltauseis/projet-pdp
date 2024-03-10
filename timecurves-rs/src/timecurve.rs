use crate::{
    error::{TimeCurveErrorKind, TimecurveError},
    input::InputData,
    projection::ProjectionAlgorithm,
};

pub struct TimecurvePoint {
    pub label: String,
    // t <-> timelabel sous forme de nombre pour le format de fichier input par défaut
    // TODO : parse le timelabel pour le transformer en temps UNIX, pas encore implémenté
    pub t: Option<u64>,
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

            timecurves.push(timecurve);
        }
        timecurves
    }

    pub fn compute_control_points(&mut self, sigma: f64) {
        for i in 0..self.points.len() {
            let current = &self.points[i];
            let previous = &self.points[i.saturating_sub(1)]; // for first point, previous is the first point
            let next = &self.points[(i + 1).clamp(0, self.points.len() - 1)]; // for last point, next is the last point

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

            // first point doesn't have a previous control point
            if i > 0 {
                self.points[i].c_prev = Some(control_1);
            }

            // last point doesn't have a next control point
            if i < self.points.len() - 1 {
                self.points[i].c_next = Some(control_2);
            }
        }
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
