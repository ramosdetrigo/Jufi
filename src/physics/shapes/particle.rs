use crate::{algebra::Vec2, physics::shapes::Line};
use macroquad::{
    color::Color,
    shapes::{draw_circle, draw_line},
};

#[derive(Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub speed: Vec2,
}

impl Particle {
    #[inline]
    #[must_use]
    /// Construtor da partícula
    pub fn new(position: Vec2, speed: Vec2) -> Particle {
        Particle { position, speed }
    }

    /// Atualiza a posição da partícula conforme variação de tempo e paredes do mundo
    pub fn update(&mut self, delta: f64, walls: &Vec<Line>) {
        let next_pos = self.position + self.speed * delta;
        let movement_line = Line::new(self.position, next_pos);

        // Obtém a interseção mais próxima da linha do movimento da partícula
        let closest_intersection = walls
            .iter()
            // Calcula todas as interseções e filtra aquelas que são válidas (!= None) com 0.0 <= t <= 1.0
            .filter_map(|wall| {
                movement_line.intersection(*wall).filter(|intersection| {
                    0.0 <= intersection.t && intersection.t <= 1.0
                    && 0.0 <= intersection.u && intersection.u <= 1.0
                })
            })
            .min_by(|a, b| {
                // Pega a interseção mais próxima da posição atual da partícula
                a.p.distance_to_squared(self.position)
                    .total_cmp(&b.p.distance_to_squared(self.position))
            });

        // Se houve interseção, "quica" a partícula na parede
        if let Some(intersection) = closest_intersection {
            // Movimento total a ser feito
            let total_movement = (self.speed * delta).length();
            // Movimento feito até a colisão com a parede
            let partial_movement = self.position.distance_to(intersection.p);
            // Movimento restante a ser feito
            let remaining_movement = total_movement - partial_movement;

            // Reflete a velocidade na normal da parede.
            self.speed = self.speed.bounce(intersection.normal);
            self.position = intersection.p + self.speed.normalized() * remaining_movement;
        } else {
            self.position = next_pos
        }
    }

    pub fn draw(&self, color: Color) {
        draw_circle(self.position.x as f32, self.position.y as f32, 3.0, color);
    }

    pub fn draw_movement_line(&self, delta: f64, thickness: f32, color: Color) {
        let next_pos = self.position + self.speed * delta;
        draw_line(
            self.position.x as f32,
            self.position.y as f32,
            next_pos.x as f32,
            next_pos.y as f32,
            thickness,
            color,
        );
    }
}
