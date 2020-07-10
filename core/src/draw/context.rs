use crate::draw::queue::{DrawError, DrawQueue};
use crate::draw::{Color, Primitive, TexCoord, TexRect, TextDrawMode, Vertex};
use crate::geometry::{Point, Position, Rect};
use crate::image::ImageRef;
use crate::widget::Widget;
use std::borrow::Cow;

/// Draw context attached to a widget.
#[derive(Debug)]
pub struct DrawContext<'a> {
    queue: &'a mut DrawQueue,
    pub(crate) viewport: Rect,
    offset: Position,
    pub vp_orig: Position,
}

impl<'a> DrawContext<'a> {
    /// Creates a new context from the speficied DrawQueue.
    #[inline]
    pub fn new(queue: &'a mut DrawQueue, viewport: Rect) -> Self {
        DrawContext {
            queue,
            viewport,
            offset: viewport.pos,
            vp_orig: Default::default(),
        }
    }

    /// Clears the drawing area.
    #[inline]
    pub fn clear(&mut self, color: impl Into<Color>) {
        self.queue.push_clear(color.into(), self.viewport)
    }

    /// Draws a child widget.
    pub fn draw_child<W: Widget>(&mut self, child: &W) {
        let child_vp = child.get_bounds().offset(self.offset);
        if let Some(viewport) = child_vp.clip_inside(self.viewport) {
            let vp_orig = child.viewport_origin();
            let dc = DrawContext {
                queue: self.queue,
                viewport,
                offset: child_vp.pos - vp_orig,
                vp_orig,
            };
            child.draw(dc);
        }
    }

    /// Draws raw elements into the widget area.
    #[inline]
    pub fn draw_prim(
        &mut self, primitive: Primitive, vertices: &[Vertex], indices: &[u32], texture: Option<ImageRef>,
    ) -> Result<(), DrawError> {
        self.queue
            .push_prim(primitive, vertices, indices, texture, self.viewport, self.offset.cast())
    }

    /// Draws text.
    #[inline]
    pub fn draw_text(
        &mut self, text: impl Into<Cow<'static, str>>, font_desc: impl Into<Cow<'static, str>>, mode: impl Into<TextDrawMode>,
        color: impl Into<Color>,
    ) {
        self.queue
            .push_text(text.into(), font_desc.into(), mode.into(), color.into(), self.viewport, self.offset)
    }

    /// Draws a point.
    pub fn draw_point(&mut self, p: impl Into<Point<f32>>, color: impl Into<Color>) {
        let verts = [Vertex::colored(p.into(), color.into())];
        self.draw_prim(Primitive::Points, &verts, &[0], None).unwrap()
    }

    /// Draws a line.
    pub fn draw_line(&mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, color: impl Into<Color>) {
        let color = color.into();
        let verts = [Vertex::colored(p0.into(), color), Vertex::colored(p1.into(), color)];
        self.draw_prim(Primitive::Lines, &verts, &[0, 1], None).unwrap()
    }

    /// Draws a triangle.
    pub fn draw_triangle(
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, p2: impl Into<Point<f32>>, color: impl Into<Color>,
    ) {
        let color = color.into();
        let verts = [
            Vertex::colored(p0.into(), color),
            Vertex::colored(p1.into(), color),
            Vertex::colored(p2.into(), color),
        ];
        self.draw_prim(Primitive::Triangles, &verts, &[0, 1, 2], None).unwrap()
    }

    /// Draws a rectangle.
    pub fn draw_rect(&mut self, rect: impl Into<Rect>, color: impl Into<Color>) {
        if let Some([top_left, top_right, bot_left, bot_right]) = rect_corners(rect) {
            let color = color.into();
            let verts = [
                Vertex::colored(top_left, color),
                Vertex::colored(top_right, color),
                Vertex::colored(bot_right, color),
                Vertex::colored(bot_left, color),
            ];
            self.draw_prim(Primitive::Triangles, &verts, &[0, 1, 2, 2, 3, 0], None).unwrap()
        }
    }

    /// Draws an image.
    pub fn draw_image(&mut self, pos: impl Into<Position>, image: ImageRef) {
        let rect = Rect::new(pos, image.get_size());
        if let Some([top_left, top_right, bot_left, bot_right]) = rect_corners(rect) {
            let verts = [
                Vertex::textured(top_left, TexCoord::TOP_LEFT),
                Vertex::textured(top_right, TexCoord::TOP_RIGHT),
                Vertex::textured(bot_right, TexCoord::BOTTOM_RIGHT),
                Vertex::textured(bot_left, TexCoord::BOTTOM_LEFT),
            ];
            self.draw_prim(Primitive::Triangles, &verts, &[0, 1, 2, 2, 3, 0], Some(image))
                .unwrap()
        }
    }

    /// Draws a rectangle with an image.
    pub fn draw_image_rect(
        &mut self, rect: impl Into<Rect>, color: impl Into<Color>, image: ImageRef, image_crop: impl Into<Option<TexRect>>,
    ) {
        if let Some([top_left, top_right, bot_left, bot_right]) = rect_corners(rect) {
            let color = color.into();
            let tex_rect = image_crop.into().unwrap_or_default();
            let verts = [
                Vertex::new(top_left, color, tex_rect.top_left),
                Vertex::new(top_right, color, tex_rect.top_right()),
                Vertex::new(bot_right, color, tex_rect.bot_right),
                Vertex::new(bot_left, color, tex_rect.bot_left()),
            ];
            self.draw_prim(Primitive::Triangles, &verts, &[0, 1, 2, 2, 3, 0], Some(image))
                .unwrap()
        }
    }
}

fn rect_corners(rect: impl Into<Rect>) -> Option<[Point<f32>; 4]> {
    let rect = rect.into();
    if !rect.size.is_zero_area() {
        Some([
            rect.top_left().cast(),
            rect.top_right().cast(),
            rect.bottom_left().cast(),
            rect.bottom_right().cast(),
        ])
    } else {
        None
    }
}
