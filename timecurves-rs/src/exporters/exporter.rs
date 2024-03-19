use crate::timecurve::TimecurveSet;
use palette::{Darken, Hsv, IntoColor, Mix, Srgb};

static COLORS: [(u8, u8, u8); 3] = [
    (255, 105, 22), // orange
    (34, 130, 251), // blue
    (149, 221, 60), // green
];

pub trait Exporter {
    fn export(&self, timecurve_set: &TimecurveSet) -> String;
}

pub fn curve_color_lerp(curve_id: usize, u: f32) -> (u8, u8, u8) {
    let color_id = curve_id % COLORS.len();

    let r = COLORS[color_id].0;
    let g = COLORS[color_id].1;
    let b = COLORS[color_id].2;

    let start_color: Hsv =
        Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0).into_color();
    let end_color = Hsv::from(start_color).darken(0.7);

    let color = start_color.mix(end_color, u);

    let srgb: Srgb = color.into_color();

    return (
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    );
}
