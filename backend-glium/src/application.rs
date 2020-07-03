use crate::window::GliumWindow;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowId;
use std::collections::HashMap;
use widgets::toplevel::TopLevel;

pub struct GliumApplication<T> {
    event_loop: EventLoop<()>,
    window_map: HashMap<WindowId, GliumWindow<T>>,
}

impl<T: TopLevel + 'static> GliumApplication<T> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn add_window(&mut self, mut window: T) {
        window.update();
        let mut gl_win = GliumWindow::new(window, &self.event_loop);
        if gl_win.window.push_event(widgets::event::Event::Created) {
            gl_win.window.update();
        }
        self.window_map.insert(gl_win.get_id(), gl_win);
    }

    pub fn run(self) -> ! {
        use glium::glutin::event::Event;

        let event_loop = self.event_loop;
        let mut window_map = self.window_map;

        event_loop.run(move |event, _, cf| {
            *cf = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, window_id } => {
                    if let Some(window) = window_map.get_mut(&window_id) {
                        let mut is_close_req = false;
                        let mut window_changed = false;
                        match event {
                            WindowEvent::CloseRequested => {
                                is_close_req = true;
                            }
                            WindowEvent::Moved(_)
                            | WindowEvent::Resized(_)
                            | WindowEvent::Focused(_)
                            | WindowEvent::ScaleFactorChanged { .. }
                            | WindowEvent::ThemeChanged(_) => window_changed = true,
                            _ => (),
                        }

                        let ev_consumed = window.push_event(event);
                        if window_changed || ev_consumed {
                            // event was consumed, update and trigger a redraw
                            window.update();
                            window.redraw();
                        } else if is_close_req {
                            // CloseRequest wasn't consumed, destroy window
                            window.window.push_event(widgets::event::Event::Destroyed);
                            window_map.remove(&window_id);
                        }
                    }
                }
                Event::MainEventsCleared => {
                    if window_map.is_empty() {
                        // no windows left, close the application
                        *cf = ControlFlow::Exit;
                    }
                }
                Event::RedrawRequested(window_id) => {
                    if let Some(window) = window_map.get_mut(&window_id) {
                        window.draw();
                    }
                }
                _ => (),
            }
        })
    }
}

impl<T> Default for GliumApplication<T> {
    #[inline]
    fn default() -> Self {
        Self {
            event_loop: EventLoop::new(),
            window_map: Default::default(),
        }
    }
}
