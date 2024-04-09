use super::Exporter;

pub struct SVGExporter {
    pub thickness: f64,
}

impl SVGExporter {
    pub fn new(thickness: f64) -> Self {
        return Self { thickness };
    }
}

impl Exporter for SVGExporter {
    fn export(&self, timecurve_set: &crate::timecurve::TimecurveSet) -> String {
        let mut output = String::new();

        const PADDING: f64 = 0.1;

        // svg opening tags
        output.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"100%\" height=\"100%\" viewBox=\"0 0 {} {}\">\n", 
        1.0 + PADDING * 2.0,
        1.0 + PADDING * 2.0));

        // draw the lines first so they are in the background
        for (curve_id, curve) in timecurve_set.get_curves().iter().enumerate() {
            // for each overlapping couple of 2 points
            for i in 0..curve.get_points().len() - 1 {
                let p1 = &curve.get_points()[i];
                let p2 = &curve.get_points()[i + 1];

                let u = i as f32 / (curve.get_points().len() - 1) as f32;
                let color = super::curve_color_lerp(curve_id, u);

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
                    color.0,
                    color.1,
                    color.2,
                    self.thickness / 150.0,
                ));
            }

            // draw the points last so they sit on top of the lines
            for (i, point) in curve.get_points().iter().enumerate() {
                let u = i as f32 / (curve.get_points().len() - 1) as f32;
                let color = super::curve_color_lerp(curve_id, u);

                output.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"rgb({},{},{})\" data-timelabel=\"{}\"/>\n",
                    point.get_pos_x() + PADDING,
                    1.0 - point.get_pos_y() + PADDING,
                    self.thickness / 120.0,
                    color.0,
                    color.1,
                    color.2,
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
