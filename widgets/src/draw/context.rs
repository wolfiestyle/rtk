use crate::draw::queue::{DrawError, DrawQueue};
use crate::draw::{Color, ImageRef, Primitive, Vertex};
use crate::geometry::{Point, Rect, Size};
use crate::widget::Widget;

/// Draw context attached to a widget.
#[derive(Debug)]
pub struct DrawContext<'a> {
    queue: &'a mut DrawQueue,
    pub(crate) viewport: Rect,
}

impl<'a> DrawContext<'a> {
    /// Creates a new context from the speficied DrawQueue.
    #[inline]
    pub fn new(queue: &'a mut DrawQueue, viewport: Rect) -> Self {
        DrawContext { queue, viewport }
    }

    /// Clears the drawing area.
    #[inline]
    pub fn clear(&mut self, color: impl Into<Color>) {
        self.queue.push_clear(color.into())
    }

    /// Draws a child widget.
    pub fn draw_child<W: Widget>(&mut self, child: &W) {
        let child_vp = child.get_bounds().offset(self.viewport.pos);
        if let Some(viewport) = child_vp.clip_inside(self.viewport) {
            let dc = DrawContext::new(self.queue, viewport);
            child.draw(dc);
        }
    }

    /// Draws raw elements into the widget area.
    #[inline]
    pub fn draw_prim(
        &mut self, primitive: Primitive, vertices: &[Vertex], indices: &[u32], texture: Option<ImageRef>,
    ) -> Result<(), DrawError> {
        self.queue
            .push_prim(primitive, vertices, indices, texture, self.viewport)
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
        &mut self, p0: impl Into<Point<f32>>, p1: impl Into<Point<f32>>, p2: impl Into<Point<f32>>,
        color: impl Into<Color>,
    ) {
        let color = color.into();
        let verts = [
            Vertex::colored(p0.into(), color),
            Vertex::colored(p1.into(), color),
            Vertex::colored(p2.into(), color),
        ];
        self.draw_prim(Primitive::Triangles, &verts, &[0, 1, 2], None).unwrap()
    }

    /// Draws a rectangle with an optional image.
    pub fn draw_rect(
        &mut self, pos: impl Into<Point<f32>>, size: impl Into<Size>, color: impl Into<Color>, image: Option<ImageRef>,
    ) {
        let size = size.into();
        if size.is_zero_area() {
            return;
        }
        let top_left = pos.into();
        let bot_right = top_left + (size - Size::square(1)).as_pointf();
        let top_right = Point {
            x: bot_right.x,
            y: top_left.y,
        };
        let bot_left = Point {
            x: top_left.x,
            y: bot_right.y,
        };
        let color = color.into();
        let verts = [
            Vertex::new(top_left, color, [0.0, 0.0]),
            Vertex::new(top_right, color, [1.0, 0.0]),
            Vertex::new(bot_right, color, [1.0, 1.0]),
            Vertex::new(bot_left, color, [0.0, 1.0]),
        ];
        self.draw_prim(Primitive::Triangles, &verts, &[0, 1, 2, 2, 3, 0], image)
            .unwrap()
    }
}
