use crate::draw::{Color, TexCoord};
use crate::geometry::Pointf;

/// A single vertex for drawing operations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vertex {
    pub pos: Pointf,
    pub color: Color,
    pub texc: TexCoord,
}

impl Vertex {
    #[inline]
    pub const fn new(pos: Pointf, color: Color, texc: TexCoord) -> Self {
        Vertex { pos, color, texc }
    }

    #[inline]
    pub const fn colored(pos: Pointf, color: Color) -> Self {
        Vertex {
            pos,
            color,
            texc: [0.0, 0.0],
        }
    }

    #[inline]
    pub const fn textured(pos: Pointf, texc: TexCoord) -> Self {
        Vertex {
            pos,
            color: Color::WHITE,
            texc,
        }
    }

    #[inline]
    pub fn translate(self, offset: Pointf) -> Self {
        Vertex {
            pos: self.pos + offset,
            color: self.color,
            texc: self.texc,
        }
    }
}
