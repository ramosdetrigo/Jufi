use std::{f64::consts::PI, ptr};

use jufi::{
    algebra::Vec2,
    physics::{
        generators::point_cloud_radial,
        shapes::{AABB, Circle, Collider, OBB, collides},
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
    let mut obb1 = OBB::enclosing(&cloud2);
    let mut aabb2 = AABB::enclosing(&cloud3);
    let mut circle = Circle::enclosing(&cloud4);
    let mut obb2 = OBB::enclosing(&cloud5);

    // Gera um círculo que vai seguir o mouse
    let mut mouse: Box<dyn Collider> = Box::new(Circle::new(Vec2::NULL, 10.0));

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);
        let _delta = get_frame_time();

        // Randomiza as respectivas nuvens ao pressionar as teclas de 1 a 4
        if is_key_pressed(KeyCode::Key1) {
            cloud1 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 250.0), 100.0);
            aabb1 = AABB::enclosing(&cloud1);
        }
        if is_key_pressed(KeyCode::Key2) {
            cloud2 = point_cloud_radial(randf_range(3, 50), Vec2::new(400.0, 250.0), 100.0);
            obb1 = OBB::enclosing(&cloud2)
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
            obb2 = OBB::enclosing(&cloud5);
        }

        // Controle a forma que segue o mouse
        mouse.set_center(mouse_pos); // Atualiza a posição

        let (dx, dy);
        // Lê a roda do mouse
        if is_key_down(KeyCode::LeftShift) {
            // Modifier: Shift -> Cresce na horizontal ao invés de na vertical
            dx = mouse_wheel().1 as f64 * 2.0;
            dy = 0.0;
        } else {
            dx = 0.0;
            dy = mouse_wheel().1 as f64 * 2.0;
        }
        mouse.grow(dx, dy);

        // Gira o objeto
        if is_key_down(KeyCode::A) {
            mouse.rotate(-0.025);
        } else if is_key_down(KeyCode::D) {
            mouse.rotate(0.025);
        }

        if is_key_pressed(KeyCode::Q) {
            // Circle
            let prev_size = mouse.size();
            mouse = Box::new(Circle::new(mouse.center(), prev_size.x.max(prev_size.y)))
        } else if is_key_pressed(KeyCode::W) {
            // AABB
            let offset = mouse.size() / 2.0;
            let prev_center = mouse.center();
            mouse = Box::new(AABB::new(prev_center - offset, prev_center + offset))
        } else if is_key_pressed(KeyCode::E) {
            // OBB
            let prev_size = mouse.size();
            let prev_center = mouse.center();
            mouse = Box::new(OBB::from_angle(prev_center, prev_size / 2.0, PI / 4.0))
        }

        // Desenha cada collider checando por colisão uma com a outra
        let colliders: [&dyn Collider; 6] = [&aabb1, &aabb2, &obb1, &obb2, &circle, &*mouse];

        colliders.iter().for_each(|c| {
            let is_hit = colliders
                .iter()
                // ptr::eq --> skipa colisão consigo mesmo
                .any(|other| !ptr::eq(c, other) && collides(*c, *other));
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
            "Q, W, D - Objeto do mouse vira círculo, AABB, ou OBB, respectivamente.",
            10.0,
            24.0,
            16,
            color::WHITE,
            Some(&nunito),
        );
        print(
            "Roda do mouse - Aumenta o objeto do mouse (shift para aumentar na horizontal)",
            10.0,
            38.0,
            16,
            color::WHITE,
            Some(&nunito),
        );
        print(
            "A e D - Gira o objeto (só OBB)",
            10.0,
            52.0,
            16,
            color::WHITE,
            Some(&nunito),
        );
        print(
            "Amarelo: colisão | Branco: sem colisão",
            10.0,
            66.0,
            16,
            color::WHITE,
            Some(&nunito),
        );

        next_frame().await;
    }
}
