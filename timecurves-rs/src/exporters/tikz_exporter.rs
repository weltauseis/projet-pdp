use super::Exporter;

pub struct TikzExporter {
    point_width: f64,
    curve_width: f64,
}

impl TikzExporter {
    pub fn new(point_width: f64, curve_width: f64) -> Self {
        return Self {
            point_width,
            curve_width,
        };
    }
}

impl Exporter for TikzExporter {
    fn export(&self, curves: &Vec<crate::timecurve::Timecurve>) -> String {
        let mut output = String::new();

        // header
        output.push_str("\\documentclass[tikz,border=10pt]{standalone}\n");
        output.push_str("\\begin{document}\n");
        output.push_str("\\begin{tikzpicture}\n");
        output.push_str("\\draw[thin,dotted] (-1,-1) grid (1,1);\n");

        // points
        for curve in curves {
            for point in &curve.points {
                output.push_str(&format!(
                    "\\fill[black] ({},{}) circle ({});\n",
                    point.pos.0, point.pos.1, self.point_width
                ));
            }
        }

        // control points (optional)
        for curve in curves {
            for point in &curve.points {
                if let Some(c_prev) = point.c_prev {
                    output.push_str(&format!(
                        "\\fill[red] ({},{}) circle ({});\n",
                        c_prev.0,
                        c_prev.1,
                        self.point_width * 0.5
                    ));
                }
                if let Some(c_next) = point.c_next {
                    output.push_str(&format!(
                        "\\fill[blue] ({},{}) circle ({});\n",
                        c_next.0,
                        c_next.1,
                        self.point_width * 0.5
                    ));
                }
            }
        }

        // edges
        for curve in curves {
            for slice in curve.points.windows(2) {
                let p1 = &slice[0];
                let p2 = &slice[1];

                output.push_str(&format!(
                    "\\draw [line width={}] ({},{}) .. controls ({},{}) and ({},{}) .. ({},{});\n",
                    self.curve_width,
                    p1.pos.0,
                    p1.pos.1,
                    p1.c_next.unwrap().0,
                    p1.c_next.unwrap().1,
                    p2.c_prev.unwrap().0,
                    p2.c_prev.unwrap().1,
                    p2.pos.0,
                    p2.pos.1,
                ))
            }
        }

        // end of file
        output.push_str("\\end{tikzpicture}\n");
        output.push_str("\\end{document}\n");

        return output;
    }
}
