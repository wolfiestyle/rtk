use crate::queue::DrawQueue;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowId;
use glium::glutin::{ContextBuilder, GlProfile, Robustness};
use widgets::event::Event;
use widgets::toplevel::{TopLevel, WindowAttributes};
use widgets_winit::{make_win_builder, BackendWindow};

#[derive(Debug)]
pub struct GliumWindow<T> {
    draw_queue: DrawQueue,
    cur_attr: WindowAttributes,
    window: T,
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

        let draw_queue = DrawQueue::new(display);

        Self {
            draw_queue,
            cur_attr: win_attr.clone(),
            window,
        }
    }

    fn display(&self) -> &glium::Display {
        &self.draw_queue.display
    }
}

impl<T: TopLevel> BackendWindow for GliumWindow<T> {
    fn get_id(&self) -> WindowId {
        self.display().gl_window().window().id()
    }

    fn update(&mut self) {
        self.window.update_layout()
        //TODO: compare `self.cur_attr` with `self.window.get_window_attributes()` to make changes to real window
    }

    fn draw(&mut self) {
        self.draw_queue.clear();
        self.window.draw(&mut self.draw_queue);
        self.draw_queue.execute(self.cur_attr.size);
    }

    fn request_redraw(&self) {
        self.display().gl_window().window().request_redraw();
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
