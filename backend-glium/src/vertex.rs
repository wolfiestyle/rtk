use widgets::draw::{ColorOp, TexCoord};
use widgets::geometry::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: [f32; 2],
    color_mul: [f32; 4],
    color_add: [f32; 4],
    texc: [f32; 2],
}

glium::implement_vertex!(Vertex, pos, color_mul, color_add, texc);

impl Vertex {
    #[inline]
    pub fn new(pos: Point<f32>, color: ColorOp, texc: TexCoord) -> Self {
        Self {
            pos: pos.into(),
            color_mul: color.mul.into(),
            color_add: color.add.into(),
            texc: texc.into(),
        }
    }
}

impl From<(Point<f32>, ColorOp, TexCoord)> for Vertex {
    #[inline]
    fn from((pos, color, texc): (Point<f32>, ColorOp, TexCoord)) -> Self {
        Self::new(pos, color, texc)
    }
}
