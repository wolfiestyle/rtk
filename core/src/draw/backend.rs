use crate::draw::{Color, FillMode, TexCoord, TexRect, TextDrawMode};
use crate::geometry::{Point, Rect};

/// Drawing interface implemented by the backend.
pub trait DrawBackend {
    fn clear(&mut self, color: Color, viewport: Rect);

    fn draw_point(&mut self, pos: Point<f32>, texc: TexCoord, fill: FillMode, viewport: Rect);

    fn draw_line(&mut self, pos: [Point<f32>; 2], texc: [TexCoord; 2], fill: FillMode, viewport: Rect);

    fn draw_triangle(&mut self, pos: [Point<f32>; 3], texc: [TexCoord; 3], fill: FillMode, viewport: Rect);

    fn draw_rect(&mut self, rect: Rect, texr: TexRect, fill: FillMode, viewport: Rect);

    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect);
}
