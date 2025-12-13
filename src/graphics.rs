use macroquad::{color::Color, text::{Font, TextParams, draw_text_ex}};

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
