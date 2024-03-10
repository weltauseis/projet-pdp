use crate::timecurve::Timecurve;

pub trait TextBuilder {
    fn begin_document(&self, curves: &Vec<Timecurve>) -> Option<String>;
    fn add_point(&self, x: f64, y: f64, curve_name: &str) -> Option<String>;
    fn add_control_point(&self, x: f64, y: f64, name: &str) -> Option<String>;
    fn add_edge(&self, x1: f64, y1: f64, x2: f64, y2: f64, name: &str) -> Option<String>;
    fn end_document(&self) -> Option<String>;
}
