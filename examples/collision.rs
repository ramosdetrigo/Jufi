use jufi::{
    algebra::Vec2,
    physics::{
        generators::point_cloud_radial,
        shapes::{AABB, Circle, Collider, OOBB},
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

    let mut cloud1 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 200.0), 100.0);
    let mut cloud2 = point_cloud_radial(randf_range(3, 50), Vec2::new(400.0, 200.0), 100.0);
    let mut cloud3 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 350.0), 100.0);
    let mut cloud4 = point_cloud_radial(randf_range(3, 50), Vec2::new(500.0, 400.0), 100.0);

    let mut aabb1 = AABB::enclosing(&cloud1);
    let mut oobb = OOBB::enclosing(&cloud2);
    let mut aabb2 = AABB::enclosing(&cloud3);
    let mut circle = Circle::enclosing(&cloud4);

    let mut mouse_circle = Circle::new(Vec2::NULL, 10.0);

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);
        let _delta = get_frame_time();

        mouse_circle.center = mouse_pos;
        let (_, mouse_wheel_y) = mouse_wheel();
        mouse_circle.radius += mouse_wheel_y as f64 * 2.0;

        if is_key_pressed(KeyCode::Key1) {
            cloud1 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 200.0), 100.0);
            aabb1 = AABB::enclosing(&cloud1);
        }
        if is_key_pressed(KeyCode::Key2) {
            cloud2 = point_cloud_radial(randf_range(3, 50), Vec2::new(400.0, 200.0), 100.0);
            oobb = OOBB::enclosing(&cloud2)
        }
        if is_key_pressed(KeyCode::Key3) {
            cloud3 = point_cloud_radial(randf_range(3, 50), Vec2::new(200.0, 350.0), 100.0);
            aabb2 = AABB::enclosing(&cloud3);
        }
        if is_key_pressed(KeyCode::Key4) {
            cloud4 = point_cloud_radial(randf_range(3, 50), Vec2::new(500.0, 400.0), 100.0);
            circle = Circle::enclosing(&cloud4);
        }

        let colliders = [
            Collider::Circle(circle),
            Collider::Circle(mouse_circle),
            Collider::AABB(aabb1),
            Collider::AABB(aabb2),
            Collider::OOBB(oobb),
        ];

        colliders.iter().for_each(|c| {
            let is_hit = colliders.iter().any(|other| c != other && c.collides_with(*other));
            c.draw(2.0, if is_hit { color::YELLOW } else { color::WHITE })
        });

        cloud1.iter().for_each(|p| p.draw(color::RED));
        cloud2.iter().for_each(|p| p.draw(color::GREEN));
        cloud3.iter().for_each(|p| p.draw(color::BLUE));
        cloud4.iter().for_each(|p| p.draw(color::PINK));

        print("1 - Randomiza nuvem vermelha", 10.0, 10.0, 20, color::WHITE, Some(&nunito));
        print("2 - Randomiza nuvem verde", 10.0, 30.0, 20, color::WHITE, Some(&nunito));
        print("3 - Randomiza nuvem azul", 10.0, 50.0, 20, color::WHITE, Some(&nunito));
        print("4 - Randomiza nuvem rosa", 10.0, 70.0, 20, color::WHITE, Some(&nunito));

        next_frame().await;
    }
}
