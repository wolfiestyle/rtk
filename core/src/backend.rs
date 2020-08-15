use crate::draw::{ColorOp, FillMode, TexCoord, TextSection, TextureId};
use crate::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use crate::geometry::{Point, Rect};
use crate::image::Image;
use std::fmt;
use std::ops::Add;

/// Resources provided by the backend.
pub trait Resources {
    /// Creates a texture with a specific id.
    ///
    /// If a texture already exists on that id, it will be replaced.
    fn load_texture(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError>;

    /// Creates a texture with a specific id, only if it isn't used.
    ///
    /// If a texture already exists on that id, it won't be replaced.
    fn load_texture_once(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError>;

    /// Deletes the texture on the specified id.
    fn delete_texture(&mut self, id: TextureId);

    /// Lists all the fonts installed on the system.
    ///
    /// The string returned is the family name of each font.
    fn enumerate_fonts(&self) -> Vec<String>;

    /// Selects a system font based on family names and properties.
    ///
    /// Family names are checked in order (to specify alternatives).
    /// Returns `None` if there is no font that matches any of the family names or properties.
    fn select_font(&self, family_names: &[FontFamily], properties: &FontProperties) -> Option<FontSource>;

    /// Loads the specified font.
    ///
    /// The result of this method is cached, so a single font is loaded only once.
    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError>;

    /// Creates a texture from an image.
    #[inline]
    fn create_texture(&mut self, image: &Image) -> Result<TextureId, TextureError> {
        let id = TextureId::new();
        self.load_texture(id, image).map(|_| id)
    }
}

/// Drawing interface implemented by the backend.
pub trait DrawBackend: Resources {
    type Vertex: Vertex;

    /// Draws triangles from vertices and indices.
    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>;

    /// Draws text.
    fn draw_text(&mut self, text: TextSection, viewport: Rect);

    /// Draws a rectangle.
    ///
    /// The default implementation splits the rect into two triangles, and
    /// assumes OpenGL-style fill rules (must cover the center of the pixel to
    /// generate a fragment).
    #[inline]
    fn draw_rect(&mut self, rect: Rect, fill: FillMode, viewport: Rect) {
        if rect.size.is_zero_area() || !rect.intersects(viewport) {
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

/// Error produced by texture operations.
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

/// Required trait bounds for `DrawBackend::Vertex`.
pub trait Vertex: Copy + From<(Point<f32>, ColorOp, TexCoord)> + Add<Point<f32>, Output = Self> {}

impl<T> Vertex for T where T: Copy + From<(Point<f32>, ColorOp, TexCoord)> + Add<Point<f32>, Output = T> {}
