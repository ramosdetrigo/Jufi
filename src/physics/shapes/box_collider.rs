use macroquad::color::Color;

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Circle, OOBB},
};

/// Um trait que engloba caixas em geral (AABB e OOBB) e lida com suas colisões.
pub trait BoxCollider {
    /// Desenha a caixa
    fn draw(&self, thickness: f32, color: Color);

    /// Retorna o eixo u da caixa
    fn u(&self) -> Vec2;

    /// Retorna o eixo v da caixa
    fn v(&self) -> Vec2;

    /// Retorna o centro da caixa
    fn center(&self) -> Vec2;

    /// Retorna os extents (metade da largura e altura) da caixa
    fn extents(&self) -> Vec2;

    /// Checa se uma caixa colide com a outra usando SAT
    fn collides_with_box(&self, other: &dyn BoxCollider) -> bool {
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

        // Se nenhum teste retornou falso, as caixas estão colidindo.
        true
    }

    /// Checa se a caixa colide com um círculo
    fn collides_with_circle(&self, other: &Circle) -> bool {
        let s_extents = self.extents();
        
        // A ideia aqui é projetar o círculo pro espaço local da caixa
        let d = other.center - self.center();
        let local_x = d.dot(self.u());
        let local_y = d.dot(self.v());

        // E depois fazer o teste de colisão círculo - AABB normalmente
        let closest_x = local_x.clamp(-s_extents.x, s_extents.x);
        let closest_y = local_y.clamp(-s_extents.y, s_extents.y);

        // A distância do círculo pro ponto mais próximo dele com a caixa
        // deve ser menor que o seu raio para indicar colisão.
        let dx = local_x - closest_x;
        let dy = local_y - closest_y;
        dx * dx + dy * dy < other.radius * other.radius
    }
}
