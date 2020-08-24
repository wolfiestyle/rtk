pub use font_kit::family_name::FamilyName as FontFamily;
pub use font_kit::properties::Properties as FontProperties;
pub use font_kit::properties::Stretch as FontStretch;
pub use font_kit::properties::Style as FontStyle;
pub use font_kit::properties::Weight as FontWeight;
pub use glyph_brush::ab_glyph::PxScale as TextSize;
pub use glyph_brush::FontId;

use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontSource {
    pub path: PathBuf,
    pub font_index: u32,
}

impl<T: Into<PathBuf>> From<T> for FontSource {
    #[inline]
    fn from(path: T) -> Self {
        Self {
            path: path.into(),
            font_index: 0,
        }
    }
}

#[derive(Debug)]
pub enum FontLoadError {
    InvalidData,
    InvalidIndex,
    Io(io::Error),
}

impl From<io::Error> for FontLoadError {
    #[inline]
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl fmt::Display for FontLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidData => write!(f, "Invalid font data"),
            Self::InvalidIndex => write!(f, "Invalid font index"),
            Self::Io(err) => write!(f, "IO error while loading font: {}", err),
        }
    }
}
