use macroquad::color::Color;

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Circle, OOBB},
};

/// Um trait que engloba caixas em geral (AABB e OOBB) e lida com suas colisões.
pub trait Collider {
    /// Desenha a caixa
    fn draw(&self, thickness: f32, color: Color);

    /// Retorna o centro do objeto
    fn center(&self) -> Vec2;

    /// Retorna os valores min,max da projeção do objeto sobre um eixo
    fn project(&self, axis: Vec2) -> (f64, f64);

    /// Retorna os eixos com que o objeto contribui para o cálculo.
    fn axes(&self, other: &dyn Collider) -> Vec<Vec2>;
}

/// Checa se uma caixa colide com a outra usando SAT
pub fn collides(a: &dyn Collider, b: &dyn Collider) -> bool {
    // Obtém os eixos necessários para o teste
    let mut axes = a.axes(b);
    axes.extend(b.axes(a));

    // Realiza o teste para cada eixo necessário
    for axis in axes {
        let (min_a, max_a) = a.project(axis);
        let (min_b, max_b) = b.project(axis);

        // (SAT) Se algum eixo indica separação entre os objetos,
        // isso é suficiente para indicar que eles não estão colidindo.
        if max_a < min_b || max_b < min_a {
            return false;
        }
    }

    // Se nenhum teste retornou falso, as caixas estão colidindo.
    true
}
