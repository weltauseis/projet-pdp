use super::exporter::Exporter;
use std::fmt::Write;

pub struct CSVExporter {}

impl CSVExporter {
    pub fn new() -> Self {
        return Self {};
    }
}

// il faut que je modifie, c'est dégueulasse parce que j'export aussi les points de controles pour debug
// donc il y a plein de colonnes vides
// mais in fine ça sera beaucoup plus propres vu qu'il y aura que les points normaux dans le csv
impl Exporter for CSVExporter {
    fn export(&self, curves: &Vec<crate::timecurve::Timecurve>) -> String {
        let mut output = String::new();

        // CSV header
        for (id, curve) in curves.iter().enumerate() {
            output.push_str(&format!(
                "{1}{0}_px,{0}_py,{0}_cx,{0}_cy",
                curve.name.as_str(),
                if id > 0 { "," } else { "" }
            ));
        }

        output.push('\n');

        // points values
        for (id, curve) in curves.iter().enumerate() {
            for point in &curve.points {
                for _ in 0..id {
                    output.push_str(",,,,");
                }

                output.push_str(&format!("{},{},,", point.pos.0, point.pos.1));

                for _ in 0..curves.len() - id - 1 {
                    output.push_str(",,,,");
                }

                output.push('\n');

                if let Some(p) = point.c_prev {
                    for _ in 0..id {
                        output.push_str(",,,,");
                    }
                    output.push_str(&format!(",,{},{}", p.0, p.1));
                    for _ in 0..curves.len() - id - 1 {
                        output.push_str(",,,,");
                    }

                    output.push('\n');
                }

                if let Some(p) = point.c_next {
                    for _ in 0..id {
                        output.push_str(",,,,");
                    }
                    output.push_str(&format!(",,{},{}", p.0, p.1));
                    for _ in 0..curves.len() - id - 1 {
                        output.push_str(",,,,");
                    }

                    output.push('\n');
                }
            }
        }

        return output;
    }
}
