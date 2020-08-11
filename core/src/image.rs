//! Image type.
use crate::geometry::Size;
#[cfg(feature = "image")]
use image::{DynamicImage, ImageBuffer, ImageResult, Luma, LumaA, Primitive, Rgb, Rgba};
use std::fmt;
use std::path::Path;

/// An image to be used for drawing operations.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    data: Option<ImageData>,
    size: Size,
    format: PixelFormat,
}

impl Image {
    /// Creates a new image from raw pixels.
    pub fn new(data: impl Into<ImageData>, size: impl Into<Size>, format: PixelFormat) -> Self {
        let mut data = data.into();
        let size = size.into();
        // make sure the buffer is big enough
        let data_len = data.len();
        let expected_len = size.area() * format.num_components();
        if data_len != expected_len {
            data.resize(expected_len);
        }

        Self {
            data: Some(data),
            size,
            format,
        }
    }

    /// Creates a new empty image.
    pub fn new_empty(size: impl Into<Size>, format: PixelFormat) -> Self {
        Self {
            data: None,
            size: size.into(),
            format,
        }
    }

    /// Loads an image from file.
    #[cfg(feature = "image")]
    #[inline]
    pub fn from_file(path: impl AsRef<Path>) -> ImageResult<Self> {
        image::open(path).map(Image::from)
    }

    /// Load an image from memory.
    #[cfg(feature = "image")]
    #[inline]
    pub fn from_bytes(buffer: &[u8]) -> ImageResult<Self> {
        image::load_from_memory(buffer).map(Image::from)
    }

    #[inline]
    pub fn get_data(&self) -> &Option<ImageData> {
        &self.data
    }

    #[inline]
    pub fn get_size(&self) -> Size {
        self.size
    }

    #[inline]
    pub fn get_format(&self) -> PixelFormat {
        self.format
    }
}

#[cfg(feature = "image")]
impl From<DynamicImage> for Image {
    fn from(image: DynamicImage) -> Self {
        match image {
            DynamicImage::ImageLuma8(buf) => buf.into(),
            DynamicImage::ImageLumaA8(buf) => buf.into(),
            DynamicImage::ImageRgb8(buf) => buf.into(),
            DynamicImage::ImageRgba8(buf) => buf.into(),
            DynamicImage::ImageBgr8(_) => image.into_rgb().into(),
            DynamicImage::ImageBgra8(_) => image.into_rgba().into(),
            DynamicImage::ImageLuma16(buf) => buf.into(),
            DynamicImage::ImageLumaA16(buf) => buf.into(),
            DynamicImage::ImageRgb16(buf) => buf.into(),
            DynamicImage::ImageRgba16(buf) => buf.into(),
        }
    }
}

#[cfg(feature = "image")]
impl<T: PixelComponent + Primitive + 'static> From<ImageBuffer<Luma<T>, Vec<T>>> for Image {
    fn from(buf: ImageBuffer<Luma<T>, Vec<T>>) -> Self {
        let size = buf.dimensions();
        Self::new(buf.into_raw(), size, PixelFormat::Luma)
    }
}

#[cfg(feature = "image")]
impl<T: PixelComponent + Primitive + 'static> From<ImageBuffer<LumaA<T>, Vec<T>>> for Image {
    fn from(buf: ImageBuffer<LumaA<T>, Vec<T>>) -> Self {
        let size = buf.dimensions();
        Self::new(buf.into_raw(), size, PixelFormat::LumaA)
    }
}

#[cfg(feature = "image")]
impl<T: PixelComponent + Primitive + 'static> From<ImageBuffer<Rgb<T>, Vec<T>>> for Image {
    fn from(buf: ImageBuffer<Rgb<T>, Vec<T>>) -> Self {
        let size = buf.dimensions();
        Self::new(buf.into_raw(), size, PixelFormat::Rgb)
    }
}

#[cfg(feature = "image")]
impl<T: PixelComponent + Primitive + 'static> From<ImageBuffer<Rgba<T>, Vec<T>>> for Image {
    fn from(buf: ImageBuffer<Rgba<T>, Vec<T>>) -> Self {
        let size = buf.dimensions();
        Self::new(buf.into_raw(), size, PixelFormat::Rgba)
    }
}

/// Raw contents of an image.
#[derive(Clone, PartialEq)]
pub enum ImageData {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    F32(Vec<f32>),
}

impl ImageData {
    fn len(&self) -> usize {
        match self {
            ImageData::U8(v) => v.len(),
            ImageData::U16(v) => v.len(),
            ImageData::U32(v) => v.len(),
            ImageData::F32(v) => v.len(),
        }
    }

    fn resize(&mut self, new_len: usize) {
        match self {
            ImageData::U8(v) => v.resize(new_len, 0),
            ImageData::U16(v) => v.resize(new_len, 0),
            ImageData::U32(v) => v.resize(new_len, 0),
            ImageData::F32(v) => v.resize(new_len, 0.0),
        }
    }
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageData::U8(_) => f.debug_tuple("U8").field(&format_args!("[...]")).finish(),
            ImageData::U16(_) => f.debug_tuple("U16").field(&format_args!("[...]")).finish(),
            ImageData::U32(_) => f.debug_tuple("U32").field(&format_args!("[...]")).finish(),
            ImageData::F32(_) => f.debug_tuple("F32").field(&format_args!("[...]")).finish(),
        }
    }
}

impl<T: PixelComponent> From<Vec<T>> for ImageData {
    #[inline]
    fn from(data: Vec<T>) -> Self {
        T::image_data_from(data)
    }
}

impl<T: PixelComponent> From<Box<[T]>> for ImageData {
    #[inline]
    fn from(data: Box<[T]>) -> Self {
        T::image_data_from(data.into_vec())
    }
}

impl<T: PixelComponent> From<&[T]> for ImageData {
    #[inline]
    fn from(data: &[T]) -> Self {
        T::image_data_from(data.to_vec())
    }
}

/// Pixel format of an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    Luma,
    LumaA,
    Rgb,
    Rgba,
}

impl PixelFormat {
    #[inline]
    pub fn num_components(self) -> usize {
        match self {
            PixelFormat::Luma => 1,
            PixelFormat::LumaA => 2,
            PixelFormat::Rgb => 3,
            PixelFormat::Rgba => 4,
        }
    }
}

pub trait PixelComponent: Copy {
    fn image_data_from(data: Vec<Self>) -> ImageData;
}

impl PixelComponent for u8 {
    #[inline]
    fn image_data_from(data: Vec<Self>) -> ImageData {
        ImageData::U8(data)
    }
}

impl PixelComponent for u16 {
    #[inline]
    fn image_data_from(data: Vec<Self>) -> ImageData {
        ImageData::U16(data)
    }
}

impl PixelComponent for u32 {
    #[inline]
    fn image_data_from(data: Vec<Self>) -> ImageData {
        ImageData::U32(data)
    }
}

impl PixelComponent for f32 {
    #[inline]
    fn image_data_from(data: Vec<Self>) -> ImageData {
        ImageData::F32(data)
    }
}
