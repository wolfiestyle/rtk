use crate::draw::{Color, DrawBackend, FillMode, TexCoord, TexRect, TextDrawMode};
use crate::geometry::{Point, Position, Rect};
use crate::image::Image;
use crate::widget::Widget;
use array_ext::Array;

/// Draw context attached to a widget.
#[derive(Debug)]
pub struct DrawContext<'b, B> {
    backend: &'b mut B,
    viewport: Rect,
    offset: Position,
    vp_orig: Position,
}

impl<'b, B: DrawBackend> DrawContext<'b, B> {
    /// Creates a new context from the speficied DrawBackend.
    #[inline]
    pub fn new(backend: &'b mut B, viewport: Rect) -> Self {
        DrawContext {
            backend,
            viewport,
            offset: viewport.pos,
            vp_orig: Default::default(),
        }
    }

    /// Returns the viewport origin (coordinates of top-left corner).
    #[inline]
    pub fn origin(&self) -> Position {
        self.vp_orig
    }

    /// Draws a child widget.
    #[inline]
    pub fn draw_child<W: Widget>(&mut self, child: &W) {
        let child_vp = child.get_bounds().offset(self.offset);
        if let Some(viewport) = child_vp.clip_inside(self.viewport) {
            let vp_orig = child.viewport_origin();
            let dc = DrawContext {
                backend: self.backend,
                viewport,
                offset: child_vp.pos - vp_orig,
                vp_orig,
            };
            child.draw(dc);
        }
    }

    /// Clears the drawing area.
    #[inline]
    pub fn clear(&mut self, color: impl Into<Color>) {
        self.backend.clear(color.into(), self.viewport)
    }

    /// Draws a point.
    #[inline]
    pub fn draw_point<'a>(&mut self, p: impl Into<Point<f32>>, texc: impl Into<Option<TexCoord>>, fill: impl Into<FillMode<'a>>) {
        let pos = p.into() + self.offset.cast();
        self.backend
            .draw_point(pos, texc.into().unwrap_or_default(), fill.into(), self.viewport)
    }

    /// Draws a line.
    #[inline]
    pub fn draw_line<'a>(
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, texc: impl Into<Option<[TexCoord; 2]>>,
        fill: impl Into<FillMode<'a>>,
    ) {
        let offset = self.offset.cast();
        let verts = [p0.into(), p1.into()].map_(|p| p + offset);
        self.backend
            .draw_line(verts, texc.into().unwrap_or_default(), fill.into(), self.viewport)
    }

    /// Draws a triangle.
    #[inline]
    pub fn draw_triangle<'a>(
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, p2: impl Into<Point<f32>>, texc: impl Into<Option<[TexCoord; 3]>>,
        fill: impl Into<FillMode<'a>>,
    ) {
        let offset = self.offset.cast();
        let verts = [p0.into(), p1.into(), p2.into()].map_(|p| p + offset);
        self.backend
            .draw_triangle(verts, texc.into().unwrap_or_default(), fill.into(), self.viewport)
    }

    /// Draws a rectangle.
    #[inline]
    pub fn draw_rect<'a>(&mut self, rect: impl Into<Rect>, texr: impl Into<Option<TexRect>>, fill: impl Into<FillMode<'a>>) {
        let rect = rect.into().offset(self.offset);
        self.backend
            .draw_rect(rect, texr.into().unwrap_or_default(), fill.into(), self.viewport)
    }

    /// Draws an image.
    #[inline]
    pub fn draw_image(&mut self, pos: impl Into<Position>, image: &Image) {
        let rect = Rect::new(pos.into() + self.offset, image.get_size());
        self.backend
            .draw_rect(rect, Default::default(), FillMode::Texture(image), self.viewport)
    }

    /// Draws text.
    #[inline]
    pub fn draw_text(&mut self, text: &str, font_desc: &str, mode: impl Into<TextDrawMode>, color: impl Into<Color>) {
        self.backend
            .draw_text(text, font_desc, mode.into().offset(self.offset), color.into(), self.viewport)
    }
}
