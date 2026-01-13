use macroquad::color::Color;

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Circle, Line, OOBB},
};

/// Um trait que engloba objetos em geral e lida com suas colisões.
pub trait Collider {
    /// Move o centro do objeto para uma posição específica
    fn set_center(&mut self, pos: Vec2);

    /// Rotaciona o objeto por um ângulo theta (só funciona para OOBB no código atual)
    fn rotate(&mut self, theta: f64);

    /// Aumenta/diminui o tamanho do objeto
    fn grow(&mut self, width: f64, height: f64);

    /// Retorna a largura e altura do objeto
    fn size(&self) -> Vec2;

    /// Desenha o objeto
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
    /// pro ponto mais próximo entre o círculo.)
    fn sat_axes(&self, other: &dyn Collider) -> Vec<Vec2>;
}

/// Checa se um objeto colide com o outro usando SAT
pub fn collides(a: &dyn Collider, b: &dyn Collider) -> bool {
    // Obtém os eixos necessários para o teste
    let ax = a.sat_axes(b);
    let bx = b.sat_axes(a);
    let axes = ax.iter().chain(
        // Filtra os eixos paralelos
        bx.iter().filter(|b_axis| !ax.iter().any(|a_axis| a_axis.is_parallel(**b_axis))),
    );

    // (SAT) Se algum eixo indica separação entre os objetos,
    // isso é suficiente para indicar que eles não estão colidindo.
    !axes.into_iter().any(|axis| {
        let (min_a, max_a) = a.project(*axis);
        let (min_b, max_b) = b.project(*axis);
        max_a < min_b || max_b < min_a
    })
}
