use crate::draw::{Color, ColorOp, FillMode, TexCoord, TextDrawMode};
use crate::geometry::{Point, Rect};
use crate::image::Image;

/// Drawing interface implemented by the backend.
pub trait DrawBackend {
    type Vertex: Copy + From<(Point<f32>, ColorOp, TexCoord)>;

    fn clear(&mut self, color: Color, viewport: Rect);

    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, image: Option<&Image>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>;

    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect);

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
