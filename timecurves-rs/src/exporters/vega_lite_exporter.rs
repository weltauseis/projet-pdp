/*
* Copyright (c) 2024, Kevin Jourdain
*
* SPDX-License-Identifier: BSD-3-Clause
*/

use serde_json::json;

use crate::timecurve::TimecurveSet;

use super::Exporter;

/// An exporter to Vega-Lite format.
pub struct VegaLiteExporter {
    size: u64,
}

impl VegaLiteExporter {
    /// Creates a new instance of the Vega-Lite exporter.
    ///
    /// ### Arguments
    ///
    /// * `size` - The size of the output in pixels.
    pub fn new(size: u64) -> Self {
        return Self { size };
    }
}

impl Exporter for VegaLiteExporter {
    /// Exports the timecurve set to a Vega-Lite string.
    ///
    /// ### Arguments
    ///
    /// * `timecurve_set` - The timecurve set to be exported.
    ///
    /// ### Returns
    ///
    /// The exported data as a string in Vega-Lite format.
    fn export(&self, timecurve_set: &TimecurveSet) -> String {
        let mut output = String::new();

        let data = json!({
            "values": timecurve_set.get_curves().iter().flat_map(|curve| {
                curve.get_points().iter().map(|point| {
                    json!({"curve" : curve.get_name(), "x": point.get_pos_x(), "y": point.get_pos_y(), "label": point.get_label(), "t": point.get_t()})
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
