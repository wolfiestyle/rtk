use crate::draw::{Color, ColorOp, TexRect, TextureId};
use std::ops;

/// Drawing fill mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillMode {
    Color(Color),
    Texture(TextureId, TexRect),
    ColoredTexture(ColorOp, TextureId, TexRect),
}

impl FillMode {
    #[inline]
    pub fn color(&self) -> ColorOp {
        match self {
            FillMode::Color(color) => color.clone().into(),
            FillMode::ColoredTexture(color, _, _) => *color,
            _ => Default::default(),
        }
    }

    #[inline]
    pub fn texture(&self) -> Option<TextureId> {
        match self {
            FillMode::Texture(tex, _) | FillMode::ColoredTexture(_, tex, _) => Some(*tex),
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

impl From<Color> for FillMode {
    #[inline]
    fn from(color: Color) -> Self {
        FillMode::Color(color)
    }
}

impl From<TextureId> for FillMode {
    #[inline]
    fn from(tex: TextureId) -> Self {
        FillMode::Texture(tex, Default::default())
    }
}

impl From<(TextureId, TexRect)> for FillMode {
    #[inline]
    fn from((tex, texr): (TextureId, TexRect)) -> Self {
        FillMode::Texture(tex, texr)
    }
}

impl ops::Mul<Color> for FillMode {
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

impl ops::Add<Color> for FillMode {
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
