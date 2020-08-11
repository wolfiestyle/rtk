use crate::shared_res::SharedRes;
use crate::window::GliumWindow;
use std::rc::Rc;
use widgets::backend::BackendResources;
use widgets::font::{FontFamily, FontId, FontLoadError, FontProperties, FontSource};
use widgets::toplevel::TopLevel;
use widgets_winit::MainLoop;

#[derive(Debug)]
pub struct GliumApplication<T> {
    main_loop: MainLoop<GliumWindow<T>>,
    shared_res: Rc<SharedRes>,
}

impl<T: TopLevel + 'static> GliumApplication<T> {
    #[inline]
    pub fn new() -> Self {
        let main_loop = MainLoop::new();
        let shared_res = Rc::new(SharedRes::new(&main_loop));

        Self { main_loop, shared_res }
    }

    #[inline]
    pub fn add_window(&mut self, window: T) {
        self.main_loop
            .add_window(GliumWindow::new(window, &self.main_loop, self.shared_res.clone()))
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

impl<T> BackendResources for GliumApplication<T> {
    #[inline]
    fn enumerate_fonts(&self) -> Vec<String> {
        self.shared_res.enumerate_fonts()
    }

    #[inline]
    fn select_font(&self, family_names: &[FontFamily], properties: &FontProperties) -> Option<FontSource> {
        self.shared_res.select_font(family_names, properties)
    }

    #[inline]
    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError> {
        self.shared_res.load_font(font_src)
    }
}
