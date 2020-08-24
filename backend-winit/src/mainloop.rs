use crate::event::translate_event;
use std::collections::HashMap;
use std::ops::Deref;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowId;

pub trait BackendWindow<R> {
    fn get_id(&self) -> WindowId;
    fn update(&mut self, resources: &mut R);
    fn draw(&mut self, resources: &mut R);
    fn request_redraw(&self);
    fn push_event(&mut self, event: rtk::event::Event) -> bool;
}

#[derive(Debug)]
pub struct MainLoop<T, R> {
    event_loop: EventLoop<()>,
    window_map: HashMap<WindowId, T>,
    pub resources: R,
}

impl<T: BackendWindow<R> + 'static, R: 'static> MainLoop<T, R> {
    #[inline]
    pub fn new<F>(resource_f: F) -> Self
    where
        F: FnOnce(&EventLoop<()>) -> R,
    {
        let event_loop = EventLoop::new();
        let resources = resource_f(&event_loop);
        Self {
            event_loop,
            window_map: Default::default(),
            resources,
        }
    }

    #[inline]
    pub fn add_window(&mut self, window: T) {
        self.window_map.insert(window.get_id(), window);
    }

    pub fn run(self) -> ! {
        use winit::event::Event;

        let mut window_map = self.window_map;
        let mut resources = self.resources;

        self.event_loop.run(move |event, _, cf| {
            *cf = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, window_id } => {
                    if let Some(window) = window_map.get_mut(&window_id) {
                        let is_close_req = matches!(event, WindowEvent::CloseRequested);
                        let window_changed = matches!(event,
                            WindowEvent::Moved(_)
                            | WindowEvent::Resized(_)
                            | WindowEvent::Focused(_)
                            | WindowEvent::ScaleFactorChanged { .. }
                            | WindowEvent::ThemeChanged(_)
                        );

                        let ev_consumed = translate_event(event).map_or(false, |ev| window.push_event(ev));
                        if window_changed || ev_consumed {
                            // event was consumed, update and trigger a redraw
                            window.update(&mut resources);
                            window.request_redraw();
                        } else if is_close_req {
                            // CloseRequest wasn't consumed, destroy window
                            window.push_event(rtk::event::Event::Destroyed);
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
                        window.draw(&mut resources);
                    }
                }
                _ => (),
            }
        })
    }
}

impl<T, R> Deref for MainLoop<T, R> {
    type Target = EventLoop<()>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.event_loop
    }
}
