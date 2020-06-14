use crate::geometry::Rect;

mod color;
pub use color::Color;
mod vertex;
pub use vertex::Vertex;
mod queue;
pub use queue::*;
mod context;
pub use context::DrawContext;
mod image;
pub use self::image::*;

/// Texture coordinates (in [0, 1] range).
pub type TexCoord = [f32; 2];

/// Types of drawing primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCmdPrim {
    pub primitive: Primitive,
    pub idx_offset: usize, // the draw command references the indices on a shared vertex buffer
    pub idx_len: usize,
    pub texture: Option<ImageRef>,
    pub viewport: Rect,
}

/// A single draw command.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Clear(Color),
    Primitives(DrawCmdPrim),
}
