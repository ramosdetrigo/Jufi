use jufi::{algebra::Vec2, physics::shapes::Line, utils::print};
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

    // Linhas para desenhar os vetores
    // (o que importa é o p2. o p1 é sempre 0,0)
    let mut line1 = Line::new(Vec2::new(0.0, 0.0), Vec2::new(200.0, 0.0));
    let mut line2 = Line::new(Vec2::new(0.0, 0.0), Vec2::new(200.0, 0.0));
    // Linhas auxiliares para mostrar o ângulo no círculo
    let mut line3 = line1.clone();
    let mut line4 = line1.clone();

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        // Flipa na vertical para lidar com o eixo y ser positivo pra baixo
        let mouse_pos = Vec2::new(mx as f64, -my as f64) - Vec2::new(400.0, -300.0);
        let _delta = get_frame_time();

        // Câmera para centralizar no 0,0 e flipar o eixo y
        set_camera(&Camera2D {
            zoom: vec2(2.0 / screen_width(), -2.0 / screen_height()),
            ..Default::default()
        });

        // Atualiza e desenha os vetores
        line1.p2 = mouse_pos;
        if is_mouse_button_pressed(MouseButton::Left) {
            line2.p2 = mouse_pos;
        }
        if line1.is_degenerate() {
            line3.p2 = line1.p2;
        } else {
            line3.p2 = line1.p2.normalized() * 200.0
        }
        if line2.is_degenerate() {
            line4.p2 = line2.p2;
        } else {
            line4.p2 = line2.p2.normalized() * 200.0
        }

        line3.draw(2.0, color::WHITE.with_alpha(0.2));
        line4.draw(2.0, color::WHITE.with_alpha(0.2));
        line1.draw(2.0, color::SKYBLUE);
        line2.draw(2.0, color::ORANGE);

        // Desenha o círculo unitário e um arco mostrando o ângulo atual do vetor do mouse
        draw_circle_lines(0.0, 0.0, 200.0, 2.0, color::WHITE);
        let vec1_angle = line1.p2.angle_between(Vec2::X).to_degrees();
        // desenha o arco em outra posição dependendo da direção do vetor
        if line1.p2.cross(Vec2::X) > 0.0 {
            draw_arc(0.0, 0.0, 64, 200.0, -vec1_angle as f32, 5.0, vec1_angle as f32, color::BLUE);
        } else {
            draw_arc(0.0, 0.0, 64, 200.0, 0.0, 5.0, vec1_angle as f32, color::BLUE);
        }

        // Desenha o retângulo de pseudoângulos e o "arco" do vetor 1 ao seu redor
        draw_rectangle_lines(-200.0, -200.0, 400.0, 400.0, 2.0, color::WHITE);
        // Pseudoângulo do vetor do mouse (azul) no quadrado
        let a_vec1_pseudo_sqr = if line1.is_degenerate() {
            0.0
        } else {
            line1.p2.square_pseudoangle()
        };
        draw_pseudoangle_arc(a_vec1_pseudo_sqr);


        // Ângulo entre vetores via produto escalar
        let a_vecs_dot = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.angle_between(line2.p2).to_degrees()
        };

        // Ângulo entre vetores via produto vetorial
        let a_vecs_cross = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.angle_between_cross(line2.p2).to_degrees()
        };

        // Pseudoângulo do cosseno entre vetores
        let a_vecs_pseudo_cos = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.cos_pseudoangle_between(line2.p2)
        };

        // Pseudoângulo entre vetores no quadrado
        let a_vecs_pseudo_sqr = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.square_pseudoangle_between(line2.p2)
        };

        set_default_camera();


        print(format!("Ângulo (via produto escalar): {a_vecs_dot}").as_str(), 10.0, 10.0, 16, color::WHITE, Some(&nunito));
        print(format!("Ângulo (via produto vetorial): {a_vecs_cross}").as_str(), 10.0, 26.0, 16, color::WHITE, Some(&nunito));
        print(format!("Pseudoângulo do cosseno: {a_vecs_pseudo_cos}").as_str(), 10.0, 42.0, 16, color::WHITE, Some(&nunito));
        print(format!("Pseudoângulo no quadrado: {a_vec1_pseudo_sqr}").as_str(), 10.0, 58.0, 16, color::WHITE, Some(&nunito));
        print(format!("Pseudoângulo entre os vetores no quadrado: {a_vecs_pseudo_sqr}").as_str(), 10.0, 74.0, 16, color::WHITE, Some(&nunito));

        next_frame().await
    }
}


fn draw_pseudoangle_arc(progress: f64) {
    let mut p = progress as f32;

    // Primeira linha -> direita-topo
    let y1 = (p * 200.0).min(200.0);
    p -= y1 / 200.0;
    draw_line(200.0, 0.0, 200.0, y1, 5.0, color::GREEN);

    if p <= 0.0 { return }

    // Segunda linha -> topo
    let x1 = (p * 200.0).min(400.0);
    p -= x1 / 200.0;
    draw_line(200.0, 200.0, 200.0 - x1, 200.0, 5.0, color::GREEN);

    if p <= 0.0 { return }

    // Terceira linha -> esquerda
    let y2 = (p * 200.0).min(400.0);
    p -= y2 / 200.0;
    draw_line(-200.0, 200.0, -200.0, 200.0 - y2, 5.0, color::GREEN);

    if p <= 0.0 { return }

    // Quarta linha -> baixo
    let x2 = (p * 200.0).min(400.0);
    p -= x2 / 200.0;
    draw_line(-200.0, -200.0, -200.0 + x2, -200.0, 5.0, color::GREEN);

    if p <= 0.0 { return }

    // Quinta linha -> direita-baixo
    let y3 = (p * 200.0).min(200.0);
    draw_line(200.0, -200.0, 200.0, -200.0 + y3, 5.0, color::GREEN);
}
