use crate::shared_res::{FontTex, SharedResources};
use crate::vertex::Vertex;
use glium::index::PrimitiveType;
use glium::{uniform, Surface};
use glyph_brush::{BrushAction, BrushError};
use std::ops::Range;
use widgets::backend::{DrawBackend, Resources, TextureError};
use widgets::draw::{Color, TextSection, TextureId};
use widgets::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use widgets::geometry::{Rect, Size};
use widgets::image::Image;

/// Buffer with draw commands to be sent to the backend.
#[derive(Debug)]
pub struct DrawQueue<'a> {
    /// Shared vertex buffer.
    vertices: Vec<Vertex>,
    /// Shared index buffer.
    indices: Vec<u32>,
    /// List of draw commands to be executed.
    commands: Vec<DrawCommand>,
    /// Shared GL resources used for drawing.
    shared_res: &'a mut SharedResources,
}

impl<'a> DrawQueue<'a> {
    pub fn new(shared_res: &'a mut SharedResources) -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            commands: vec![],
            shared_res,
        }
    }

    /// Adds raw elements to the draw queue.
    #[inline]
    pub fn push_tris<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>, viewport: Rect)
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
            Some(DrawCommand::Triangles(cmd)) if cmd.compatible_with(viewport, texture) => {
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
    pub fn push_text(&mut self, text: TextSection, viewport: Rect) {
        self.commands.push(DrawCommand::Text(text, viewport))
    }

    /// Runs the stored draw commands by drawing them into the framebuffer.
    pub fn render(&mut self, display: &glium::Display, clear_color: Option<Color>) {
        let vertex_buf = glium::VertexBuffer::new(display, &self.vertices).unwrap();
        let index_buf = glium::index::IndexBuffer::new(display, PrimitiveType::TrianglesList, &self.indices).unwrap();
        let mut draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let win_size: Size = display.get_framebuffer_dimensions().into();
        let mut target = display.draw();

        if let Some(Color { r, g, b, a }) = clear_color {
            target.clear_color(r, g, b, a);
        }

        for drawcmd in &self.commands {
            match drawcmd {
                DrawCommand::Triangles(cmd) => {
                    // clip the viewport against the visible window area
                    if let Some(scissor) = cmd.viewport.clip_inside(win_size.into()) {
                        // indices reference a single shared vertex buffer
                        let indices = index_buf.slice(cmd.idx_range.clone()).unwrap();
                        // get texture to use
                        let texture = cmd
                            .texture
                            .and_then(|id| self.shared_res.texture_map.get(&id))
                            .unwrap_or(&self.shared_res.t_white);
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
                DrawCommand::Text(section, viewport) => {
                    if let Some(scissor) = viewport.clip_inside(win_size.into()) {
                        self.shared_res.glyph_brush.queue(section);
                        let font_tex = &self.shared_res.font_tex;
                        let action = self
                            .shared_res
                            .glyph_brush
                            .process_queued(|rect, data| font_tex.update(rect, data), |gvert| gvert.into());
                        match action {
                            Ok(BrushAction::Draw(verts)) => {
                                let vertex_buf = glium::VertexBuffer::new(display, &verts).unwrap();
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
                            Ok(BrushAction::ReDraw) => unimplemented!(),
                            Err(BrushError::TextureTooSmall { suggested: (w, h) }) => {
                                self.shared_res.font_tex = FontTex::new(&self.shared_res.display, (w, h)).unwrap();
                                self.shared_res.glyph_brush.resize_texture(w, h);
                            }
                        }
                    }
                }
            }
        }

        target.finish().unwrap();
    }
}

impl DrawBackend for DrawQueue<'_> {
    type Vertex = Vertex;

    #[inline]
    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>,
    {
        self.push_tris(vertices.into_iter(), indices.into_iter(), texture, viewport)
    }

    #[inline]
    fn draw_text(&mut self, text: TextSection, viewport: Rect) {
        self.push_text(text, viewport)
    }
}

impl Resources for DrawQueue<'_> {
    #[inline]
    fn load_texture(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError> {
        self.shared_res.load_texture(id, image)
    }

    #[inline]
    fn load_texture_once(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError> {
        self.shared_res.load_texture_once(id, image)
    }

    #[inline]
    fn delete_texture(&mut self, id: TextureId) {
        self.shared_res.delete_texture(id)
    }

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

/// A single draw command.
#[derive(Debug, Clone)]
enum DrawCommand {
    Triangles(DrawCmdData),
    Text(TextSection, Rect),
}

/// Draw command detail.
#[derive(Debug, Clone)]
struct DrawCmdData {
    /// Range inside the shared vertex buffer.
    idx_range: Range<usize>,
    /// Image to use for this draw command.
    texture: Option<TextureId>,
    /// Clipping viewport.
    viewport: Rect,
}

impl DrawCmdData {
    #[inline]
    fn compatible_with(&self, viewport: Rect, texture: Option<TextureId>) -> bool {
        self.viewport == viewport && self.texture == texture
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
