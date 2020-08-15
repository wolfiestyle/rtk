use crate::queue::DrawQueue;
use crate::shared_res::SharedResources;
use crate::vertex::Vertex;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowId;
use glium::glutin::{ContextBuilder, GlProfile, Robustness};
use std::fmt;
use widgets::backend::{DrawBackend, Resources, TextureError};
use widgets::draw::{TextSection, TextureId};
use widgets::event::Event;
use widgets::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use widgets::geometry::Rect;
use widgets::image::Image;
use widgets::toplevel::{TopLevel, WindowAttributes};
use widgets_winit::{make_win_builder, BackendWindow};

pub struct GliumWindow<T> {
    display: glium::Display,
    draw_queue: DrawQueue,
    cur_attr: WindowAttributes,
    window: T,
}

impl<T: fmt::Debug> fmt::Debug for GliumWindow<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GliumWindow")
            .field("display", &format_args!("..."))
            .field("draw_queue", &self.draw_queue)
            .field("cur_attr", &self.cur_attr)
            .field("window", &self.window)
            .finish()
    }
}

impl<T: TopLevel> GliumWindow<T> {
    pub fn new(window: T, event_loop: &EventLoop<()>, shared_res: &SharedResources) -> Self {
        let win_attr = window.get_attr();
        let win_builder = make_win_builder(win_attr);
        let shared_win = shared_res.display.gl_window();

        let mut ctx = ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_gl_robustness(Robustness::TryRobustNoResetNotification)
            .with_shared_lists(shared_win.context())
            .with_double_buffer(Some(true));
        ctx.pf_reqs.hardware_accelerated = None;
        ctx.pf_reqs.depth_bits = None;
        ctx.pf_reqs.stencil_bits = None;

        let display = glium::Display::new(win_builder, ctx, event_loop).unwrap();
        drop(shared_win);

        if let Some(pos) = win_attr.position {
            display.gl_window().window().set_outer_position(PhysicalPosition::new(pos.x, pos.y));
        }

        Self {
            draw_queue: Default::default(),
            cur_attr: win_attr.clone(),
            window,
            display,
        }
    }
}

impl<T: TopLevel> BackendWindow<SharedResources> for GliumWindow<T> {
    fn get_id(&self) -> WindowId {
        self.display.gl_window().window().id()
    }

    fn update(&mut self, resources: &mut SharedResources) {
        if self.cur_attr.size.is_zero_area() {
            let size: [u32; 2] = self.display.gl_window().window().inner_size().into();
            self.cur_attr.set_size(size);
        }

        self.window.update_layout(resources);
        //TODO: compare `self.cur_attr` with `self.window.get_window_attributes()` to make changes to real window
    }

    fn draw(&mut self, resources: &mut SharedResources) {
        self.draw_queue.clear();
        self.window.draw(&mut BackendWrapper {
            queue: &mut self.draw_queue,
            resources,
        });
        self.draw_queue.render(&self.display, self.window.get_attr().background, resources);
    }

    fn request_redraw(&self) {
        self.display.gl_window().window().request_redraw();
    }

    fn push_event(&mut self, event: Event) -> bool {
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

struct BackendWrapper<'a> {
    pub queue: &'a mut DrawQueue,
    pub resources: &'a mut SharedResources,
}

impl DrawBackend for BackendWrapper<'_> {
    type Vertex = Vertex;

    #[inline]
    fn draw_triangles<V, I>(&mut self, vertices: V, indices: I, texture: Option<TextureId>, viewport: Rect)
    where
        V: IntoIterator<Item = Self::Vertex>,
        I: IntoIterator<Item = u32>,
    {
        self.queue.push_tris(vertices.into_iter(), indices.into_iter(), texture, viewport)
    }

    #[inline]
    fn draw_text(&mut self, text: TextSection, viewport: Rect) {
        self.queue.push_text(text, viewport)
    }
}

impl Resources for BackendWrapper<'_> {
    #[inline]
    fn load_texture(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError> {
        self.resources.load_texture(id, image)
    }

    #[inline]
    fn load_texture_once(&mut self, id: TextureId, image: &Image) -> Result<(), TextureError> {
        self.resources.load_texture_once(id, image)
    }

    #[inline]
    fn delete_texture(&mut self, id: TextureId) {
        self.resources.delete_texture(id)
    }

    #[inline]
    fn enumerate_fonts(&self) -> Vec<String> {
        self.resources.enumerate_fonts()
    }

    #[inline]
    fn select_font(&self, family_names: &[FontFamily], properties: &FontProperties) -> Option<FontSource> {
        self.resources.select_font(family_names, properties)
    }

    #[inline]
    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError> {
        self.resources.load_font(font_src)
    }
}
