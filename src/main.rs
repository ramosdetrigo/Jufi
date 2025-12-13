use macroquad::{color, prelude::*};

mod algebra; // Vector and matrix classes, etc. Algebra.
use algebra::Vec2;

use crate::physics::shapes::Line;
mod graphics; // Wrappers to more easily draw stuff on screen
mod physics; // Physics stuff: Shapes, collision detection, etc.

#[macroquad::main("Hello, World!")]
async fn main() {
    // a fonte que iremos usar
    let font = load_ttf_font("NunitoSans-Regular.ttf").await.unwrap();

    let mut reta1 = Line::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
    let mut reta_mouse = reta1.clone();

    loop {
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);

        // Clicando com o botão esquerdo você muda o ponto p1 da reta
        if is_mouse_button_pressed(MouseButton::Left) {
            reta_mouse.p1 = mouse_pos;
        }
        // E o ponto p2 segue o mouse.
        reta_mouse.p2 = mouse_pos;

        reta1.draw(2.0, color::VIOLET);
        reta_mouse.draw(2.0, color::SKYBLUE);

        clear_background(WHITE);
        graphics::print(
            "Hello, World!",
            mouse.x as f32,
            mouse.y as f32,
            24,
            color::BLACK,
            Some(&font),
        );

        next_frame().await
    }
}
