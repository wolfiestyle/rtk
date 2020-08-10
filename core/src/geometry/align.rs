pub use glyph_brush::HorizontalAlign as HAlign;
pub use glyph_brush::VerticalAlign as VAlign;

/// Defines an object's alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Alignment {
    pub horizontal: HAlign,
    pub vertical: VAlign,
}

impl From<HAlign> for Alignment {
    #[inline]
    fn from(horizontal: HAlign) -> Self {
        Alignment {
            horizontal,
            vertical: VAlign::Top,
        }
    }
}

impl From<VAlign> for Alignment {
    #[inline]
    fn from(vertical: VAlign) -> Self {
        Alignment {
            horizontal: HAlign::Left,
            vertical,
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment {
            horizontal: HAlign::Left,
            vertical: VAlign::Top,
        }
    }
}

impl_from_unit_default!(Alignment);

/* TODO: pull request this stuff, or make a wrapper
/// Horizontal alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl HAlign {
    #[inline]
    pub fn flip(self) -> Self {
        match self {
            HAlign::Left => HAlign::Right,
            HAlign::Center => HAlign::Center,
            HAlign::Right => HAlign::Left,
        }
    }

    #[inline]
    pub fn value(self) -> f32 {
        match self {
            HAlign::Left => 0.0,
            HAlign::Center => 0.5,
            HAlign::Right => 1.0,
        }
    }
}

impl Default for HAlign {
    #[inline]
    fn default() -> Self {
        HAlign::Left
    }
}

/// Vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

impl VAlign {
    #[inline]
    pub fn flip(self) -> Self {
        match self {
            VAlign::Top => VAlign::Bottom,
            VAlign::Center => VAlign::Center,
            VAlign::Bottom => VAlign::Top,
        }
    }

    #[inline]
    pub fn value(self) -> f32 {
        match self {
            VAlign::Top => 0.0,
            VAlign::Center => 0.5,
            VAlign::Bottom => 1.0,
        }
    }
}

impl Default for VAlign {
    #[inline]
    fn default() -> Self {
        VAlign::Top
    }
}
*/
