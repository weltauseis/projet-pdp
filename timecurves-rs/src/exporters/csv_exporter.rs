use crate::timecurve::TimecurveSet;

use super::exporter::Exporter;

pub struct CSVExporter {}

impl CSVExporter {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Exporter for CSVExporter {
    fn export(&self, timecurve_set: &TimecurveSet) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("curve,label,x,y\n");

        // points values
        for curve in timecurve_set.get_curves() {
            for point in curve.get_points() {
                output.push_str(&format!(
                    "{},{},{},{}\n",
                    curve.get_name(),
                    point.get_label(),
                    point.get_pos().get_x(),
                    point.get_pos().get_y(),
                ));
            }
        }

        return output;
    }
}
