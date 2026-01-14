use jufi::{
    algebra::Vec2,
    physics::{
        generators::point_cloud,
        shapes::{AABB, Circle, OBB},
    },
    utils::{print, randf_range},
};
use macroquad::{color, prelude::*, rand::srand};


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

    // Gera nuvem de pontos aleatória e suas bounding boxes
    let mut points = point_cloud(randf_range(3, 50), 200.0, 150.0, 600.0, 450.0);
    let mut aabb = AABB::enclosing(&points);
    let mut circle = Circle::enclosing(&points);
    let mut obb = OBB::enclosing(&points);
    let mut mouse_point;

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);
        let _delta = get_frame_time();

        // Move o ponto do mouse
        mouse_point = mouse_pos;

        // Regenera a nuvem
        if is_key_pressed(KeyCode::Space) {
            points = point_cloud(randf_range(3, 50), 200.0, 150.0, 600.0, 450.0);
            aabb = AABB::enclosing(&points);
            circle = Circle::enclosing(&points);
            obb = OBB::enclosing(&points);
        }

        // Adiciona ponto na nuvem
        if is_mouse_button_pressed(MouseButton::Left) {
            points.push(mouse_point);
            aabb = AABB::enclosing(&points);
            circle = Circle::enclosing(&points);
            obb = OBB::enclosing(&points);
        }

        // Desenha as boundaries e a nuvem
        aabb.draw(2.0, color::WHITE);
        circle.draw(2.0, color::WHITE);
        obb.draw(2.0, color::WHITE);
        for p in &points {
            p.draw(color::YELLOW);
        }

        // Desenha o ponto do mouse e avisos de "colisão"
        mouse_point.draw(color::RED);
        if obb.contains_point(mouse_point) {
            print("O mouse está dentro da OBB!", 10.0, 10.0, 20, color::PINK, Some(&nunito));
        }
        if aabb.contains_point(mouse_point) {
            print("O mouse está dentro da AABB!", 10.0, 30.0, 20, color::YELLOW, Some(&nunito));
        }
        if circle.contains_point(mouse_point) {
            print("O mouse está dentro do círculo!", 10.0, 50.0, 20, color::SKYBLUE, Some(&nunito));
        }

        // Mostra a ajuda na tela
        print("Left click - Adicionar ponto", 10.0, 550.0, 20, color::WHITE, Some(&nunito));
        print("Espaço - Randomizar nuvem", 10.0, 570.0, 20, color::WHITE, Some(&nunito));
        next_frame().await
    }
}
