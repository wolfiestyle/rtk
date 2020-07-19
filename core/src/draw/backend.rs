use crate::draw::{Color, Primitive, TextDrawMode, Vertex};
use crate::geometry::Rect;
use crate::image::ImageRef;

/// Drawing interface with the backend.
pub trait DrawBackend {
    type Vertex: Vertex;

    fn clear(&mut self, color: Color, viewport: Rect);
    fn draw_prim(&mut self, primitive: Primitive, vertices: &[Self::Vertex], indices: &[u32], texture: Option<ImageRef>, viewport: Rect);
    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect);
}
