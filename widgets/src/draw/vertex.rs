use crate::draw::{Color, TexCoord};
use crate::geometry::Point;

/// A single vertex for drawing operations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vertex {
    pub pos: Point<f32>,
    pub color: Color,
    pub texc: TexCoord,
}

impl Vertex {
    #[inline]
    pub const fn new(pos: Point<f32>, color: Color, texc: TexCoord) -> Self {
        Vertex { pos, color, texc }
    }

    #[inline]
    pub const fn colored(pos: Point<f32>, color: Color) -> Self {
        Vertex {
            pos,
            color,
            texc: TexCoord::TOP_LEFT,
        }
    }

    #[inline]
    pub const fn textured(pos: Point<f32>, texc: TexCoord) -> Self {
        Vertex {
            pos,
            color: Color::WHITE,
            texc,
        }
    }

    #[inline]
    pub fn translate(self, offset: Point<f32>) -> Self {
        Vertex {
            pos: self.pos + offset,
            color: self.color,
            texc: self.texc,
        }
    }
}
