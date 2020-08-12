use crate::shared_res::SharedResources;
use crate::window::GliumWindow;
use std::ops;
use widgets::toplevel::TopLevel;
use widgets_winit::{BackendWindow, MainLoop};

#[derive(Debug)]
pub struct GliumApplication<T> {
    main_loop: MainLoop<GliumWindow<T>, SharedResources>,
}

impl<T: TopLevel + 'static> GliumApplication<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            main_loop: MainLoop::new(SharedResources::new),
        }
    }

    #[inline]
    pub fn add_window(&mut self, window: T) {
        let mut window = GliumWindow::new(window, &self.main_loop, self);
        window.update(self);
        if window.push_event(widgets::event::Event::Created) {
            window.update(self);
        }
        self.main_loop.add_window(window)
    }

    #[inline]
    pub fn run(self) -> ! {
        self.main_loop.run()
    }
}

impl<T: TopLevel + 'static> Default for GliumApplication<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ops::Deref for GliumApplication<T> {
    type Target = SharedResources;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.main_loop.resources
    }
}

impl<T> ops::DerefMut for GliumApplication<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.main_loop.resources
    }
}
