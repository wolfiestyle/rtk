use crate::shared_res::SharedRes;
use crate::vertex::{TextVertex, Vertex};
use glium::index::PrimitiveType;
use glium::texture::SrgbTexture2d;
use glium::GlObject;
use glium::{uniform, Surface};
use glyph_brush::{BrushAction, BrushError};
use std::fmt;
use std::ops::Range;
use std::rc::Rc;
use widgets::draw::{BackendResources, Color, DrawBackend, TextSection};
use widgets::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use widgets::geometry::{Rect, Size};
use widgets::image::Image;

/// Draw command detail.
#[derive(Debug, Clone)]
struct DrawCmdData {
    /// Range inside the shared vertex buffer.
    idx_range: Range<usize>,
    /// Image to use for this draw command.
    texture: Option<Rc<SrgbTexture2d>>,
    /// Clipping viewport.
    viewport: Rect,
}

impl DrawCmdData {
    #[inline]
    fn compatible_with(&self, viewport: Rect, texture: &Option<Rc<SrgbTexture2d>>) -> bool {
        self.viewport == viewport && self.texture.as_ref().map(|t| t.get_id()) == texture.as_ref().map(|t| t.get_id())
    }
}

/// A single draw command.
#[derive(Debug, Clone)]
enum DrawCommand {
    Clear(Color, Rect),
    Triangles(DrawCmdData),
    Text { verts: Vec<TextVertex>, viewport: Rect },
    TextRedraw(Rect),
}

/// Buffer with draw commands to be sent to the backend.
pub struct DrawQueue {
    /// Shared vertex buffer.
    vertices: Vec<Vertex>,
    /// Shared index buffer.
    indices: Vec<u32>,
    /// List of draw commands to be executed.
    commands: Vec<DrawCommand>,
    /// Shared OpenGL resources
    shared_res: Rc<SharedRes>,
    /// Glium context handle
    pub display: glium::Display,
}

impl DrawQueue {
    pub fn new(display: glium::Display, shared_res: Rc<SharedRes>) -> Self {
        Self {
            vertices: Default::default(),
            indices: Default::default(),
            commands: Default::default(),
            shared_res,
            display,
        }
    }

