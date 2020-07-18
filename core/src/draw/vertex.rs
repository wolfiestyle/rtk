use crate::draw::{Color, TexCoord};
use crate::geometry::Point;

pub trait Vertex: Copy {
    fn new(pos: Point<f32>, color: Color, texc: TexCoord) -> Self;

    fn translate(self, offset: Point<f32>) -> Self;

    fn colored(pos: Point<f32>, color: Color) -> Self {
        Self::new(pos, color, TexCoord::TOP_LEFT)
    }

    fn textured(pos: Point<f32>, texc: TexCoord) -> Self {
        Self::new(pos, Color::WHITE, texc)
    }
}
