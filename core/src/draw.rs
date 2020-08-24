//! Types used to communicate with the drawing backend.
mod color;
mod context;
mod fillmode;
mod texcoord;
mod text;
pub use color::*;
pub use context::*;
pub use fillmode::*;
pub use texcoord::*;
pub use text::*;

use std::ops;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Unique texture id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureId(usize);

static TEXTURE_ID: AtomicUsize = AtomicUsize::new(1);

impl TextureId {
    pub fn new() -> Self {
        let id = TEXTURE_ID.fetch_add(1, Ordering::Relaxed);
        Self(id)
    }
}

impl ops::Mul<Color> for TextureId {
    type Output = FillMode;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        FillMode::ColoredTexture(ColorOp::mul(rhs), self, Default::default())
    }
}

impl ops::Add<Color> for TextureId {
    type Output = FillMode;

    #[inline]
    fn add(self, rhs: Color) -> Self::Output {
        FillMode::ColoredTexture(ColorOp::add(rhs), self, Default::default())
    }
}
