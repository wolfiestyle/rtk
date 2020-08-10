use crate::draw::{Color, ColorOp, DrawBackend, FillMode, TexCoord, TextSection};
use crate::geometry::{Point, Position, Rect};
use crate::image::Image;
use crate::widget::Widget;
use std::ops;

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

    /// Draws a single triangle.
    #[inline]
    pub fn draw_triangle(
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, p2: impl Into<Point<f32>>, color: impl Into<Color>,
    ) {
        let offset = self.offset.cast();
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
    pub fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, color: impl Into<Color>)
    where
        V: IntoIterator<Item = Point<f32>>,
        I: IntoIterator<Item = u32>,
    {
        let offset = self.offset.cast();
        let color = color.into().into();
        let verts = vertices.into_iter().map(|p| (p + offset, color, Default::default()).into());
        self.backend.draw_triangles(verts, indices, None, self.viewport)
    }

    /// Draws textured triangles from vertices and indices.
    #[inline]
    pub fn draw_triangles_uv<V, I>(&mut self, vertices: V, indices: I, color: impl Into<ColorOp>, image: &Image)
    where
        V: IntoIterator<Item = (Point<f32>, TexCoord)>,
        I: IntoIterator<Item = u32>,
    {
        let offset = self.offset.cast();
        let color = color.into();
        let verts = vertices.into_iter().map(|(pos, texc)| (pos + offset, color, texc).into());
        self.backend.draw_triangles(verts, indices, Some(image), self.viewport)
    }

    /// Draws a rectangle.
    #[inline]
    pub fn draw_rect<'a>(&mut self, rect: impl Into<Rect>, fill: impl Into<FillMode<'a>>) {
        let rect = rect.into().offset(self.offset);
        self.backend.draw_rect(rect, fill.into(), self.viewport)
    }

    /// Draws an image.
    #[inline]
    pub fn draw_image(&mut self, pos: impl Into<Position>, image: &Image) {
        let rect = Rect::new(pos.into() + self.offset, image.get_size());
        self.backend.draw_rect(rect, image.into(), self.viewport)
    }

    /// Draws text.
    #[inline]
    pub fn draw_text(&mut self, mut text: TextSection) {
        let pos: Point<f32> = text.screen_position.into();
        text.screen_position = (pos + self.offset.cast()).into();
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
