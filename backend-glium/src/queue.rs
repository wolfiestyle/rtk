use glium::index::PrimitiveType;
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d, SrgbTexture2d, TextureCreationError};
use glium::{uniform, Surface};
use std::borrow::Cow;
use std::fmt;
use weak_table::WeakKeyHashMap;
use widgets::draw::{Color, DrawBackend, FillMode, TexCoord, TextDrawMode};
use widgets::geometry::{Point, Rect, Size};
use widgets::image::{Image, ImageData, ImageRef, ImageWeakRef, PixelFormat};

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
struct DrawCmdPrim {
    /// The primitive to draw.
    primitive: Primitive,
    /// Offset inside the shared index buffer on the draw queue.
    idx_offset: usize,
    /// Length of the indices slice.
    idx_len: usize,
    /// Image to use for this draw command.
    texture: Option<ImageRef>,
    /// Clipping viewport.
    viewport: Rect,
}

/// Text draw command detail.
#[derive(Debug, Clone)]
struct DrawCmdText {
    text: Cow<'static, str>,
    font_desc: Cow<'static, str>,
    mode: TextDrawMode,
    color: Color,
    viewport: Rect,
}

/// A single draw command.
#[derive(Debug, Clone)]
enum DrawCommand {
    Clear(Color, Rect),
    Primitives(DrawCmdPrim),
    Text(DrawCmdText),
}

/// Buffer with draw commands to be sent to the backend.
//#[derive(Debug)]
pub struct DrawQueue {
    /// Shared vertex buffer.
    vertices: Vec<Vertex>,
    /// Shared index buffer.
    indices: Vec<u32>,
    /// List of draw commands to be executed.
    commands: Vec<DrawCommand>,
    texture_map: WeakKeyHashMap<ImageWeakRef, SrgbTexture2d>,
    t_white: SrgbTexture2d,
    program: glium::Program,
    pub display: glium::Display,
}

impl DrawQueue {
    pub fn new(display: glium::Display) -> Self {
        let vert_src = include_str!("widgets.vert.glsl");
        let frag_src = include_str!("widgets.frag.glsl");
        let program = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();

        let image = RawImage2d::from_raw_rgba(vec![255u8; 4], (1, 1));
        let t_white = SrgbTexture2d::with_mipmaps(&display, image, MipmapsOption::NoMipmap).unwrap();

        Self {
            vertices: Default::default(),
            indices: Default::default(),
            commands: Default::default(),
            program,
            texture_map: Default::default(),
            t_white,
            display,
        }
    }

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
    fn push_clear(&mut self, color: Color, viewport: Rect) {
        self.commands.push(DrawCommand::Clear(color, viewport))
    }

    /// Adds raw elements to the draw queue.
    fn push_prim(&mut self, primitive: Primitive, vertices: &[Vertex], indices: &[u32], texture: Option<ImageRef>, viewport: Rect) {
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
    fn push_text(&mut self, text: Cow<'static, str>, font_desc: Cow<'static, str>, mode: TextDrawMode, color: Color, viewport: Rect) {
        self.commands.push(DrawCommand::Text(DrawCmdText {
            text,
            font_desc,
            mode,
            color,
            viewport,
        }));
    }

    pub fn load_textures(&mut self) {
        self.texture_map.remove_expired();

        for cmd in &self.commands {
            if let DrawCommand::Primitives(DrawCmdPrim { texture: Some(image), .. }) = cmd {
                let display = &self.display;
                self.texture_map
                    .entry(image.clone())
                    .or_insert_with(|| to_glium_texture(image, display).unwrap());
            }
        }
    }

    /// Runs the stored draw commands.
    pub fn execute(&self, win_size: Size) {
        let mut target = self.display.draw();
        let vertex_buf = glium::VertexBuffer::new(&self.display, &self.vertices).unwrap();

        for drawcmd in &self.commands {
            match drawcmd {
                DrawCommand::Clear(color, viewport) => {
                    if let Some(vp) = viewport.clip_inside(win_size.into()) {
                        let rect = to_glium_rect(vp, win_size.h);
                        target.clear(Some(&rect), Some((color.r, color.g, color.b, color.a)), false, None, None);
                    }
                }
                DrawCommand::Primitives(cmd) => {
                    // clip the viewport against the visible window area
                    if let Some(scissor) = cmd.viewport.clip_inside(win_size.into()) {
                        let mode = match cmd.primitive {
                            Primitive::Points => PrimitiveType::Points,
                            Primitive::Lines => PrimitiveType::LinesList,
                            Primitive::Triangles => PrimitiveType::TrianglesList,
                        };
                        // indices reference a single shared vertex buffer
                        let indices = &self.indices[cmd.idx_offset..cmd.idx_offset + cmd.idx_len];
                        let index_buf = glium::IndexBuffer::new(&self.display, mode, indices).unwrap();
                        // get texture to use
                        let texture = cmd
                            .texture
                            .as_ref()
                            .and_then(|img| self.texture_map.get(img))
                            .unwrap_or(&self.t_white);
                        // settings for the pipeline
                        let uniforms = uniform! {
                            vp_size: <[f32; 2]>::from(win_size.as_point()),
                            tex: texture.sampled()
                                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                        };
                        let draw_params = glium::DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            scissor: Some(to_glium_rect(scissor, win_size.h)),
                            ..Default::default()
                        };
                        // perform the draw command
                        target
                            .draw(&vertex_buf, &index_buf, &self.program, &uniforms, &draw_params)
                            .unwrap();
                    }
                }
                DrawCommand::Text(cmd) => {
                    //TODO: implement text drawing
                    dbg!(cmd);
                }
            }
        }

        target.finish().unwrap();
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

impl fmt::Debug for DrawQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GliumWindow")
            .field("vertices", &self.vertices)
            .field("indices", &self.indices)
            .field("commands", &self.commands)
            .field("texture_map", &self.texture_map)
            .field("t_white", &self.t_white)
            .field("program", &self.program)
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

fn to_glium_texture(image: &Image, display: &glium::Display) -> Result<SrgbTexture2d, TextureCreationError> {
    let (width, height) = image.get_size().into();
    match image.get_data() {
        None => SrgbTexture2d::empty(display, width, height),
        Some(ImageData::U8(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::U8,
                    PixelFormat::LumaA => ClientFormat::U8U8,
                    PixelFormat::Rgb => ClientFormat::U8U8U8,
                    PixelFormat::Rgba => ClientFormat::U8U8U8U8,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
        Some(ImageData::U16(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::U16,
                    PixelFormat::LumaA => ClientFormat::U16U16,
                    PixelFormat::Rgb => ClientFormat::U16U16U16,
                    PixelFormat::Rgba => ClientFormat::U16U16U16U16,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
        Some(ImageData::U32(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::U32,
                    PixelFormat::LumaA => ClientFormat::U32U32,
                    PixelFormat::Rgb => ClientFormat::U32U32U32,
                    PixelFormat::Rgba => ClientFormat::U32U32U32U32,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
        Some(ImageData::F32(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::F32,
                    PixelFormat::LumaA => ClientFormat::F32F32,
                    PixelFormat::Rgb => ClientFormat::F32F32F32,
                    PixelFormat::Rgba => ClientFormat::F32F32F32F32,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
    }
}
