use widgets::draw::{Color, TexCoord};
use widgets::geometry::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    texc: [f32; 2],
}

glium::implement_vertex!(Vertex, pos, color, texc);

impl Vertex {
    #[inline]
    pub fn new(pos: Point<f32>, color: Color, texc: TexCoord) -> Self {
        Self {
            pos: pos.into(),
            color: color.into(),
            texc: texc.into(),
        }
    }
}

impl From<(Point<f32>, Color, TexCoord)> for Vertex {
    #[inline]
    fn from((pos, color, texc): (Point<f32>, Color, TexCoord)) -> Self {
        Self::new(pos, color, texc)
    }
}
