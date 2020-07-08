use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowId;
use glium::glutin::{ContextBuilder, GlProfile, Robustness};
use glium::index::PrimitiveType;
use glium::texture::{ClientFormat, RawImage2d, SrgbTexture2d};
use glium::{uniform, Surface};
use std::fmt;
use weak_table::WeakKeyHashMap;
use widgets::draw::{DrawCmdPrim, DrawCommand, DrawQueue, Primitive};
use widgets::image::{ImageData, ImageWeakRef, PixelFormat};
use widgets::toplevel::{TopLevel, WindowAttributes};
use widgets_winit::{make_win_builder, BackendWindow};

pub struct GliumWindow<T> {
    display: glium::Display,
    program: glium::Program,
    t_white: SrgbTexture2d,
    texture_map: WeakKeyHashMap<ImageWeakRef, SrgbTexture2d>,
    draw_queue: DrawQueue,
    cur_attr: WindowAttributes,
    pub window: T,
}

impl<T: TopLevel> GliumWindow<T> {
    pub fn new(window: T, event_loop: &EventLoop<()>) -> Self {
        let win_attr = window.get_attr();
        let win_builder = make_win_builder(win_attr);

        let mut ctx = ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_gl_robustness(Robustness::TryRobustNoResetNotification)
            .with_double_buffer(Some(true));
        ctx.pf_reqs.hardware_accelerated = None;
        ctx.pf_reqs.depth_bits = None;
        ctx.pf_reqs.stencil_bits = None;

        let display = glium::Display::new(win_builder, ctx, event_loop).unwrap();

        if let Some(pos) = win_attr.position {
            display.gl_window().window().set_outer_position(PhysicalPosition::new(pos.x, pos.y));
        }

        let vert_src = include_str!("widgets.vert.glsl");
        let frag_src = include_str!("widgets.frag.glsl");
        let program = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();

        let image = RawImage2d::from_raw_rgba(vec![255u8; 4], (1, 1));
        let t_white = SrgbTexture2d::new(&display, image).unwrap();

        Self {
            display,
            program,
            t_white,
            texture_map: Default::default(),
            draw_queue: DrawQueue::new(),
            cur_attr: win_attr.clone(),
            window,
        }
    }

    fn draw_elements(&self, target: &mut glium::Frame) {
        let win_size = self.window.get_attr().size;
        let vertex_buf = glium::VertexBuffer::new(&self.display, &self.draw_queue.vertices).unwrap();

        for drawcmd in &self.draw_queue.commands {
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
                            Primitive::LineStrip => PrimitiveType::LineStrip,
                            Primitive::Triangles => PrimitiveType::TrianglesList,
                            Primitive::TriangleStrip => PrimitiveType::TriangleStrip,
                            Primitive::TriangleFan => PrimitiveType::TriangleFan,
                        };
                        // indices reference a single shared vertex buffer
                        let indices = &self.draw_queue.indices[cmd.idx_offset..cmd.idx_offset + cmd.idx_len];
                        let index_buf = glium::IndexBuffer::new(&self.display, mode, indices).unwrap();
                        // get texture to use
                        let texture = cmd
                            .texture
                            .as_ref()
                            .and_then(|img| self.texture_map.get(img))
                            .unwrap_or(&self.t_white);
                        // settings for the pipeline
                        let uniforms = uniform! {
                            vp_size: win_size.as_pointf().components(),
                            tex: texture.sampled().minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
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
    }

    fn load_textures(&mut self) {
        self.texture_map.remove_expired();

        for cmd in &self.draw_queue.commands {
            if let DrawCommand::Primitives(DrawCmdPrim { texture: Some(image), .. }) = cmd {
                let display = &self.display;
                self.texture_map.entry(image.clone()).or_insert_with(|| match image.get_data() {
                    ImageData::Empty => SrgbTexture2d::empty(display, image.get_size().w, image.get_size().h).unwrap(),
                    ImageData::Bpp8(vec) => {
                        let img = RawImage2d {
                            data: std::borrow::Cow::Borrowed(&vec),
                            width: image.get_size().w,
                            height: image.get_size().h,
                            format: match image.get_format() {
                                PixelFormat::Luma => ClientFormat::U8,
                                PixelFormat::LumaA => ClientFormat::U8U8,
                                PixelFormat::Rgb => ClientFormat::U8U8U8,
                                PixelFormat::Rgba => ClientFormat::U8U8U8U8,
                            },
                        };
                        SrgbTexture2d::new(display, img).unwrap()
                    }
                    ImageData::Bpp16(vec) => {
                        let img = RawImage2d {
                            data: std::borrow::Cow::Borrowed(&vec),
                            width: image.get_size().w,
                            height: image.get_size().h,
                            format: match image.get_format() {
                                PixelFormat::Luma => ClientFormat::U16,
                                PixelFormat::LumaA => ClientFormat::U16U16,
                                PixelFormat::Rgb => ClientFormat::U16U16U16,
                                PixelFormat::Rgba => ClientFormat::U16U16U16U16,
                            },
                        };
                        SrgbTexture2d::new(display, img).unwrap()
                    }
                });
            }
        }
    }
}

impl<T: TopLevel> BackendWindow for GliumWindow<T> {
    fn draw(&mut self) {
        self.draw_queue.clear();
        self.window.draw(&mut self.draw_queue);
        self.load_textures();
        let mut target = self.display.draw();
        self.draw_elements(&mut target);
        target.finish().unwrap();
    }

    fn update(&mut self) {
        self.window.update_layout()
        //TODO: compare `self.cur_attr` with `self.window.get_window_attributes()` to make changes to real window
    }

    fn redraw(&self) {
        self.display.gl_window().window().request_redraw();
    }

    fn get_id(&self) -> WindowId {
        self.display.gl_window().window().id()
    }

    fn push_event(&mut self, event: widgets::event::Event) -> bool {
        use widgets::event::Event;

        match event {
            Event::Resized(size) => {
                self.cur_attr.set_size(size);
                self.window.get_attr_mut().set_size(size);
            }
            Event::Moved(pos) => {
                self.cur_attr.set_position(pos);
                self.window.get_attr_mut().set_position(pos);
            }
            _ => (),
        }
        self.window.push_event(event)
    }
}

impl<T: fmt::Debug> fmt::Debug for GliumWindow<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GliumWindow")
            .field("program", &self.program)
            .field("t_white", &self.t_white)
            .field("texture_map", &self.texture_map)
            .field("draw_queue", &self.draw_queue)
            .field("cur_attr", &self.cur_attr)
            .field("window", &self.window)
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
