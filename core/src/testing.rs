use crate::backend::{DrawBackend, Resources, TextureError};
use crate::draw::{Color, ColorOp, TexCoord, TextSection, TextureId};
use crate::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use crate::geometry::{Point, Rect};
use crate::image::Image;
use std::collections::HashMap;

/// Test backend implementation.
///
/// It does nothing but storing the values it receives.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestBackend {
    pub textures: HashMap<TextureId, Image>,
    pub fonts: Vec<FontSource>,
    pub draw_cmd: Vec<TestDrawCmd>,
}

impl Resources for TestBackend {
    fn load_texture(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError> {
        self.textures.insert(id, image.clone());
        Ok(())
    }

    fn load_texture_once(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError> {
        self.textures.entry(id).or_insert_with(|| image.clone());
        Ok(())
    }

    fn delete_texture(&mut self, id: TextureId) {
        self.textures.remove(&id);
    }

    fn enumerate_fonts(&self) -> Vec<String> {
        unimplemented!()
    }

    fn select_font(&self, _family_names: &[FontFamily], _properties: &FontProperties) -> Option<FontSource> {
        unimplemented!()
    }

    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError> {
        let id = self.fonts.len();
        self.fonts.push(font_src.clone());
        Ok(FontId(id))
    }
}

impl DrawBackend for TestBackend {
    type Vertex = TestVertex;

    fn clear(&mut self, color: Color, viewport: Rect) {
        self.draw_cmd.push(TestDrawCmd::Clear { color, viewport });
    }

    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>,
    {
        self.draw_cmd.push(TestDrawCmd::Triangles {
            vertices: vertices.into_iter().collect(),
            indices: indices.into_iter().collect(),
            texture,
            viewport,
        })
    }

    fn draw_text(&mut self, text: TextSection, viewport: Rect) {
        self.draw_cmd.push(TestDrawCmd::Text { text, viewport })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TestVertex {
    pub pos: Point<f32>,
    pub color: ColorOp,
    pub texc: TexCoord,
}

impl From<(Point<f32>, ColorOp, TexCoord)> for TestVertex {
    fn from((pos, color, texc): (Point<f32>, ColorOp, TexCoord)) -> Self {
        TestVertex { pos, color, texc }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestDrawCmd {
    Clear {
        color: Color,
        viewport: Rect,
    },
    Triangles {
        vertices: Vec<TestVertex>,
        indices: Vec<u32>,
        texture: Option<TextureId>,
        viewport: Rect,
    },
    Text {
        text: TextSection,
        viewport: Rect,
    },
}
