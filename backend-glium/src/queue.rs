use std::borrow::Cow;
use widgets::draw::{Color, DrawBackend, FillMode, TexCoord, TextDrawMode};
use widgets::geometry::Point;
use widgets::geometry::Rect;
use widgets::image::ImageRef;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
    texc: [f32; 2],
}

glium::implement_vertex!(Vertex, pos, color, texc);

impl Vertex {
    fn new(pos: Point<f32>, color: Color, texc: TexCoord) -> Self {
        Self {
            pos: pos.into(),
            color: color.into(),
            texc: texc.into(),
        }
    }
}

/// Types of drawing primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    Points,
    Lines,
    Triangles,
}

/// Primitive draw command detail.
#[derive(Debug, Clone)]
pub struct DrawCmdPrim {
    /// The primitive to draw.
    pub primitive: Primitive,
    /// Offset inside the shared index buffer on the draw queue.
    pub idx_offset: usize,
    /// Length of the indices slice.
    pub idx_len: usize,
    /// Image to use for this draw command.
    pub texture: Option<ImageRef>,
    /// Clipping viewport.
    pub viewport: Rect,
}

/// Text draw command detail.
#[derive(Debug, Clone)]
pub struct DrawCmdText {
    pub text: Cow<'static, str>,
    pub font_desc: Cow<'static, str>,
    pub mode: TextDrawMode,
    pub color: Color,
    pub viewport: Rect,
}

/// A single draw command.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    Clear(Color, Rect),
    Primitives(DrawCmdPrim),
    Text(DrawCmdText),
}

/// Buffer with draw commands to be sent to the backend.
#[derive(Debug, Clone, Default)]
pub struct DrawQueue {
    /// Shared vertex buffer.
    pub vertices: Vec<Vertex>,
    /// Shared index buffer.
    pub indices: Vec<u32>,
    /// List of draw commands to be executed.
    pub commands: Vec<DrawCommand>,
}

impl DrawQueue {
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
    pub(crate) fn push_prim(
        &mut self, primitive: Primitive, vertices: &[Vertex], indices: &[u32], texture: Option<ImageRef>, viewport: Rect,
    ) {
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

impl DrawBackend for DrawQueue {
    #[inline]
    fn clear(&mut self, color: Color, viewport: Rect) {
        self.push_clear(color, viewport)
    }

    #[inline]
    fn draw_point(&mut self, pos: Point<f32>, texc: TexCoord, fill: FillMode, viewport: Rect) {
        let verts = [Vertex::new(pos, fill.color(), texc)];
        self.push_prim(Primitive::Points, &verts, &[0], fill.texture(), viewport)
    }

    #[inline]
    fn draw_line(&mut self, pos: [Point<f32>; 2], texc: [TexCoord; 2], fill: FillMode, viewport: Rect) {
        let color = fill.color();
        let verts = [Vertex::new(pos[0], color, texc[0]), Vertex::new(pos[1], color, texc[1])];
        self.push_prim(Primitive::Lines, &verts, &[0, 1], fill.texture(), viewport)
    }

    #[inline]
    fn draw_triangle(&mut self, pos: [Point<f32>; 3], texc: [TexCoord; 3], fill: FillMode, viewport: Rect) {
        let color = fill.color();
        let verts = [
            Vertex::new(pos[0], color, texc[0]),
            Vertex::new(pos[1], color, texc[1]),
            Vertex::new(pos[2], color, texc[2]),
        ];
        self.push_prim(Primitive::Triangles, &verts, &[0, 1, 2], fill.texture(), viewport)
    }

    #[inline]
    fn draw_text(&mut self, text: &str, font_desc: &str, mode: TextDrawMode, color: Color, viewport: Rect) {
        self.push_text(text.to_owned().into(), font_desc.to_owned().into(), mode, color, viewport)
    }
}
