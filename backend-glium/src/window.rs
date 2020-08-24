use crate::queue::DrawQueue;
use crate::shared_res::SharedResources;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowId;
use rtk::event::Event;
use rtk::toplevel::{TopLevel, WindowAttributes};
use rtk_winit::{make_win_builder, BackendWindow};
use std::fmt;

pub struct GliumWindow<T> {
    display: glium::Display,
    cur_attr: WindowAttributes,
    window: T,
}

impl<T: fmt::Debug> fmt::Debug for GliumWindow<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GliumWindow")
            .field("display", &format_args!("..."))
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

        let ctx_builder = SharedResources::ctx_params()
            .with_shared_lists(shared_win.context())
            .with_double_buffer(Some(true));

        let display = glium::Display::new(win_builder, ctx_builder, event_loop).unwrap();
        drop(shared_win);

        if let Some(pos) = win_attr.position {
            display.gl_window().window().set_outer_position(PhysicalPosition::new(pos.x, pos.y));
        }

        Self {
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
        let mut draw_queue = DrawQueue::new(resources);
        self.window.draw(&mut draw_queue);
        draw_queue.render(&self.display, self.window.get_attr().background);
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
