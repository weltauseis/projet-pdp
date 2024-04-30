/*
* Copyright (c) 2024, Kevin Jourdain
*
* SPDX-License-Identifier: BSD-3-Clause
*/

use crate::timecurve::TimecurveSet;

/// Trait representing an exporter for timecurve sets.
pub trait Exporter {
    /// Exports the given timecurve set and returns the exported data as a string of the desired format.
    ///
    /// ### Arguments
    ///
    /// * `timecurve_set` - The timecurve set to be exported.
    ///
    /// ### Returns
    ///
    /// The exported data as a string.
    fn export(&self, timecurve_set: &TimecurveSet) -> String;
}
