use crate::draw::{Color, ColorOp, FillMode, TexCoord, TextSection, TextureId};
use crate::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use crate::geometry::{Point, Rect};
use crate::image::Image;
use std::fmt;

/// Resources provided by the backend.
pub trait BackendResources {
    fn load_texture(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError>;

    fn load_texture_once(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError>;

    fn delete_texture(&mut self, id: TextureId);

    fn enumerate_fonts(&self) -> Vec<String>;

    fn select_font(&self, family_names: &[FontFamily], properties: &FontProperties) -> Option<FontSource>;

    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError>;

    #[inline]
    fn create_texture(&mut self, image: &Image) -> Result<TextureId, TextureError> {
        let id = TextureId::new();
        self.load_texture(id, image)?;
        Ok(id)
    }
}

/// Drawing interface implemented by the backend.
pub trait DrawBackend {
    type Vertex: Copy + From<(Point<f32>, ColorOp, TexCoord)>;

    fn clear(&mut self, color: Color, viewport: Rect);

    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>;

    fn draw_text(&mut self, text: TextSection, viewport: Rect);

    #[inline]
    fn draw_rect(&mut self, rect: Rect, fill: FillMode, viewport: Rect) {
        if rect.size.is_zero_area() {
            return;
        }
        let top_left = rect.pos.cast();
        let bot_right = top_left + rect.size.as_point();
        let top_right = top_left.with_x(bot_right.x);
        let bot_left = bot_right.with_x(top_left.x);
        let color = fill.color();
        let texr = fill.texrect();
        let verts = [
            (top_left, color, texr.top_left()).into(),
            (top_right, color, texr.top_right()).into(),
            (bot_right, color, texr.bot_right()).into(),
            (bot_left, color, texr.bot_left()).into(),
        ];
        let indices = [0, 1, 2, 2, 3, 0];
        self.draw_triangles(verts.iter().copied(), indices.iter().copied(), fill.texture(), viewport)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureError {
    FormatNotSupported,
    DimensionsNotSupported,
    TypeNotSupported,
}

impl fmt::Display for TextureError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::TextureError::*;
        let desc = match *self {
            FormatNotSupported => "The requested format is not supported by the backend",
            DimensionsNotSupported => "The requested texture dimensions are not supported",
            TypeNotSupported => "The texture format is not supported by the backend",
        };
        fmt.write_str(desc)
    }
}
