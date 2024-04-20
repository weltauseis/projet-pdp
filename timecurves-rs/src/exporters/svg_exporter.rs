use super::Exporter;

/// An exporter to SVG format.
pub struct SVGExporter {
    thickness: f64,
}

impl SVGExporter {
    /// Creates a new instance of the SVG exporter.
    ///
    /// ### Arguments
    ///
    /// * `thickness` - The thickness of the lines and points in the SVG. 1.0 is the default value.
    pub fn new(thickness: f64) -> Self {
        return Self { thickness };
    }
}

impl Exporter for SVGExporter {
    /// Exports the timecurve set to an SVG string.
    ///
    /// ### Arguments
    /// * `timecurve_set` - The timecurve set to be exported.
    ///
    /// ### Returns
    ///
    /// The exported curves as an SVG string. Each SVG circle element has a `data-timelabel` attribute that contains the time label of the point.
    /// This attribute can be used to display the time label when hovering over the point in a web browser, for custom visualizations.
    fn export(&self, timecurve_set: &crate::timecurve::TimecurveSet) -> String {
        let mut output = String::new();

        const PADDING: f64 = 0.1;

        // svg opening tags
        output.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"100%\" height=\"100%\" viewBox=\"0 0 {} {}\">\n", 
        1.0 + PADDING * 2.0,
        1.0 + PADDING * 2.0));

        // draw the lines first so they are in the background
        for curve in timecurve_set.get_curves().iter() {
            // for each overlapping couple of 2 points
            for i in 0..curve.get_points().len() - 1 {
                let p1 = &curve.get_points()[i];
                let p2 = &curve.get_points()[i + 1];

                // draw the spline between the two points
                output.push_str(&format!(
                    "<path d=\"M {} {} C {} {} {} {} {} {}\" fill=\"none\" stroke=\"rgb({},{},{})\" stroke-width=\"{}\" />\n",
                    p1.get_pos_x() + PADDING,
                    1.0 - p1.get_pos_y() + PADDING, // because svg (0,0) is top left, so 1.0 - y to flip the y axis
                    p1.get_c_next().unwrap().get_x() + PADDING,
                    1.0 - p1.get_c_next().unwrap().get_y() + PADDING,
                    p2.get_c_prev().unwrap().get_x() + PADDING,
                    1.0 - p2.get_c_prev().unwrap().get_y() + PADDING,
                    p2.get_pos_x() + PADDING,
                    1.0 - p2.get_pos_y() + PADDING,
                    p2.get_color().0,
                    p2.get_color().1,
                    p2.get_color().2,
                    self.thickness / 150.0,
                ));
            }

            // draw the points last so they sit on top of the lines
            for point in curve.get_points().iter() {
                output.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"rgb({},{},{})\" data-timelabel=\"{}\"/>\n",
                    point.get_pos_x() + PADDING,
                    1.0 - point.get_pos_y() + PADDING,
                    self.thickness / 120.0,
                    point.get_color().0,
                    point.get_color().1,
                    point.get_color().2,
                    point.get_label(),
                ));
            }

            // draw control points for debugging
            /*            for point in curve.get_points().iter() {
                if let Some(c_next) = point.get_c_next() {
                    output.push_str(&format!(
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"red\" />\n",
                        c_next.0 + PADDING,
                        1.0 - c_next.1 + PADDING,
                        self.thickness / 200.0,
                    ));
                }

                if let Some(c_prev) = point.get_c_prev() {
                    output.push_str(&format!(
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"blue\" />\n",
                        c_prev.0 + PADDING,
                        1.0 - c_prev.1 + PADDING,
                        self.thickness / 200.0,
                    ));
                }
            } */
        }

        // svg closing tags
        output.push_str("</svg>");

        return output;
    }
}
