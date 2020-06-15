use crate::geometry::Point;
use std::ops;

/// Texture coordinates (in [0, 1] range).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct TexCoord {
    u: f32,
    v: f32,
}

impl TexCoord {
    pub const TOP_LEFT: TexCoord = TexCoord::new(0.0, 0.0);
    pub const TOP_RIGHT: TexCoord = TexCoord::new(1.0, 0.0);
    pub const BOTTOM_LEFT: TexCoord = TexCoord::new(0.0, 1.0);
    pub const BOTTOM_RIGHT: TexCoord = TexCoord::new(1.0, 1.0);

    #[inline]
    pub const fn new(u: f32, v: f32) -> Self {
        TexCoord { u, v }
    }

    #[inline]
    pub fn normalize(self) -> Self {
        TexCoord {
            u: self.u % 1.0,
            v: self.v % 1.0,
        }
    }

    #[inline]
    pub fn components(self) -> [f32; 2] {
        [self.u, self.v]
    }

    #[inline]
    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(f32) -> f32,
    {
        TexCoord {
            u: f(self.u),
            v: f(self.v),
        }
    }

    #[inline]
    pub fn map2<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(f32, f32) -> f32,
    {
        TexCoord {
            u: f(self.u, other.u),
            v: f(self.v, other.v),
        }
    }

    #[inline]
    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut f32),
    {
        f(&mut self.u);
        f(&mut self.v);
    }

    #[inline]
    pub fn map2_mut<F>(&mut self, other: Self, mut f: F)
    where
        F: FnMut(&mut f32, f32),
    {
        f(&mut self.u, other.u);
        f(&mut self.v, other.v);
    }
}

impl From<[f32; 2]> for TexCoord {
    #[inline]
    fn from([u, v]: [f32; 2]) -> Self {
        TexCoord { u, v }
    }
}

impl From<(f32, f32)> for TexCoord {
    #[inline]
    fn from((u, v): (f32, f32)) -> Self {
        TexCoord { u, v }
    }
}

impl From<Point<f32>> for TexCoord {
    #[inline]
    fn from(Point { x: u, y: v }: Point<f32>) -> Self {
        TexCoord { u, v }
    }
}

impl ops::Add for TexCoord {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Add::add)
    }
}

impl ops::Sub for TexCoord {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Sub::sub)
    }
}

impl ops::Mul<f32> for TexCoord {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl ops::Div<f32> for TexCoord {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

impl ops::Rem<f32> for TexCoord {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        self.map(|a| a % rhs)
    }
}

impl ops::AddAssign for TexCoord {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.map2_mut(rhs, ops::AddAssign::add_assign)
    }
}

impl ops::SubAssign for TexCoord {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.map2_mut(rhs, ops::SubAssign::sub_assign)
    }
}

impl ops::MulAssign<f32> for TexCoord {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.map_mut(|a| *a *= rhs)
    }
}

impl ops::DivAssign<f32> for TexCoord {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.map_mut(|a| *a /= rhs)
    }
}

impl ops::RemAssign<f32> for TexCoord {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.map_mut(|a| *a %= rhs)
    }
}
