//! Types used to communicate with the drawing backend.
use crate::geometry::{Alignment, HAlign, Position, Rect, VAlign};
use crate::image::Image;

mod color;
pub use color::Color;
mod texcoord;
pub use texcoord::*;
mod context;
pub use context::DrawContext;
mod backend;
pub use backend::*;

/// Drawing fill mode.
#[derive(Debug, Clone, PartialEq)]
pub enum FillMode<'a> {
    Color(Color),
    Texture(&'a Image),
    ColoredTexture(&'a Image, Color),
}

impl FillMode<'_> {
    #[inline]
    pub fn color(&self) -> Color {
        match self {
            FillMode::Color(color) | FillMode::ColoredTexture(_, color) => *color,
            _ => Color::WHITE,
        }
    }

    #[inline]
    pub fn texture(&self) -> Option<&Image> {
        match self {
            FillMode::Texture(img) | FillMode::ColoredTexture(img, _) => Some(img),
            _ => None,
        }
    }
}

impl From<Color> for FillMode<'_> {
    #[inline]
    fn from(color: Color) -> Self {
        FillMode::Color(color)
    }
}

impl<'a> From<&'a Image> for FillMode<'a> {
    #[inline]
    fn from(img: &'a Image) -> Self {
        FillMode::Texture(img)
    }
}

impl<'a> From<(Color, &'a Image)> for FillMode<'a> {
    #[inline]
    fn from((color, img): (Color, &'a Image)) -> Self {
        FillMode::ColoredTexture(img, color)
    }
}

impl<'a> From<(&'a Image, Color)> for FillMode<'a> {
    #[inline]
    fn from((img, color): (&'a Image, Color)) -> Self {
        FillMode::ColoredTexture(img, color)
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
