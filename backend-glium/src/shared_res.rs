use crate::vertex::TextVertex;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::{ContextBuilder, GlProfile, Robustness};
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d, SrgbTexture2d, Texture2d, TextureCreationError};
use glyph_brush::ab_glyph::FontVec;
use glyph_brush::{Extra, FontId, GlyphBrush, GlyphBrushBuilder};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use widgets::backend::BackendResources;
use widgets::draw::TextureId;
use widgets::font::{FontLoadError, FontSource};
use widgets::image::{Image, ImageData, PixelFormat};

pub struct SharedResources {
    pub(crate) display: glium::Display,
    pub(crate) t_white: SrgbTexture2d,
    pub(crate) program: glium::Program,
    pub(crate) font_src: SystemSource,
    pub(crate) texture_map: HashMap<TextureId, SrgbTexture2d>,
    loaded_fonts: HashMap<FontSource, FontId>,
    pub(crate) glyph_brush: GlyphBrush<TextVertex, Extra, FontVec>,
    pub(crate) font_tex: FontTex,
    pub(crate) text_prog: glium::Program,
}

impl SharedResources {
    pub(crate) fn new(event_loop: &EventLoop<()>) -> Self {
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
        let t_white = SrgbTexture2d::with_mipmaps(&display, image, MipmapsOption::NoMipmap)
            .unwrap()
            .into();

        let vert_src = include_str!("standard.vert.glsl");
        let frag_src = include_str!("standard.frag.glsl");
        let program = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();

        let vert_src = include_str!("text.vert.glsl");
        let frag_src = include_str!("text.frag.glsl");
        let text_prog = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();

        let glyph_brush = GlyphBrushBuilder::using_fonts(vec![]).cache_redraws(false).build();

        let font_tex = FontTex::new(&display, glyph_brush.texture_dimensions()).unwrap();

        let mut this = Self {
            display,
            t_white,
            program,
            font_src: SystemSource::new(),
            texture_map: Default::default(),
            loaded_fonts: Default::default(),
            glyph_brush: glyph_brush.into(),
            font_tex,
            text_prog,
        };

        let default_font = this.select_font(&[FamilyName::SansSerif], &Default::default()).unwrap();
        this.load_font(&default_font).unwrap();

        this
    }
}

impl BackendResources for SharedResources {
    fn load_texture(&mut self, id: TextureId, image: &Image) {
        let texture = to_glium_texture(image, &self.display).unwrap();
        self.texture_map.insert(id, texture);
    }

    fn enumerate_fonts(&self) -> Vec<String> {
        self.font_src.all_families().unwrap()
    }

    fn select_font(&self, family_names: &[FamilyName], properties: &Properties) -> Option<FontSource> {
        self.font_src
            .select_best_match(family_names, properties)
            .ok()
            .map(from_fontkit_handle)
    }

    fn load_font(&mut self, font_src: &FontSource) -> Result<FontId, FontLoadError> {
        if let Some(font_id) = self.loaded_fonts.get(font_src) {
            Ok(*font_id)
        } else {
            let data = std::fs::read(&font_src.path)?;
            let font = FontVec::try_from_vec_and_index(data, font_src.font_index).map_err(|_| FontLoadError::InvalidData)?;
            let id = self.glyph_brush.add_font(font);
            self.loaded_fonts.insert(font_src.clone(), id);
            Ok(id)
        }
    }
}

// pls implement Debug on your types..
impl fmt::Debug for SharedResources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SharedResources")
            .field("display", &format_args!("..."))
            .field("t_white", &self.t_white)
            .field("program", &self.program)
            .field("font_src", &format_args!("..."))
            .field("texture_map", &self.texture_map)
            .field("loaded_fonts", &self.loaded_fonts)
            .field("glyph_brush", &self.glyph_brush)
            .field("font_tex", &self.font_tex)
            .field("text_prog", &self.text_prog)
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct FontTex(pub Texture2d);

impl FontTex {
    fn new(display: &glium::Display, (w, h): (u32, u32)) -> Result<Self, TextureCreationError> {
        Texture2d::empty_with_format(display, glium::texture::UncompressedFloatFormat::U8, MipmapsOption::NoMipmap, w, h).map(FontTex)
    }

    #[inline]
    pub fn update(&self, rect: glyph_brush::Rectangle<u32>, data: &[u8]) {
        let rect = glium::Rect {
            left: rect.min[0],
            bottom: rect.min[1], // bottom is the new top
            width: rect.width(),
            height: rect.height(),
        };
        let img = RawImage2d {
            data: Cow::Borrowed(data),
            width: rect.width,
            height: rect.height,
            format: ClientFormat::U8,
        };
        self.0.write(rect, img);
    }
}

impl Deref for FontTex {
    type Target = Texture2d;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
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

fn from_fontkit_handle(handle: font_kit::handle::Handle) -> FontSource {
    match handle {
        font_kit::handle::Handle::Path { path, font_index } => FontSource { path, font_index },
        _ => unimplemented!(), // font selection only returns paths AFAIK
    }
}
