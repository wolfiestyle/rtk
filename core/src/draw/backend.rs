use crate::draw::{Color, FillMode, TexCoord, TexRect, TextDrawMode};
use crate::geometry::{Point, Rect};

/// Drawing interface implemented by the backend.
pub trait DrawBackend {
    fn clear(&mut self, color: Color, viewport: Rect);

    fn draw_point(&mut self, pos: Point<f32>, texc: TexCoord, fill: FillMode, viewport: Rect);

    fn draw_line(&mut self, pos: [Point<f32>; 2], texc: [TexCoord; 2], fill: FillMode, viewport: Rect);

    fn draw_triangle(&mut self, pos: [Point<f32>; 3], texc: [TexCoord; 3], fill: FillMode, viewport: Rect);

    fn draw_rect(&mut self, rect: Rect, texr: TexRect, fill: FillMode, viewport: Rect) {
        if let Some([top_left, top_right, bot_right, bot_left]) = rect_corners(rect) {
            self.draw_triangle(
                [top_left, top_right, bot_right],
                [texr.top_left(), texr.top_right(), texr.bot_right()],
                fill.clone(),
                viewport,
            );
            self.draw_triangle(
                [bot_right, bot_left, top_left],
                [texr.bot_right(), texr.bot_left(), texr.top_left()],
                fill,
                viewport,
            );
        }
    }

    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect);
}

fn rect_corners(rect: Rect) -> Option<[Point<f32>; 4]> {
    if !rect.size.is_zero_area() {
        Some([
            rect.top_left().cast(),
            rect.top_right().cast(),
            rect.bottom_right().cast(),
            rect.bottom_left().cast(),
        ])
    } else {
        None
    }
}
