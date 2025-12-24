use std::f64::consts::PI;

use macroquad::{
    color,
    prelude::*,
    rand::{RandomRange, srand},
};

use jufi::{algebra::Vec2, utils::print};
use jufi::physics::shapes::{Line, Particle};

#[macroquad::main("Hello, World!")]
async fn main() {
    // Seta uma seed aleatória baseada no horário do sistema
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    srand(current_time);
    // Fonte principal
    let nunito = load_ttf_font("NunitoSans-Regular.ttf").await.unwrap();

    // Vetor de partículas
    let mut particles = vec![Particle::new(
        Vec2::new(400.0, 300.0),
        Vec2::new(200.0, 0.0).rotated(randf_range(-PI, PI)),
    )];

    // Gera um quadrado aleatório no centro da tela
    let points = [
        Vec2::new(200.0, 100.0) + Vec2::new(randf_range(-100.0, 100.0), randf_range(-100.0, 100.0)),
        Vec2::new(600.0, 100.0) + Vec2::new(randf_range(-100.0, 100.0), randf_range(-100.0, 100.0)),
        Vec2::new(600.0, 500.0) + Vec2::new(randf_range(-100.0, 100.0), randf_range(-100.0, 100.0)),
        Vec2::new(200.0, 500.0) + Vec2::new(randf_range(-100.0, 100.0), randf_range(-100.0, 100.0)),
    ];
    let mut lines = vec![
        Line::new(points[0], points[1]),
        Line::new(points[1], points[2]),
        Line::new(points[2], points[3]),
        Line::new(points[3], points[0]),
        Line::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
    ];

    let mut reta_mouse = Line::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);
        let delta = get_frame_time();

        // Adiciona uma nova partícula no sistema se espaço for pressionado
        if is_key_pressed(KeyCode::Space) {
            particles.push(Particle::new(
                Vec2::new(400.0, 300.0),
                Vec2::new(200.0, 0.0).rotated(randf_range(-PI, PI)),
            ));
        }

        // A última reta corresponde à posição do mouse.
        if is_mouse_button_pressed(MouseButton::Left) {
            reta_mouse.p1 = mouse_pos
        }
        reta_mouse.p2 = mouse_pos;
        reta_mouse.draw(2.0, color::BEIGE.with_alpha(0.25));

        if is_key_pressed(KeyCode::Enter) {
            lines.push(reta_mouse);
            reta_mouse.p1 = mouse_pos
        }

        // Desenha as retas
        for line in &lines {
            line.draw(2.0, color::WHITE);
        }

        // Desenha a partícula
        for particle in &mut particles {
            particle.update(delta as f64, &lines);
            particle.draw(color::RED);
            particle.draw_movement_line(delta as f64, 2.0, color::BLUE);
        }

        print("Espaço - Gera nova partícula", 10.0, 10.0, 16, color::WHITE, Some(&nunito));
        print("Clique esquerdo - Muda a posição da linha", 10.0, 26.0, 16, color::WHITE, Some(&nunito));
        print("Enter - Adiciona a nova linha", 10.0, 42.0, 16, color::WHITE, Some(&nunito));
        next_frame().await
    }
}

/// Função wrapper pra gerar números aleatórios
fn randf_range(low: f64, high: f64) -> f64 {
    RandomRange::gen_range(low, high)
}
