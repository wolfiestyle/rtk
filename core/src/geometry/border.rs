use crate::geometry::{Rect, Size};

/// Defines the borders of a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Border {
    /// The left border width.
    pub left: u32,
    /// The right border width.
    pub right: u32,
    /// The top border height.
    pub top: u32,
    /// The bottom border height.
    pub bottom: u32,
}

impl Border {
    /// Creates a border with all sides of the same width.
    #[inline]
    pub const fn all(width: u32) -> Self {
        Border {
            left: width,
            right: width,
            top: width,
            bottom: width,
        }
    }

    /// Creates a border with symmetric sides.
    #[inline]
    pub const fn symmetric(h_width: u32, v_width: u32) -> Self {
        Border {
            left: h_width,
            right: h_width,
            top: v_width,
            bottom: v_width,
        }
    }

    /// The total width of horizontal borders.
    #[inline]
    pub fn total_width(self) -> u32 {
        self.left + self.right
    }

    /// The total height of vertical borders.
    #[inline]
    pub fn total_height(self) -> u32 {
        self.top + self.bottom
    }

    /// Obtains the components as an array.
    /// The returned array contains `[left, right, top, bottom]`.
    #[inline]
    pub fn components(self) -> [u32; 4] {
        [self.left, self.right, self.top, self.bottom]
    }

    /// Applies this border definition to the specified Rect.
    ///
    /// This produces up to four non-overlapping rectangles.
    pub fn calc_rects<F>(&self, bounds: Rect, mut return_rect: F)
    where
        F: FnMut(Rect),
    {
        if self.top > 0 {
            return_rect(Rect {
                pos: bounds.pos,
                size: bounds.size.with_height(self.top),
            });
        }

        if self.left > 0 {
            return_rect(Rect {
                pos: bounds.pos.offset(0, self.top as i32),
                size: Size {
                    w: self.left,
                    h: bounds.size.h - self.top,
                },
            });
        }

        if self.right > 0 {
            return_rect(Rect {
                pos: bounds.pos.offset((bounds.size.w - self.right) as i32, self.top as i32),
                size: Size {
                    w: self.right,
                    h: bounds.size.h - self.top,
                },
            });
        }

        if self.bottom > 0 {
            return_rect(Rect {
                pos: bounds
                    .pos
                    .offset(self.left as i32, (bounds.size.h - self.bottom) as i32),
                size: Size {
                    w: bounds.size.w - self.left - self.right,
                    h: self.bottom,
                },
            });
        }
    }
}
