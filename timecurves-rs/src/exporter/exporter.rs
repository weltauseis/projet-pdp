use crate::timecurve::Timecurve;

use super::text_builder::TextBuilder;

pub fn export(curves: &Vec<Timecurve>, builder: &impl TextBuilder) -> String {
    let mut result = String::new();

    match builder.begin_document(curves) {
        Some(s) => result.push_str(&s),
        None => {}
    }

    for curve in curves {
        for point in &curve.points {
            if let Some(s) = builder.add_point(point.pos.0, point.pos.1, &curve.name) {
                result.push_str(&s);
            }

            if let Some(p) = point.c_prev {
                if let Some(s) = builder.add_control_point(p.0, p.1, &curve.name) {
                    result.push_str(&s)
                }
            }

            if let Some(p) = point.c_next {
                if let Some(s) = builder.add_control_point(p.0, p.1, &curve.name) {
                    result.push_str(&s)
                }
            }
        }
    }

    return result;
}
