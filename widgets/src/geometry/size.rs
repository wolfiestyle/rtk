use crate::geometry::{Border, Point, Pointf, Pointi};
use std::ops;

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
    pub fn area(self) -> u32 {
        self.w * self.h
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
    pub fn as_point(self) -> Pointi {
        Point {
            x: self.w as i32,
            y: self.h as i32,
        }
    }

    #[inline]
    pub fn as_pointf(self) -> Pointf {
        Point {
            x: self.w as f32,
            y: self.h as f32,
        }
    }

    #[inline]
    pub fn components(self) -> [u32; 2] {
        [self.w, self.h]
    }

    #[inline]
    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(u32) -> u32,
    {
        Size {
            w: f(self.w),
            h: f(self.h),
        }
    }

    #[inline]
    pub fn map2<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(u32, u32) -> u32,
    {
        Size {
            w: f(self.w, other.w),
            h: f(self.h, other.h),
        }
    }

    #[inline]
    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut u32),
    {
        f(&mut self.w);
        f(&mut self.h);
    }

    #[inline]
    pub fn map2_mut<F>(&mut self, other: Self, mut f: F)
    where
        F: FnMut(&mut u32, u32),
    {
        f(&mut self.w, other.w);
        f(&mut self.h, other.h);
    }
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

impl ops::Add for Size {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Add::add)
    }
}

impl ops::Sub for Size {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Sub::sub)
    }
}

impl ops::Mul<u32> for Size {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u32) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl ops::Div<u32> for Size {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u32) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

impl ops::Rem<u32> for Size {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: u32) -> Self::Output {
        self.map(|a| a % rhs)
    }
}

impl ops::AddAssign for Size {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.map2_mut(rhs, ops::AddAssign::add_assign)
    }
}

impl ops::SubAssign for Size {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.map2_mut(rhs, ops::SubAssign::sub_assign)
    }
}

impl ops::MulAssign<u32> for Size {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        self.map_mut(|a| *a *= rhs)
    }
}

impl ops::DivAssign<u32> for Size {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        self.map_mut(|a| *a /= rhs)
    }
}

impl ops::RemAssign<u32> for Size {
    #[inline]
    fn rem_assign(&mut self, rhs: u32) {
        self.map_mut(|a| *a %= rhs)
    }
}