    /// Clears all data from the draw queue.
    #[inline]
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.commands.clear();
        //self.texture_map.remove_expired();  //TODO: GC old textures
    }

    /// Adds a clear command to the draw queue.
    #[inline]
    fn push_clear(&mut self, color: Color, viewport: Rect) {
        self.commands.push(DrawCommand::Clear(color, viewport))
    }

    /// Adds raw elements to the draw queue.
    #[inline]
    fn push_tris<V, I>(&mut self, vertices: V, indices: I, texture: Option<Rc<SrgbTexture2d>>, viewport: Rect)
    where
        V: Iterator<Item = Vertex>,
        I: Iterator<Item = u32>,
    {
        // append vertices to the buffer
        let base_vert = self.vertices.len() as u32;
        self.vertices.extend(vertices);
        // indices are added with an offset pointing to a single vertex buffer
        let base_idx = self.indices.len();
        self.indices.extend(indices.map(|i| i + base_vert));

        // check if the last draw command has the same state of the incoming one
        match self.commands.last_mut() {
            Some(DrawCommand::Triangles(cmd)) if cmd.compatible_with(viewport, &texture) => {
                // ..then we only need to add more indices
                cmd.idx_range.end = self.indices.len();
            }
            _ => {
                // state change, we need to create a new draw command
                self.commands.push(DrawCommand::Triangles(DrawCmdData {
                    idx_range: base_idx..self.indices.len(),
                    texture,
                    viewport,
                }));
            }
        }
    }

    /// Adds text to the draw queue.
    #[inline]
    fn push_text(&mut self, text: TextSection, viewport: Rect) {
        let mut glyph_brush = self.shared_res.glyph_brush.borrow_mut();
        glyph_brush.queue(text);
        let action = glyph_brush.process_queued(|rect, data| self.shared_res.update_font_tex(rect, data), |gvert| gvert.into());
        match action {
            Ok(BrushAction::Draw(verts)) => self.commands.push(DrawCommand::Text { verts, viewport }),
            Ok(BrushAction::ReDraw) => self.commands.push(DrawCommand::TextRedraw(viewport)),
            Err(BrushError::TextureTooSmall { suggested }) => {
                todo!("resize_tex: {:?}", suggested);
            }
        }
    }

    /// Runs the stored draw commands.
    pub fn execute(&self) {
        let vertex_buf = glium::VertexBuffer::new(&self.display, &self.vertices).unwrap();
        let index_buf = glium::index::IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &self.indices).unwrap();
        let mut draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let win_size: Size = self.display.get_framebuffer_dimensions().into();
        let mut target = self.display.draw();

        for drawcmd in &self.commands {
            match drawcmd {
                DrawCommand::Clear(color, viewport) => {
                    if let Some(vp) = viewport.clip_inside(win_size.into()) {
                        let rect = to_glium_rect(vp, win_size.h);
                        target.clear(Some(&rect), Some((color.r, color.g, color.b, color.a)), false, None, None);
                    }
                }
                DrawCommand::Triangles(cmd) => {
                    // clip the viewport against the visible window area
                    if let Some(scissor) = cmd.viewport.clip_inside(win_size.into()) {
                        // indices reference a single shared vertex buffer
                        let indices = index_buf.slice(cmd.idx_range.clone()).unwrap();
                        // get texture to use
                        let texture = cmd.texture.as_deref().unwrap_or(&self.shared_res.t_white);
                        // settings for the pipeline
                        let uniforms = uniform! {
                            vp_size: <[f32; 2]>::from(win_size.as_point()),
                            tex: texture.sampled()
                                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                        };
                        draw_params.scissor = Some(to_glium_rect(scissor, win_size.h));
                        // perform the draw command
                        target
                            .draw(&vertex_buf, indices, &self.shared_res.program, &uniforms, &draw_params)
                            .unwrap();
                    }
                }
                DrawCommand::Text { verts, viewport } => {
                    if let Some(scissor) = viewport.clip_inside(win_size.into()) {
                        let vertex_buf = glium::VertexBuffer::new(&self.display, &verts).unwrap();
                        let uniforms = uniform! {
                            vp_size: <[f32; 2]>::from(win_size.as_point()),
                            tex: self.shared_res.font_tex.sampled()
                                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        };
                        draw_params.scissor = Some(to_glium_rect(scissor, win_size.h));
                        target
                            .draw(
                                (glium::vertex::EmptyVertexAttributes { len: 4 }, vertex_buf.per_instance().unwrap()),
                                glium::index::NoIndices(PrimitiveType::TriangleStrip),
                                &self.shared_res.text_prog,
                                &uniforms,
                                &draw_params,
                            )
                            .unwrap();
                    }
                }
                DrawCommand::TextRedraw(viewport) => {
                    todo!("redraw_text: {:?}", viewport);
                }
            }
        }

        target.finish().unwrap();
    }
}

impl BackendResources for DrawQueue {
    #[inline]
    fn enumerate_fonts(&self) -> Vec<String> {
        self.shared_res.enumerate_fonts()
    }

    #[inline]
    fn select_font(&self, family_names: &[FontFamily], properties: &FontProperties) -> Option<FontSource> {
        self.shared_res.select_font(family_names, properties)
    }

    #[inline]
    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError> {
        self.shared_res.load_font(font_src)
    }
}

impl DrawBackend for DrawQueue {
    type Vertex = Vertex;

    #[inline]
    fn clear(&mut self, color: Color, viewport: Rect) {
        self.push_clear(color, viewport)
    }

    #[inline]
    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, image: Option<&Image>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>,
    {
        let texture = image.map(|img| self.shared_res.load_texture(img));
        self.push_tris(vertices.into_iter(), indices.into_iter(), texture, viewport)
    }

    #[inline]
    fn draw_text(&mut self, text: TextSection, viewport: Rect) {
        self.push_text(text, viewport)
    }
}

impl fmt::Debug for DrawQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GliumWindow")
            .field("vertices", &self.vertices)
            .field("indices", &self.indices)
            .field("commands", &self.commands)
            .field("shared_res", &self.shared_res)
            .field("display", &format_args!("..."))
            .finish()
    }
}

fn to_glium_rect(rect: widgets::geometry::Rect, win_height: u32) -> glium::Rect {
    glium::Rect {
        left: rect.pos.x as u32,
        bottom: win_height - rect.size.h - rect.pos.y as u32,
        width: rect.size.w,
        height: rect.size.h,
    }
}
