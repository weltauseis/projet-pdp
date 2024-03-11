#[allow(dead_code)]
// kind = type d'erreur, on peut s'en servir pour écrire un message différent en fonction de l'erreur
// info : information ADDITIONNELLE sur l'erreur, pour aider à la comprendre
// c.a.d : pas besoin dire "matrix is not square" dans info, on le sait déjà grâce à kind
// mais plutot "matrix has 3 rows ≠ 4 columns," par exemple
// c'est raisonnable non ?? dites moi si vous avez une meilleure idée pour la gestion d'erreur
#[derive(Debug)]
pub struct TimecurveError {
    kind: TimeCurveErrorKind,
    info: String,
}

#[derive(Debug)]
pub enum TimeCurveErrorKind {
    NonSquareDistanceMatrix,
    EvaluatedOutsideRange,
    InvalidTimeLabel,
}

impl std::fmt::Display for TimecurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl TimecurveError {
    pub fn new(kind: TimeCurveErrorKind, info: &str) -> Self {
        Self {
            kind,
            info: String::from(info),
        }
    }
}
