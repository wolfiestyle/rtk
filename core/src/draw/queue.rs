use crate::draw::{Color, DrawBackend, DrawCmdPrim, DrawCmdText, DrawCommand, FillMode, Primitive, TexCoord, TextDrawMode, Vertex};
use crate::geometry::Point;
use crate::geometry::Rect;
use crate::image::ImageRef;
use std::borrow::Cow;

/// Buffer with draw commands to be sent to the backend.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DrawQueue<V> {
    /// Shared vertex buffer.
    pub vertices: Vec<V>,
    /// Shared index buffer.
    pub indices: Vec<u32>,
    /// List of draw commands to be executed.
    pub commands: Vec<DrawCommand>,
}

impl<V: Vertex> DrawQueue<V> {
    /// Clears all data from the draw queue.
    #[inline]
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.commands.clear();
    }

    /// Checks if the last draw command has the same state of the incoming one.
    fn get_last_cmd_if_compatible(&mut self, primitive: Primitive, viewport: Rect, texture: &Option<ImageRef>) -> Option<&mut DrawCmdPrim> {
        if let Some(DrawCommand::Primitives(cmd)) = self.commands.last_mut() {
            if cmd.primitive == primitive && &cmd.texture == texture && cmd.viewport == viewport {
                return Some(cmd);
            }
        }
        None
    }

    /// Adds a clear command to the draw queue.
    #[inline]
    pub(crate) fn push_clear(&mut self, color: Color, viewport: Rect) {
        self.commands.push(DrawCommand::Clear(color, viewport))
    }

    /// Adds raw elements to the draw queue.
    pub(crate) fn push_prim(&mut self, primitive: Primitive, vertices: &[V], indices: &[u32], texture: Option<ImageRef>, viewport: Rect) {
        // append vertices to the buffer
        let base_vert = self.vertices.len() as u32;
        self.vertices.extend(vertices);
        // check if the previous draw command can be reused
        if let Some(cmd) = self.get_last_cmd_if_compatible(primitive, viewport, &texture) {
            // we only need to add more indices
            cmd.idx_len += indices.len();
        } else {
            // state change, we need to create a new draw command
            self.commands.push(DrawCommand::Primitives(DrawCmdPrim {
                primitive,
                idx_offset: self.indices.len(),
                idx_len: indices.len(),
                texture,
                viewport,
            }));
        }
        // indices are added with an offset pointing to a single vertex buffer
        self.indices.extend(indices.iter().map(|i| i + base_vert));
    }

    /// Adds a draw text command to the draw queue.
    #[inline]
    pub(crate) fn push_text(
        &mut self, text: Cow<'static, str>, font_desc: Cow<'static, str>, mode: TextDrawMode, color: Color, viewport: Rect,
    ) {
        self.commands.push(DrawCommand::Text(DrawCmdText {
            text,
            font_desc,
            mode,
            color,
            viewport,
        }));
    }
}

//FIXME: temporary impl until we remove DrawQueue
impl<V: Vertex> DrawBackend for DrawQueue<V> {
    fn clear(&mut self, color: Color, viewport: Rect) {
        self.push_clear(color, viewport)
    }

    fn draw_point(&mut self, pos: Point<f32>, texc: TexCoord, fill: FillMode, viewport: Rect) {
        let verts = [Vertex::new(pos, fill.color(), texc)];
        self.push_prim(Primitive::Points, &verts, &[0], fill.texture(), viewport)
    }

    fn draw_line(&mut self, pos: [Point<f32>; 2], texc: [TexCoord; 2], fill: FillMode, viewport: Rect) {
        let color = fill.color();
        let verts = [Vertex::new(pos[0], color, texc[0]), Vertex::new(pos[1], color, texc[1])];
        self.push_prim(Primitive::Lines, &verts, &[0, 1], fill.texture(), viewport)
    }

    fn draw_triangle(&mut self, pos: [Point<f32>; 3], texc: [TexCoord; 3], fill: FillMode, viewport: Rect) {
        let color = fill.color();
        let verts = [
            Vertex::new(pos[0], color, texc[0]),
            Vertex::new(pos[1], color, texc[1]),
            Vertex::new(pos[2], color, texc[2]),
        ];
        self.push_prim(Primitive::Triangles, &verts, &[0, 1, 2], fill.texture(), viewport)
    }

    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect) {
        self.push_text(text.to_owned().into(), font_desc.to_owned().into(), mode, color, viewport)
    }
}
