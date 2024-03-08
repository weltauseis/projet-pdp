#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub type Vect = Position;

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x, y };
    }
}

impl Vect {
    pub fn normalise(&mut self) {
        let length = (self.x.powi(2) + self.y.powi(2)).sqrt();
        self.x = self.x / length;
        self.y = self.y / length;
    }
}
