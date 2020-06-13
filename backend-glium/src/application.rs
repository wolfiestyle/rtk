use crate::window::GliumWindow;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowId;
use std::collections::HashMap;
use widgets::widget::TopLevel;

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
    pub fn add_window(&mut self, window: T) {
        let gl_win = GliumWindow::new(window, &self.event_loop);
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
                    //TODO: handle close request
                    if let WindowEvent::CloseRequested = event {
                        *cf = ControlFlow::Exit;
                    }

                    if let Some(window) = window_map.get_mut(&window_id) {
                        if window.push_event(event).is_some() {
                            window.redraw();
                        }
                    }
                }
                Event::MainEventsCleared => {
                    for window in window_map.values_mut() {
                        window.update();
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

impl GliumApplication<Box<dyn TopLevel>> {
    #[inline]
    pub fn new_dyn() -> Self {
        Default::default()
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
