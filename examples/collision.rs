use std::ptr;

use jufi::{
    algebra::Vec2,
    physics::{
        generators::point_cloud_radial,
        shapes::{AABB, BoxCollider, Circle, OOBB},
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

    // Gera nuvens aleatórias e suas bounding boxes
    let mut cloud1 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 250.0), 100.0);
    let mut cloud2 = point_cloud_radial(randf_range(3, 50), Vec2::new(400.0, 250.0), 100.0);
    let mut cloud3 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 400.0), 100.0);
    let mut cloud4 = point_cloud_radial(randf_range(3, 50), Vec2::new(500.0, 450.0), 100.0);
    let mut cloud5 = point_cloud_radial(randf_range(3, 50), Vec2::new(300.0, 150.0), 100.0);

    let mut aabb1 = AABB::enclosing(&cloud1);
    let mut oobb1 = OOBB::enclosing(&cloud2);
    let mut aabb2 = AABB::enclosing(&cloud3);
    let mut circle = Circle::enclosing(&cloud4);
    let mut oobb2 = OOBB::enclosing(&cloud5);

    // Gera um círculo que vai seguir o mouse
    let mut mouse_circle = Circle::new(Vec2::NULL, 10.0);

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);
        let _delta = get_frame_time();

        // Atualiza a posição e o tamanho do círculo
        mouse_circle.center = mouse_pos;
        mouse_circle.radius += mouse_wheel().1 as f64 * 2.0; // Y da roda do mouse

        // Randomiza as respectivas nuvens ao pressionar as teclas de 1 a 4
        if is_key_pressed(KeyCode::Key1) {
            cloud1 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 250.0), 100.0);
            aabb1 = AABB::enclosing(&cloud1);
        }
        if is_key_pressed(KeyCode::Key2) {
            cloud2 = point_cloud_radial(randf_range(3, 50), Vec2::new(400.0, 250.0), 100.0);
            oobb1 = OOBB::enclosing(&cloud2)
        }
        if is_key_pressed(KeyCode::Key3) {
            cloud3 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 400.0), 100.0);
            aabb2 = AABB::enclosing(&cloud3);
        }
        if is_key_pressed(KeyCode::Key4) {
            cloud4 = point_cloud_radial(randf_range(3, 50), Vec2::new(500.0, 450.0), 100.0);
            circle = Circle::enclosing(&cloud4);
        }
        if is_key_pressed(KeyCode::Key5) {
            cloud5 = point_cloud_radial(randf_range(3, 50), Vec2::new(300.0, 150.0), 100.0);
            oobb2 = OOBB::enclosing(&cloud5);
        }

        // Desenha cada bounding box checando por colisão uma com a outra
        let boxes: [&dyn BoxCollider; 4] = [&aabb1, &aabb2, &oobb1, &oobb2];
        let circles: [&Circle; 2] = [&circle, &mouse_circle];
        
        boxes.iter().for_each(|c| {
            let is_hit = boxes.iter()
                // Teste para caixas (skipa colisão consigo mesmo)
                .any(|other| !ptr::eq(c, other) && c.collides_with_box(*other))
                // Teste para círculos
                || circles.iter().any(|other| c.collides_with_circle(other));
            c.draw(2.0, if is_hit { color::YELLOW } else { color::WHITE })
        });

        circles.iter().for_each(|c| {
            let is_hit = boxes.iter()
                // Teste para caixas
                .any(|other| c.collides_with_box(*other))
                // Teste para círculos (skipa colisão consigo mesmo)
                || circles.iter().any(|other| c != other && c.collides_with_circle(other));
            c.draw(2.0, if is_hit { color::YELLOW } else { color::WHITE })
        });

        // Desenha as nuvens de pontos
        cloud1.iter().for_each(|p| p.draw(color::RED));
        cloud2.iter().for_each(|p| p.draw(color::GREEN));
        cloud3.iter().for_each(|p| p.draw(color::BLUE));
        cloud4.iter().for_each(|p| p.draw(color::PINK));
        cloud5.iter().for_each(|p| p.draw(color::BROWN));

        // Tutorial
        print(
            "1 a 5 - Randomiza nuvens (1) vermelha, (2) verde, (3) azul, (4) rosa e (5) marrom",
            10.0,
            10.0,
            16,
            color::WHITE,
            Some(&nunito),
        );
        print(
            "Roda do mouse - Aumenta o círculo do mouse",
            10.0,
            24.0,
            16,
            color::WHITE,
            Some(&nunito),
        );
        print(
            "Amarelo: colisão | Branco: sem colisão",
            10.0,
            38.0,
            16,
            color::WHITE,
            Some(&nunito),
        );

        next_frame().await;
    }
}
