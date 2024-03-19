use super::Exporter;

pub struct SVGExporter;

impl SVGExporter {
    pub fn new() -> Self {
        return Self;
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
        for (curve_id, curve) in timecurve_set.curves.iter().enumerate() {
            // for each overlapping couple of 2 points
            for i in 0..curve.points.len() - 1 {
                let p1 = &curve.points[i];
                let p2 = &curve.points[i + 1];

                let u = i as f32 / (curve.points.len() - 1) as f32;
                let color = super::curve_color_lerp(curve_id, u);

                // draw the spline between the two points
                output.push_str(&format!(
                    "<path d=\"M {} {} C {} {} {} {} {} {}\" fill=\"none\" stroke=\"rgb({},{},{})\" stroke-width=\"0.01\" />\n",
                    p1.pos.0 + PADDING,
                    p1.pos.1 + PADDING,
                    p1.c_next.unwrap().0 + PADDING,
                    p1.c_next.unwrap().1 + PADDING,
                    p2.c_prev.unwrap().0 + PADDING,
                    p2.c_prev.unwrap().1 + PADDING,
                    p2.pos.0 + PADDING,
                    p2.pos.1 + PADDING,
                    color.0,
                    color.1,
                    color.2,
                ));
            }

            // draw the points last so they sit on top of the lines
            for (i, point) in curve.points.iter().enumerate() {
                let u = i as f32 / (curve.points.len() - 1) as f32;
                let color = super::curve_color_lerp(curve_id, u);

                output.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"0.01\" fill=\"rgb({},{},{})\" />\n",
                    point.pos.0 + PADDING,
                    point.pos.1 + PADDING,
                    color.0,
                    color.1,
                    color.2,
                ));
            }
        }

        // svg closing tags
        output.push_str("</svg>");

        return output;
    }
}
