use crate::geometry::{Border, Point, Position};

/// Defines the size of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Size {
    /// The object width.
    pub w: u32,
    /// The object height.
    pub h: u32,
}

impl Size {
    #[inline]
    pub const fn new(w: u32, h: u32) -> Self {
        Size { w, h }
    }

    #[inline]
    pub const fn zero() -> Self {
        Size { w: 0, h: 0 }
    }

    #[inline]
    pub const fn one() -> Self {
        Size { w: 1, h: 1 }
    }

    #[inline]
    pub const fn square(side: u32) -> Self {
        Size { w: side, h: side }
    }

    #[inline]
    pub fn is_square(self) -> bool {
        self.w == self.h
    }

    #[inline]
    pub fn is_zero_area(self) -> bool {
        self.w == 0 || self.h == 0
    }

    #[inline]
    pub fn area(self) -> usize {
        self.w as usize * self.h as usize
    }

    #[inline]
    pub fn fits_in(self, other: Size) -> bool {
        self.w <= other.w && self.h <= other.h
    }

    #[inline]
    pub fn get_nonzero(self) -> Option<Self> {
        if self.is_zero_area() {
            None
        } else {
            Some(self)
        }
    }

    #[inline]
    pub fn nonzero_or(self, default: Self) -> Self {
        if self.is_zero_area() {
            default
        } else {
            self
        }
    }

    #[inline]
    pub fn with_width(self, w: u32) -> Self {
        Size { w, h: self.h }
    }

    #[inline]
    pub fn with_height(self, h: u32) -> Self {
        Size { w: self.w, h }
    }

    #[inline]
    pub fn add_border(self, border: Border) -> Self {
        Size {
            w: self.w + border.left + border.right,
            h: self.h + border.top + border.bottom,
        }
    }

    #[inline]
    pub fn remove_border(self, border: Border) -> Self {
        Size {
            w: self.w.saturating_sub(border.left + border.right),
            h: self.h.saturating_sub(border.top + border.bottom),
        }
    }

    #[inline]
    pub fn as_position(self) -> Position {
        Position {
            x: self.w as i32,
            y: self.h as i32,
        }
    }

    #[inline]
    pub fn as_pointf(self) -> Point<f32> {
        Point {
            x: self.w as f32,
            y: self.h as f32,
        }
    }

    #[inline]
    pub fn components(self) -> [u32; 2] {
        [self.w, self.h]
    }

    implement_map!(u32, w, h);
}

impl From<[u32; 2]> for Size {
    #[inline]
    fn from([w, h]: [u32; 2]) -> Self {
        Self { w, h }
    }
}

impl From<(u32, u32)> for Size {
    #[inline]
    fn from((w, h): (u32, u32)) -> Self {
        Self { w, h }
    }
}

implement_ops!(Size, u32);
