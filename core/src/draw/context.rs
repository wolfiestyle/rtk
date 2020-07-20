use crate::draw::{Color, DrawBackend, FillMode, TexRect, TextDrawMode};
use crate::geometry::{Point, Position, Rect};
use crate::image::ImageRef;
use crate::widget::Widget;
use array_ext::Array;

/// Draw context attached to a widget.
#[derive(Debug)]
pub struct DrawContext<'a, B> {
    backend: &'a mut B,
    pub(crate) viewport: Rect,
    offset: Position,
    pub vp_orig: Position,
}

impl<'a, B: DrawBackend> DrawContext<'a, B> {
    /// Creates a new context from the speficied DrawBackend.
    #[inline]
    pub fn new(backend: &'a mut B, viewport: Rect) -> Self {
        DrawContext {
            backend,
            viewport,
            offset: viewport.pos,
            vp_orig: Default::default(),
        }
    }

    /// Clears the drawing area.
    #[inline]
    pub fn clear(&mut self, color: impl Into<Color>) {
        self.backend.clear(color.into(), self.viewport)
    }

    /// Draws a child widget.
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

    /// Draws text.
    #[inline]
    pub fn draw_text(&mut self, text: &str, font_desc: &str, mode: impl Into<TextDrawMode>, color: impl Into<Color>) {
        self.backend.draw_text(
            text.into(),
            font_desc.into(),
            mode.into().offset(self.offset),
            color.into(),
            self.viewport,
        )
    }

    /// Draws a point.
    #[inline]
    pub fn draw_point(&mut self, p: impl Into<Point<f32>>, color: impl Into<Color>) {
        let pos = p.into() + self.offset.cast();
        self.backend
            .draw_point(pos, Default::default(), FillMode::Color(color.into()), self.viewport)
    }

    /// Draws a line.
    #[inline]
    pub fn draw_line(&mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, color: impl Into<Color>) {
        let offset = self.offset.cast();
        let verts = [p0.into(), p1.into()].map_(|p| p + offset);
        self.backend
            .draw_line(verts, Default::default(), FillMode::Color(color.into()), self.viewport)
    }

    /// Draws a triangle.
    #[inline]
    pub fn draw_triangle(
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, p2: impl Into<Point<f32>>, color: impl Into<Color>,
    ) {
        let offset = self.offset.cast();
        let verts = [p0.into(), p1.into(), p2.into()].map_(|p| p + offset);
        self.backend
            .draw_triangle(verts, Default::default(), FillMode::Color(color.into()), self.viewport)
    }

    /// Draws a rectangle.
    #[inline]
    pub fn draw_rect(&mut self, rect: impl Into<Rect>, color: impl Into<Color>) {
        let rect = rect.into().offset(self.offset);
        self.backend
            .draw_rect(rect, Default::default(), FillMode::Color(color.into()), self.viewport)
    }

    /// Draws an image.
    #[inline]
    pub fn draw_image(&mut self, pos: impl Into<Position>, image: ImageRef) {
        let rect = Rect::new(pos.into() + self.offset, image.get_size());
        self.backend
            .draw_rect(rect, Default::default(), FillMode::Texture(image), self.viewport)
    }

    /// Draws a rectangle with an image.
    #[inline]
    pub fn draw_image_rect(
        &mut self, rect: impl Into<Rect>, color: impl Into<Color>, image: ImageRef, image_crop: impl Into<Option<TexRect>>,
    ) {
        let rect = rect.into().offset(self.offset);
        self.backend.draw_rect(
            rect,
            image_crop.into().unwrap_or_default(),
            FillMode::ColoredTexture(image, color.into()),
            self.viewport,
        )
    }
}
