use crate::{input::InputData, projection::ProjectionAlgorithm};

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
    pub dataset: String,
}

impl Timecurve {
    pub fn new_empty(name: &str) -> Self {
        Timecurve {
            points: Vec::new(),
            dataset: String::from(name),
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

            timecurves.push(timecurve);
        }
        timecurves
    }

    pub fn compute_control_points(&mut self, sigma: f64) {
        for i in 1..self.points.len() {
            let previous = &self.points[i - 1];
            let current = &self.points[i];
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
    }
}

// TODO : implémenter une méthode de Timecurve pour normaliser les points
// par exemple centrer les points sur (0,0) et faire en sorte qu'ils aillent de -1 à 1 ou je sais pas
// parce que pour l'instant la position des points dépend de la matrice de distance,
// c.a.d que si les distance sont genre 0.3, 0.6, etc la courbe finale sera toute petite
// alors que si les distances sont 300, 567, etc la courbe sera énorme

// TODO : implémenter une méthode de timecurve qui tourne les points autour de l'origine en fonction du temps t
// pour que le sens de lecture s'aligne sur gauche -> droite
