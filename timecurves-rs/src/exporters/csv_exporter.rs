use super::exporter::Exporter;
use std::fmt::Write;

pub struct CSVExporter {}

impl CSVExporter {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Exporter for CSVExporter {
    fn export(&self, curves: &Vec<crate::timecurve::Timecurve>) -> String {
        let mut output = String::new();

        // CSV header
        for (id, curve) in curves.iter().enumerate() {
            write!(
                &mut output,
                "{1}{0}_px,{0}_py,{0}_cx,{0}_cy",
                curve.name.as_str(),
                if id > 0 { "," } else { "" }
            )
            .unwrap(); // https://stackoverflow.com/questions/28333612/how-can-i-append-a-formatted-string-to-an-existing-string
                       // impossible que write! produise une erreur, donc on peut unwrap
        }

        output.push('\n');

        // points values
        for (id, curve) in curves.iter().enumerate() {
            for point in &curve.points {
                for _ in 0..id {
                    output.push_str(",,,,");
                }

                write!(&mut output, "{},{},,", point.pos.0, point.pos.1).unwrap();

                for _ in 0..curves.len() - id - 1 {
                    output.push_str(",,,,");
                }

                output.push('\n');

                if let Some(p) = point.c_prev {
                    for _ in 0..id {
                        output.push_str(",,,,");
                    }
                    write!(&mut output, ",,{},{}", p.0, p.1).unwrap();
                    for _ in 0..curves.len() - id - 1 {
                        output.push_str(",,,,");
                    }

                    output.push('\n');
                }

                if let Some(p) = point.c_next {
                    for _ in 0..id {
                        output.push_str(",,,,");
                    }
                    write!(&mut output, ",,{},{}", p.0, p.1).unwrap();
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
