use crate::window::GliumWindow;
use widgets::toplevel::TopLevel;
use widgets_winit::MainLoop;

#[derive(Debug)]
pub struct GliumApplication<T> {
    main_loop: MainLoop<GliumWindow<T>>,
}

impl<T: TopLevel + 'static> GliumApplication<T> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn add_window(&mut self, window: T) {
        self.main_loop.add_window(GliumWindow::new(window, &self.main_loop.event_loop))
    }

    #[inline]
    pub fn run(self) -> ! {
        self.main_loop.run()
    }
}

impl<T> Default for GliumApplication<T> {
    #[inline]
    fn default() -> Self {
        Self {
            main_loop: Default::default(),
        }
    }
}
