use crate::event::translate_event;
use std::collections::HashMap;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowId;

pub trait BackendWindow {
    fn get_id(&self) -> WindowId;
    fn update(&mut self);
    fn draw(&mut self);
    fn redraw(&self);
    fn push_event(&mut self, event: widgets::event::Event) -> bool;
}

pub struct MainLoop<T> {
    pub event_loop: EventLoop<()>,
    window_map: HashMap<WindowId, T>,
}

impl<T: BackendWindow + 'static> MainLoop<T> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn add_window(&mut self, mut window: T) {
        window.update();
        if window.push_event(widgets::event::Event::Created) {
            window.update();
        }
        self.window_map.insert(window.get_id(), window);
    }

    pub fn run(self) -> ! {
        use winit::event::Event;

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

                        let ev_consumed = translate_event(event).map(|ev| window.push_event(ev)).unwrap_or_default();
                        if window_changed || ev_consumed {
                            // event was consumed, update and trigger a redraw
                            window.update();
                            window.redraw();
                        } else if is_close_req {
                            // CloseRequest wasn't consumed, destroy window
                            window.push_event(widgets::event::Event::Destroyed);
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

impl<T> Default for MainLoop<T> {
    #[inline]
    fn default() -> Self {
        Self {
            event_loop: EventLoop::new(),
            window_map: Default::default(),
        }
    }
}
