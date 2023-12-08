use bevy::render::color::Color;

pub fn colors_equal(lhs: Color, rhs: Color) -> bool {
    let max_diff = 1. / 255.;
    (lhs.r() - rhs.r()).abs() < max_diff
        && (lhs.g() - rhs.g()).abs() < max_diff
        && (lhs.b() - rhs.b()).abs() < max_diff
}

pub fn lerp_colors(lhs: Color, rhs: Color, t: f32) -> Color {
    lhs.as_rgba_linear() * (1.0 - t) + rhs.as_rgba_linear() * t
}
