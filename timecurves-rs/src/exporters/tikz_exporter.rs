use crate::timecurve::TimecurveSet;

use super::Exporter;

pub struct TikzExporter {
    drawing_size: f64,
}

impl TikzExporter {
    pub fn new(drawing_size: Option<f64>) -> Self {
        return Self {
            drawing_size: drawing_size.unwrap_or(10.0),
        };
    }
}

impl Exporter for TikzExporter {
    fn export(&self, timecurve_set: &TimecurveSet) -> String {
        let mut output = String::new();

        let point_width = self.drawing_size / 100.0;
        let line_width = self.drawing_size / 150.0;

        // header
        output.push_str("\\begin{tikzpicture}\n");
        output.push_str(&format!(
            "\\draw[thin,dotted] (0,0) grid ({0},{0});\n",
            self.drawing_size
        ));

        // draw the lines first so they are in the background
        for (curve_id, curve) in timecurve_set.curves.iter().enumerate() {
            // for each overlapping couple of 2 points
            for i in 0..curve.points.len() - 1 {
                let p1 = &curve.points[i];
                let p2 = &curve.points[i + 1];

                let u = i as f32 / (curve.points.len() - 1) as f32;
                let color = super::curve_color_lerp(curve_id, u);

                // draw the spline between the two points
                output.push_str(&format!(
                    "\\draw [line width={:.4}cm, color={{rgb, 255:red, {}; green, {}; blue, {}}}] ({},{}) .. controls ({},{}) and ({},{}) .. ({},{});\n",
                    line_width,
                    color.0,
                    color.1,
                    color.2,
                    p1.pos.0 * self.drawing_size,
                    p1.pos.1 * self.drawing_size,
                    p1.c_next.unwrap().0 * self.drawing_size,
                    p1.c_next.unwrap().1 * self.drawing_size,
                    p2.c_prev.unwrap().0 * self.drawing_size,
                    p2.c_prev.unwrap().1 * self.drawing_size,
                    p2.pos.0 * self.drawing_size,
                    p2.pos.1 * self.drawing_size,
                ));
            }
        }

        // draw the points last so they sit on top of the lines
        for (curve_id, curve) in timecurve_set.curves.iter().enumerate() {
            for (i, point) in curve.points.iter().enumerate() {
                let u = i as f32 / (curve.points.len() - 1) as f32;
                let color = super::curve_color_lerp(curve_id, u);

                output.push_str(&format!(
                    "\\draw[color=white, thick, fill={{rgb, 255:red, {}; green, {}; blue, {}}}] ({},{}) circle ({});\n",
                    color.0,
                    color.1,
                    color.2,
                    point.pos.0 * self.drawing_size,
                    point.pos.1 * self.drawing_size,
                    point_width
                ));
            }
        }

        // end of file
        output.push_str("\\end{tikzpicture}\n");

        return output;
    }
}
