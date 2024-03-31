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
        for curve in &timecurve_set.curves {
            for point in &curve.points {
                output.push_str(&format!(
                    "{},{},{},{}\n",
                    curve.name, point.label, point.pos.0, point.pos.1
                ));
            }
        }

        return output;
    }
}
