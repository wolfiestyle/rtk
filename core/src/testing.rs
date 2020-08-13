use crate::backend::{BackendResources, TextureError};
use crate::draw::TextureId;
use crate::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use crate::image::Image;

/// Null resources implementation. Useful for testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NullResources;

impl BackendResources for NullResources {
    #[inline]
    fn load_texture(&mut self, _id: TextureId, _image: &Image) -> Result<(), TextureError> {
        Err(TextureError::FormatNotSupported)
    }

    #[inline]
    fn load_texture_once(&mut self, _id: TextureId, _image: &Image) -> Result<(), TextureError> {
        Err(TextureError::FormatNotSupported)
    }

    #[inline]
    fn delete_texture(&mut self, _id: TextureId) {}

    #[inline]
    fn enumerate_fonts(&self) -> Vec<String> {
        vec![]
    }

    #[inline]
    fn select_font(&self, _family_names: &[FontFamily], _properties: &FontProperties) -> Option<FontSource> {
        None
    }

    #[inline]
    fn load_font(&mut self, _font_src: &FontSource) -> Result<FontId, FontLoadError> {
        Err(FontLoadError::InvalidData)
    }
}
