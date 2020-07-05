//! Types used to communicate with the drawing backend.
use crate::geometry::{Alignment, HAlign, Position, Rect, VAlign};
use crate::image::ImageRef;
use std::borrow::Cow;

mod color;
pub use color::Color;
mod vertex;
pub use vertex::Vertex;
mod texcoord;
pub use texcoord::TexCoord;
mod queue;
pub use queue::*;
mod context;
pub use context::DrawContext;

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

/// Primitive draw command detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCmdPrim {
    /// The primitive to draw.
    pub primitive: Primitive,
    /// Offset inside the shared index buffer on the draw queue.
    pub idx_offset: usize,
    /// Length of the indices slice.
    pub idx_len: usize,
    /// Image to use for this draw command.
    pub texture: Option<ImageRef>,
    /// Clipping viewport.
    pub viewport: Rect,
}

/// Text draw command detail.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawCmdText {
    pub text: Cow<'static, str>,
    pub font_desc: Cow<'static, str>,
    pub mode: TextDrawMode,
    pub color: Color,
    pub viewport: Rect,
}

/// A single draw command.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Clear(Color, Rect),
    Primitives(DrawCmdPrim),
    Text(DrawCmdText),
}

/// Defines how text should be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDrawMode {
    /// Draw from the baseline at the specified position.
    Baseline(Position),
    /// Draw inside the specified rectangle.
    Bounded(Rect, Alignment),
}

impl TextDrawMode {
    pub fn offset(self, offset: Position) -> Self {
        match self {
            TextDrawMode::Baseline(pos) => TextDrawMode::Baseline(pos + offset),
            TextDrawMode::Bounded(rect, align) => TextDrawMode::Bounded(rect.offset(offset), align),
        }
    }
}

impl From<Position> for TextDrawMode {
    #[inline]
    fn from(pos: Position) -> Self {
        TextDrawMode::Baseline(pos)
    }
}

impl From<[i32; 2]> for TextDrawMode {
    #[inline]
    fn from([x, y]: [i32; 2]) -> Self {
        TextDrawMode::Baseline(Position { x, y })
    }
}

impl From<Rect> for TextDrawMode {
    #[inline]
    fn from(bounds: Rect) -> Self {
        TextDrawMode::Bounded(bounds, Default::default())
    }
}

impl From<(Rect, Alignment)> for TextDrawMode {
    #[inline]
    fn from((bounds, align): (Rect, Alignment)) -> Self {
        TextDrawMode::Bounded(bounds, align)
    }
}

impl From<(Rect, HAlign)> for TextDrawMode {
    #[inline]
    fn from((bounds, halign): (Rect, HAlign)) -> Self {
        TextDrawMode::Bounded(bounds, halign.into())
    }
}

impl From<(Rect, VAlign)> for TextDrawMode {
    #[inline]
    fn from((bounds, valign): (Rect, VAlign)) -> Self {
        TextDrawMode::Bounded(bounds, valign.into())
    }
}
