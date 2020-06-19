use crate::draw::{Color, DrawCmdPrim, DrawCmdText, DrawCommand, ImageRef, Primitive, TextDrawMode, Vertex};
use crate::geometry::{Point, Rect};
use std::borrow::Cow;
use std::fmt;

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
    /// Creates a new draw queue.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Clears all data from the draw queue.
    #[inline]
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.commands.clear();
    }

    /// Checks if the last draw command has the same state of the incoming one.
    fn get_last_cmd_if_compatible(
        &mut self, primitive: Primitive, viewport: Rect, texture: &Option<ImageRef>,
    ) -> Option<&mut DrawCmdPrim> {
        if let Some(DrawCommand::Primitives(cmd)) = self.commands.last_mut() {
            if cmd.primitive == primitive && &cmd.texture == texture && cmd.viewport == viewport {
                return Some(cmd);
            }
        }
        None
    }

    /// Adds a clear command to the draw queue.
    #[inline]
    pub(crate) fn push_clear(&mut self, color: Color) {
        self.commands.push(DrawCommand::Clear(color))
    }

    /// Adds raw elements to the draw queue.
    pub(crate) fn push_prim(
        &mut self, primitive: Primitive, vertices: &[Vertex], indices: &[u32], texture: Option<ImageRef>,
        viewport: Rect, offset: Point<f32>,
    ) -> Result<(), DrawError> {
        let nvert = vertices.len() as u32;
        // no vertices means nothing to do
        if nvert == 0 {
            return Ok(());
        }
        // check if indices are in range
        if let Some(&idx) = indices.iter().find(|&&i| i >= nvert) {
            return Err(DrawError::IndexOutOfBounds { idx, nvert });
        }
        // append vertices to the buffer
        let base_vert = self.vertices.len() as u32;
        self.vertices.extend(vertices.iter().map(|v| v.translate(offset)));
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
        Ok(())
    }

    /// Adds a draw text command to the draw queue.
    #[inline]
    pub(crate) fn push_text(
        &mut self, text: Cow<'static, str>, font_desc: Cow<'static, str>, mode: TextDrawMode, color: Color,
        viewport: Rect,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawError {
    IndexOutOfBounds { idx: u32, nvert: u32 },
}

impl fmt::Display for DrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DrawError::IndexOutOfBounds { idx, nvert } => write!(f, "index {} out of bounds ({})", idx, nvert),
        }
    }
}
