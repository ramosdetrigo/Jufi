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
    /// (O "other" é necessário para definir o eixo do círculo:
    /// o único eixo que importa seria o eixo do centro do círculo
    /// pro centro do outro objeto checado.)
    fn axes(&self) -> Vec<Vec2>;

    /// Checa se um objeto colide com o outro usando SAT
    fn collides(&self, b: &dyn Collider) -> bool {
        let a = self;

        // Obtém os eixos necessários para o teste
        let mut axes = a.axes();
        if axes.is_empty() {
            axes = center_axis(a.center(), b.center())
        }

        let mut bx = b.axes();
        if bx.is_empty() {
            bx = center_axis(a.center(), b.center())
        }

        // Filtra os eixos paralelos
        for b in bx {
            if !axes.iter().any(|a| a.is_parallel(b)) {
                axes.push(b);
            }
        }

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

        // Se nenhum teste retornou falso, os objetos estão colidindo.
        true
    }
}

fn center_axis(c1: Vec2, c2: Vec2) -> Vec<Vec2> {
    vec![(c2 - c1).normalized()]
}
