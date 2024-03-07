use crate::{input::InputData, projection::ProjectionAlgorithm};

// TODO : implémenter un wrapper autour de [f64;2] pour pouvoir faire
// p.x() au lieu de p[0]
pub struct ControlPoints {
    pub a: [f64; 2],
    pub b: [f64; 2],
}

pub struct TimecurvePoint {
    pub label: String,
    // t <-> timelabel sous forme de nombre pour le format de fichier input par défaut
    // TODO : parse le timelabel pour le transformer en temps UNIX, pas encore implémenté
    pub t: Option<u64>,
    pub pos: [f64; 2],
    pub control_points: Option<ControlPoints>, // option parce que pas encore implémenté
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
                    pos: projected_points[i],
                    // TODO : calcul des control points avec méthode variable comme l'algo
                    // de projection
                    control_points: None,
                });

                i = i + 1;
            }
            timecurves.push(timecurve);
        }
        timecurves
    }
}

// TODO : implémenter une méthode de Timecurve pour normaliser les points
// par exemple centrer les points sur (0,0) et faire en sorte qu'ils aillent de -1 à 1 ou je sais pas
// parce que pour l'instant la position des points dépend de la matrice de distance,
// c.a.d que si les distance sont genre 0.3, 0.6, etc la courbe finale sera toute petite
// alors que si les distances sont 300, 567, etc la courbe sera énorme

// TODO : implémenter une méthode de timecurve qui tourne les points autour de l'origine en fonction du temps t
// pour que le sens de lecture s'aligne sur gauche -> droite
