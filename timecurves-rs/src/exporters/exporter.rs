use crate::timecurve::Timecurve;

pub trait Exporter {
    fn export(&self, curves: &Vec<Timecurve>) -> String;
}
