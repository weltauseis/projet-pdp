/*
* Copyright (c) 2024, Kevin Jourdain
*
* SPDX-License-Identifier: BSD-3-Clause
*/

use super::exporter::Exporter;
use crate::timecurve::TimecurveSet;

/// An exporter to CSV format.
pub struct CSVExporter {}

impl CSVExporter {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Exporter for CSVExporter {
    /// Exports the timecurve set to a CSV string.
    ///
    /// ### Arguments
    /// * `timecurve_set` - The timecurve set to be exported.
    ///
    /// ### Returns
    ///
    /// The exported data as a CSV string. The CSV file has the following columns:
    /// - `curve`: The name of the curve.
    /// - `label`: The label of the point.
    /// - `x`: The x-coordinate of the point.
    /// - `y`: The y-coordinate of the point.
    fn export(&self, timecurve_set: &TimecurveSet) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("curve,label,x,y\n");

        // points values
        for curve in timecurve_set.get_curves() {
            for point in curve.get_points() {
                output.push_str(&format!(
                    "{},{},{},{}\n",
                    curve.get_name(),
                    point.get_label(),
                    point.get_pos_x(),
                    point.get_pos_y(),
                ));
            }
        }

        return output;
    }
}
