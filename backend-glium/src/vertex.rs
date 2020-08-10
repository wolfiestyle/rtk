use glyph_brush::GlyphVertex;
use widgets::draw::{Color, ColorOp, TexCoord};
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TextVertex {
    rect: [f32; 4],
    texr: [f32; 4],
    color: [u16; 4],
}

glium::implement_vertex! {
    TextVertex,
    rect normalize(false),
    texr normalize(false),
    color normalize(true)
}

impl From<GlyphVertex<'_>> for TextVertex {
    #[inline]
    fn from(vert: GlyphVertex) -> Self {
        use glyph_brush::ab_glyph::Point;

        let Point { x: x0, y: y0 } = vert.pixel_coords.min;
        let Point { x: x1, y: y1 } = vert.pixel_coords.max;
        let Point { x: u0, y: v0 } = vert.tex_coords.min;
        let Point { x: u1, y: v1 } = vert.tex_coords.max;

        Self {
            rect: [x0, y0, x1, y1],
            texr: [u0, v0, u1, v1],
            color: Color::from(vert.extra.color).into_rgb16(),
        }
    }
}
