use serde_json::json;

use crate::timecurve::TimecurveSet;

use super::Exporter;

pub struct VegaLiteExporter {
    size: u64,
}

impl VegaLiteExporter {
    pub fn new(size: Option<f64>) -> Self {
        return Self {
            size: size.unwrap_or(400.0) as u64,
        };
    }
}

impl Exporter for VegaLiteExporter {
    fn export(&self, timecurve_set: &TimecurveSet) -> String {
        let mut output = String::new();

        let data = json!({
            "values": timecurve_set.curves.iter().flat_map(|curve| {
                curve.points.iter().map(|point| {
                    json!({"curve" : curve.name, "x": point.pos.0, "y": point.pos.1, "label": point.label, "t": point.t})
                })
            }).collect::<Vec<serde_json::Value>>()
        });

        let vega_object = json!({
            "$schema" : "https://vega.github.io/schema/vega-lite/v5.json",
            "width": self.size,
            "height": self.size,
            "data" : data,
            "mark": {
                "type": "line",
                "point": {"size" : 50},
                "interpolate":"catmull-rom"
            },
            "params": [{
                "name": "grid",
                "select": "interval",
                "bind": "scales"
            }],
            "encoding": {
                "x": {
                  "field": "x", "type": "quantitative",
                  "scale": {"domain": [0, 1]}
                },
                "y": {
                  "field": "y", "type": "quantitative",
                  "scale": {"domain": [0, 1]}
                },
                "color": {"field": "curve", "type": "nominal"},
                "order": {"field": "t"},
                "tooltip": {"field" : "label"}
            },
        });

        output.push_str(&serde_json::to_string_pretty(&vega_object).unwrap());
        return output;
    }
}
