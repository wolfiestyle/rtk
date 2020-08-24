use crate::backend::DrawBackend;
use crate::draw::{Color, FillMode, TextSection, TextureId};
use crate::geometry::{Point, Position, Rect};
use crate::widget::Widget;
use std::ops;

/// Draw context attached to a widget.
#[derive(Debug)]
pub struct DrawContext<'b, B> {
    backend: &'b mut B,
    viewport: Rect,
    abs_bounds: Rect,
    vp_orig: Position,
}

impl<'b, B: DrawBackend> DrawContext<'b, B> {
    /// Creates a new context from the speficied DrawBackend.
    #[inline]
    pub fn new(backend: &'b mut B, viewport: Rect) -> Self {
        DrawContext {
            backend,
            viewport,
            abs_bounds: viewport,
            vp_orig: Default::default(),
        }
    }

    /// Returns the viewport origin (coordinates of top-left corner).
    #[inline]
    pub fn origin(&self) -> Position {
        self.vp_orig
    }

    /// Returns the absolute position of the top-left corner.
    #[inline]
    fn offset(&self) -> Position {
        self.abs_bounds.pos - self.vp_orig
    }

    /// Draws a child widget.
    #[inline]
    pub fn draw_child<W: Widget>(&mut self, child: &W) {
        let abs_bounds = child.get_bounds().offset(self.offset());
        let vp_orig = child.viewport_origin();
        if child.is_clipped() {
            if let Some(viewport) = abs_bounds.clip_inside(self.viewport) {
                let dc = DrawContext {
                    backend: self.backend,
                    viewport,
                    abs_bounds,
                    vp_orig,
                };
                child.draw(dc);
            }
        } else if abs_bounds.intersects(self.viewport) {
            let dc = DrawContext {
                backend: self.backend,
                viewport: self.viewport,
                abs_bounds,
                vp_orig,
            };
            child.draw(dc);
        }
    }

    /// Fills the entire drawing area with a single color.
    #[inline]
    pub fn fill(&mut self, color: impl Into<Color>) {
        self.backend.draw_rect(self.abs_bounds, color.into().into(), self.viewport)
    }

    /// Draws a single triangle.
    #[inline]
    pub fn draw_triangle(
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, p2: impl Into<Point<f32>>, color: impl Into<Color>,
    ) {
        let offset = self.offset().cast();
        let color = color.into().into();
        let verts = [
            (p0.into() + offset, color, Default::default()).into(),
            (p1.into() + offset, color, Default::default()).into(),
            (p2.into() + offset, color, Default::default()).into(),
        ];
        let indices = [0, 1, 2];
        self.backend
            .draw_triangles(verts.iter().copied(), indices.iter().copied(), None, self.viewport)
    }

    /// Draws triangles from vertices and indices.
    #[inline]
    pub fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>)
    where
        V: IntoIterator<Item = B::Vertex>,
        I: IntoIterator<Item = u32>,
    {
        let offset = self.offset().cast();
        let verts = vertices.into_iter().map(|v| v + offset);
        self.backend.draw_triangles(verts, indices, texture, self.viewport)
    }

    /// Draws a rectangle.
    #[inline]
    pub fn draw_rect(&mut self, rect: impl Into<Rect>, fill: impl Into<FillMode>) {
        let rect = rect.into().offset(self.offset());
        self.backend.draw_rect(rect, fill.into(), self.viewport)
    }

    /* FIXME: can we convert this?
    /// Draws an image.
    #[inline]
    pub fn draw_image(&mut self, pos: impl Into<Position>, image: &Image) {
        let rect = Rect::new(pos.into() + self.offset, image.get_size());
        self.backend.draw_rect(rect, image.into(), self.viewport)
    }
    */

    /// Draws text.
    #[inline]
    pub fn draw_text(&mut self, mut text: TextSection) {
        let pos: Point<f32> = text.screen_position.into();
        text.screen_position = (pos + self.offset().cast()).into();
        self.backend.draw_text(text, self.viewport)
    }
}

impl<'b, B: DrawBackend> ops::Deref for DrawContext<'b, B> {
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.backend
    }
}

impl<'b, B: DrawBackend> ops::DerefMut for DrawContext<'b, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.backend
    }
}
