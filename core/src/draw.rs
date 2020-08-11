//! Types used to communicate with the drawing backend.
use crate::image::Image;
use std::ops;

pub use glyph_brush::Layout as TextLayout;
pub use glyph_brush::OwnedSection as TextSection;
pub use glyph_brush::OwnedText as Text;

mod color;
pub use color::*;
mod texcoord;
pub use texcoord::*;
mod context;
pub use context::DrawContext;

/// Drawing fill mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillMode<'a> {
    Color(Color),
    Texture(&'a Image, TexRect),
    ColoredTexture(ColorOp, &'a Image, TexRect),
}

impl FillMode<'_> {
    #[inline]
    pub fn color(&self) -> ColorOp {
        match self {
            FillMode::Color(color) => color.clone().into(),
            FillMode::ColoredTexture(color, _, _) => *color,
            _ => Default::default(),
        }
    }

    #[inline]
    pub fn texture(&self) -> Option<&Image> {
        match self {
            FillMode::Texture(img, _) | FillMode::ColoredTexture(_, img, _) => Some(img),
            _ => None,
        }
    }

    #[inline]
    pub fn texrect(&self) -> TexRect {
        match self {
            FillMode::Texture(_, texr) | FillMode::ColoredTexture(_, _, texr) => *texr,
            _ => Default::default(),
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
        FillMode::Texture(img, Default::default())
    }
}

impl<'a> From<(&'a Image, TexRect)> for FillMode<'a> {
    #[inline]
    fn from((img, texr): (&'a Image, TexRect)) -> Self {
        FillMode::Texture(img, texr)
    }
}

impl<'a> ops::Mul<Color> for FillMode<'a> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        match self {
            FillMode::Color(color) => FillMode::Color(color * rhs),
            FillMode::Texture(img, texr) => FillMode::ColoredTexture(ColorOp::mul(rhs), img, texr),
            FillMode::ColoredTexture(op, img, texr) => FillMode::ColoredTexture(op * rhs, img, texr),
        }
    }
}

impl<'a> ops::Add<Color> for FillMode<'a> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Color) -> Self::Output {
        match self {
            FillMode::Color(color) => FillMode::Color(color + rhs),
            FillMode::Texture(img, texr) => FillMode::ColoredTexture(ColorOp::add(rhs), img, texr),
            FillMode::ColoredTexture(op, img, texr) => FillMode::ColoredTexture(op + rhs, img, texr),
        }
    }
}
