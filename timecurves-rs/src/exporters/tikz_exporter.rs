use crate::timecurve::TimecurveSet;

use super::Exporter;

/// An exporter to Tikz format.
pub struct TikzExporter {
    drawing_size: f64,
    thickness: f64,
}

impl TikzExporter {
    /// Creates a new instance of the Tikz exporter.
    ///
    /// ### Arguments
    ///
    /// * `drawing_size` - The size of the drawing in cm.
    /// * `thickness` - The thickness of the lines and points in the Tikz drawing. 1.0 is the default value.
    pub fn new(drawing_size: f64, thickness: f64) -> Self {
        return Self {
            drawing_size,
            thickness,
        };
    }
}

impl Exporter for TikzExporter {
    /// Exports the timecurve set to a Tikz string.
    ///
    /// ### Arguments
    ///
    /// * `timecurve_set` - The timecurve set to be exported.
    ///
    /// ### Returns
    ///
    /// The exported data as a string in Tikz format.
    /// The string opens and closes a Tikz picture environment, so it can be inserted directly in a LaTeX document.
    fn export(&self, timecurve_set: &TimecurveSet) -> String {
        let mut output = String::new();

        let point_width = (self.drawing_size / 100.0) * self.thickness;
        let line_width = (self.drawing_size / 150.0) * self.thickness;

        // header
        output.push_str("\\begin{tikzpicture}\n");
        output.push_str(&format!(
            "\\draw[thin,dotted] (0,0) grid ({0},{0});\n",
            self.drawing_size
        ));

        // draw the lines first so they are in the background
        for curve in timecurve_set.get_curves().iter() {
            // for each overlapping couple of 2 points
            for i in 0..curve.get_points().len() - 1 {
                let p1 = &curve.get_points()[i];
                let p2 = &curve.get_points()[i + 1];

                // draw the spline between the two points
                output.push_str(&format!(
                    "\\draw [line width={:.4}cm, color={{rgb, 255:red, {}; green, {}; blue, {}}}] ({},{}) .. controls ({},{}) and ({},{}) .. ({},{});\n",
                    line_width,
                    p2.get_color().0,
                    p2.get_color().1,
                    p2.get_color().2,
                    p1.get_pos_x() * self.drawing_size,
                    p1.get_pos_y() * self.drawing_size,
                    p1.get_c_next().unwrap().get_x() * self.drawing_size,
                    p1.get_c_next().unwrap().get_y() * self.drawing_size,
                    p2.get_c_prev().unwrap().get_x() * self.drawing_size,
                    p2.get_c_prev().unwrap().get_y() * self.drawing_size,
                    p2.get_pos_x() * self.drawing_size,
                    p2.get_pos_y() * self.drawing_size,
                ));
            }
        }

        // draw the points last so they sit on top of the lines
        for curve in timecurve_set.get_curves().iter() {
            for point in curve.get_points().iter() {
                output.push_str(&format!(
                    "\\draw[color=white, thick, fill={{rgb, 255:red, {}; green, {}; blue, {}}}] ({},{}) circle ({});\n",
                    point.get_color().0,
                    point.get_color().1,
                    point.get_color().2,
                    point.get_pos_x() * self.drawing_size,
                    point.get_pos_y() * self.drawing_size,
                    point_width
                ));
            }
        }

        // end of file
        output.push_str("\\end{tikzpicture}\n");

        return output;
    }
}
