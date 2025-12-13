use macroquad::{color, prelude::*, rand::RandomRange};

mod algebra; // Vector and matrix classes, etc. Algebra.
use algebra::Vec2;

use crate::physics::shapes::Line;
mod graphics; // Wrappers to more easily draw stuff on screen
mod physics; // Physics stuff: Shapes, collision detection, etc.

#[macroquad::main("Hello, World!")]
async fn main() {
    let mut reta1 = Line::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    let mut reta_mouse = reta1.clone();
    reta1.p1 = random_point(100, 500, 100, 500);
    reta1.p2 = random_point(100, 500, 100, 500);

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);

        // Clicando com o botão esquerdo você muda o ponto p1 da reta
        if is_mouse_button_pressed(MouseButton::Left) {
            reta_mouse.p1 = mouse_pos;
        }
        // E o ponto p2 segue o mouse.
        reta_mouse.p2 = mouse_pos;

        // Randomiza a reta1 quando você aperta espaço
        if is_key_pressed(KeyCode::Space) {
            reta1.p1 = random_point(100, 500, 100, 500);
            reta1.p2 = random_point(100, 500, 100, 500);
        }

        // Desenhamos a reta 1 (estática) de laranja
        reta1.draw(2.0, color::ORANGE);
        // Verde: colisão
        // Vermelho: sem colisão
        // Azul: retas paralelas
        // Ponto roxo: reta degenerada
        if reta_mouse.intersects(reta1) {
            reta_mouse.draw(2.0, color::RED);
            let p = reta_mouse.intersection_with(reta1);
            draw_circle(p.x as f32, p.y as f32, 5.0, color::VIOLET)
        } else {
            reta_mouse.draw(2.0, color::GREEN);
        }

        next_frame().await
    }
}

fn random_point(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Vec2 {
    let x = RandomRange::gen_range(min_x, max_x);
    let y = RandomRange::gen_range(min_y, max_y);

    return Vec2::new(x as f64, y as f64);
}
