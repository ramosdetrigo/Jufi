use macroquad::color::Color;

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Circle, OOBB},
};

pub trait SATCollider {
    fn draw(&self, thickness: f32, color: Color);

    fn u(&self) -> Vec2;

    fn v(&self) -> Vec2;

    fn center(&self) -> Vec2;

    fn extents(&self) -> Vec2;

    fn collides_with(&self, other: &dyn SATCollider) -> bool {
        // Eixos de teste: u e v de ambas as caixas
        let axes = [self.u(), self.v(), other.u(), other.v()];

        // Vetor entre os centros
        let v_centros = other.center() - self.center();

        // Realiza o teste para cada eixo necessário
        for axis in axes {
            // Projeções dos objetos no eixo
            let r_self = self.extents().x * axis.dot(self.u()).abs()
                + self.extents().y * axis.dot(self.v()).abs();
            let r_other = other.extents().x * axis.dot(other.u()).abs()
                + other.extents().y * axis.dot(other.v()).abs();

            // Distância entre centros projetada no eixo
            let dist = v_centros.dot(axis).abs();

            // (SAT) Para indicar sobreposição, a distância entre os centros projetada
            // no eixo deve ser menor que a soma dos "raios" projetados neste eixo
            if dist > r_self + r_other {
                return false;
            }
        }

        true
    }
}
