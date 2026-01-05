use macroquad::color::Color;

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Circle, Line, OOBB},
};

/// Um trait que engloba caixas em geral (AABB e OOBB) e lida com suas colisões.
pub trait Collider {
    /// Desenha a caixa
    fn draw(&self, thickness: f32, color: Color);

    /// Retorna o centro do objeto
    fn center(&self) -> Vec2;

    /// Retorna os valores min,max da projeção do objeto sobre um eixo
    fn project(&self, axis: Vec2) -> (f64, f64);

    /// Retorna todas as bordas do objeto
    fn edges(&self) -> Vec<Line>;

    /// Retorna os eixos com que o objeto contribui para o cálculo.
    /// (O "other" é necessário para definir o eixo do círculo:
    /// o único eixo que importa seria o eixo do centro do círculo
    /// pro ponto mais próximo entre o círculo .)
    fn sat_axes(&self, other: &dyn Collider) -> Vec<Vec2>;
}

/// Checa se um objeto colide com o outro usando SAT
pub fn collides(a: &dyn Collider, b: &dyn Collider) -> bool {
    // Obtém os eixos necessários para o teste
    let mut axes = a.sat_axes(b);
    let bx = b.sat_axes(a)
        .into_iter()
        // Filtra os eixos paralelos
        .filter(|b_axis| !axes.iter().any(|a_axis| a_axis.is_parallel(*b_axis)))
        .collect::<Vec<Vec2>>();
    axes.extend(bx);

    // (SAT) Se algum eixo indica separação entre os objetos,
    // isso é suficiente para indicar que eles não estão colidindo.
    !axes.into_iter().any(|axis| {
        let (min_a, max_a) = a.project(axis);
        let (min_b, max_b) = b.project(axis);
        max_a < min_b || max_b < min_a
    })
}
