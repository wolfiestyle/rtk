use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::{ContextBuilder, GlProfile, Robustness};
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d, SrgbTexture2d, TextureCreationError};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use widgets::image::{Image, ImageData, ImageId, PixelFormat};

pub struct SharedRes {
    pub display: glium::Display,
    pub t_white: SrgbTexture2d,
    texture_map: RefCell<HashMap<ImageId, Rc<SrgbTexture2d>>>,
    pub program: glium::Program,
}

impl SharedRes {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        // glium doesn't properly support headless yet, so we use a hidden window
        let win_builder = WindowBuilder::new().with_inner_size(PhysicalSize::new(1, 1)).with_visible(false);

        let mut ctx_builder = ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_gl_robustness(Robustness::TryRobustNoResetNotification);
        ctx_builder.pf_reqs.hardware_accelerated = None;
        ctx_builder.pf_reqs.depth_bits = None;
        ctx_builder.pf_reqs.stencil_bits = None;

        let display = glium::Display::new(win_builder, ctx_builder, event_loop).unwrap();

        let image = RawImage2d::from_raw_rgba(vec![255u8; 4], (1, 1));
        let t_white = SrgbTexture2d::with_mipmaps(&display, image, MipmapsOption::NoMipmap).unwrap();

        let vert_src = include_str!("widgets.vert.glsl");
        let frag_src = include_str!("widgets.frag.glsl");
        let program = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();

        Self {
            display,
            t_white,
            texture_map: Default::default(),
            program,
        }
    }

    pub fn load_texture(&self, image: &Image) -> Rc<SrgbTexture2d> {
        let display = &self.display;
        self.texture_map
            .borrow_mut()
            .entry(image.get_id())
            .or_insert_with(|| to_glium_texture(image, display).unwrap().into())
            .clone()
    }
}

impl fmt::Debug for SharedRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SharedRes")
            .field("display", &format_args!("..."))
            .field("t_white", &self.t_white)
            .field("texture_map", &self.texture_map)
            .field("program", &self.program)
            .finish()
    }
}

fn to_glium_texture(image: &Image, display: &glium::Display) -> Result<SrgbTexture2d, TextureCreationError> {
    let (width, height) = image.get_size().into();
    match image.get_data() {
        None => SrgbTexture2d::empty(display, width, height),
        Some(ImageData::U8(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::U8,
                    PixelFormat::LumaA => ClientFormat::U8U8,
                    PixelFormat::Rgb => ClientFormat::U8U8U8,
                    PixelFormat::Rgba => ClientFormat::U8U8U8U8,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
        Some(ImageData::U16(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::U16,
                    PixelFormat::LumaA => ClientFormat::U16U16,
                    PixelFormat::Rgb => ClientFormat::U16U16U16,
                    PixelFormat::Rgba => ClientFormat::U16U16U16U16,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
        Some(ImageData::U32(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::U32,
                    PixelFormat::LumaA => ClientFormat::U32U32,
                    PixelFormat::Rgb => ClientFormat::U32U32U32,
                    PixelFormat::Rgba => ClientFormat::U32U32U32U32,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
        Some(ImageData::F32(vec)) => {
            let img = RawImage2d {
                data: Cow::Borrowed(vec),
                width,
                height,
                format: match image.get_format() {
                    PixelFormat::Luma => ClientFormat::F32,
                    PixelFormat::LumaA => ClientFormat::F32F32,
                    PixelFormat::Rgb => ClientFormat::F32F32F32,
                    PixelFormat::Rgba => ClientFormat::F32F32F32F32,
                },
            };
            SrgbTexture2d::with_mipmaps(display, img, MipmapsOption::NoMipmap)
        }
    }
}
