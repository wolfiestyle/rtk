use crate::geometry::{Position, Rect};
use std::borrow::Cow;

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

/// Draw command text detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCmdText {
    pub text: Cow<'static, str>,
    pub font_desc: Cow<'static, str>,
    pub mode: TextDrawMode,
    pub viewport: Rect,
}

/// A single draw command.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Clear(Color),
    Primitives(DrawCmdPrim),
    Text(DrawCmdText),
}

/// Defines an object's alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Alignment {
    horizontal: HAlign,
    vertical: VAlign,
}

impl From<HAlign> for Alignment {
    #[inline]
    fn from(horizontal: HAlign) -> Self {
        Alignment {
            horizontal,
            vertical: Default::default(),
        }
    }
}

impl From<VAlign> for Alignment {
    #[inline]
    fn from(vertical: VAlign) -> Self {
        Alignment {
            horizontal: Default::default(),
            vertical,
        }
    }
}

/// Horizontal alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl Default for HAlign {
    #[inline]
    fn default() -> Self {
        HAlign::Left
    }
}

/// Vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

impl Default for VAlign {
    #[inline]
    fn default() -> Self {
        VAlign::Top
    }
}

/// Defines how text should be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDrawMode {
    /// Draw from the baseline at the specified position.
    Baseline(Position),
    /// Draw inside the specified rectangle.
    Bounded(Rect, Alignment),
}

impl From<Position> for TextDrawMode {
    #[inline]
    fn from(pos: Position) -> Self {
        TextDrawMode::Baseline(pos)
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
