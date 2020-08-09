use widgets::draw::{ColorOp, TexCoord};
use widgets::geometry::Point;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pos: [f32; 2],
    color_mul: [u16; 4],
    color_add: [u16; 4],
    texc: [f32; 2],
}

glium::implement_vertex! {
    Vertex,
    pos normalize(false),
    color_mul normalize(true),
    color_add normalize(true),
    texc normalize(false)
}

impl From<(Point<f32>, ColorOp, TexCoord)> for Vertex {
    #[inline]
    fn from((pos, color, texc): (Point<f32>, ColorOp, TexCoord)) -> Self {
        Self {
            pos: pos.into(),
            color_mul: color.mul.into_rgb16(),
            color_add: color.add.into_rgb16(),
            texc: texc.into(),
        }
    }
}
