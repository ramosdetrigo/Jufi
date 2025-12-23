use macroquad::{color::Color, rand::RandomRange, text::{Font, TextParams, draw_text_ex}};

#[inline(always)]
pub fn randf_range(low: f64, high: f64) -> f64 {
    RandomRange::gen_range(low, high)
}

#[allow(dead_code)]
/// Wrapper to show text on screen.
pub fn print(text: &str, x: f32, y: f32, font_size: u16, color: Color, font: Option<&Font>) {
    draw_text_ex(
        text,
        x,
        // Macroquad has a weird offset in text drawing. This kind of fixes it.
        y + (font_size as f32 / 1.25),
        TextParams {
            font,
            font_size,
            color,
            ..Default::default()
        },
    );
}
