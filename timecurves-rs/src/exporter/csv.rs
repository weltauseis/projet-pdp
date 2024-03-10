use crate::timecurve::Timecurve;

use super::text_builder::TextBuilder;

pub struct CSVBuilder {
    points_x_col_name: String,
    points_y_col_name: String,
    control_points_x_col_name: String,
    control_points_y_col_name: String,
    rounded: bool,
}

impl CSVBuilder {
    pub fn new(
        points_x_col_name: &str,
        points_y_col_name: &str,
        control_points_x_col_name: &str,
        control_points_y_col_name: &str,
        rounded: bool,
    ) -> Self {
        CSVBuilder {
            points_x_col_name: points_x_col_name.to_string(),
            points_y_col_name: points_y_col_name.to_string(),
            control_points_x_col_name: control_points_x_col_name.to_string(),
            control_points_y_col_name: control_points_y_col_name.to_string(),
            rounded,
        }
    }
}

impl TextBuilder for CSVBuilder {
    fn begin_document(&self, curves: &Vec<Timecurve>) -> Option<String> {
        return Some(String::from(format!(
            "{1}_{},{1}_{},{1}_{},{1}_{},\n",
            self.points_x_col_name,
            self.points_y_col_name,
            self.control_points_x_col_name,
            self.control_points_y_col_name
        )));
    }

    fn add_point(&self, x: f64, y: f64, _name: &str) -> Option<String> {
        if self.rounded {
            return Some(String::from(format!(
                "{:.2},{:.2},,\n",
                x.round(),
                y.round()
            )));
        }

        return Some(String::from(format!("{},{},,\n", x, y)));
    }

    fn add_control_point(&self, x: f64, y: f64, _name: &str) -> Option<String> {
        if self.rounded {
            return Some(String::from(format!(
                ",,,{:.2},{:.2},\n",
                x.round(),
                y.round()
            )));
        }
        return Some(String::from(format!(",,{},{},\n", x, y)));
    }

    fn add_edge(&self, _x1: f64, _y1: f64, _x2: f64, _y2: f64, _name: &str) -> Option<String> {
        return None;
    }

    fn end_document(&self) -> Option<String> {
        return None;
    }
}
