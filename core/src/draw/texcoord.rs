use crate::geometry::{Point, Rect, Size};

/// Texture coordinates (in [0, 1] range).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct TexCoord {
    pub u: f32,
    pub v: f32,
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

    implement_map!(f32, u, v);
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

impl From<TexCoord> for [f32; 2] {
    #[inline]
    fn from(t: TexCoord) -> Self {
        [t.u, t.v]
    }
}

impl From<TexCoord> for (f32, f32) {
    #[inline]
    fn from(t: TexCoord) -> Self {
        (t.u, t.v)
    }
}

implement_ops!(TexCoord, f32);
impl_from_unit_default!(TexCoord);

/// Texture coordinates of a rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexRect {
    pub top_left: TexCoord,
    pub bot_right: TexCoord,
}

impl TexRect {
    #[inline]
    pub const fn new(top_left: TexCoord, bot_right: TexCoord) -> Self {
        Self { top_left, bot_right }
    }

    pub fn from_rect(rect: impl Into<Rect>, scale: impl Into<Size>) -> Self {
        let rect = rect.into();
        let this = Self {
            top_left: rect.pos.cast().into(),
            bot_right: (rect.pos + rect.size.as_point()).cast().into(),
        };
        this / TexCoord::from(scale.into().as_point())
    }

    #[inline]
    pub fn top_left(&self) -> TexCoord {
        self.top_left
    }

    #[inline]
    pub fn bot_right(&self) -> TexCoord {
        self.bot_right
    }

    #[inline]
    pub fn top_right(&self) -> TexCoord {
        TexCoord {
            u: self.bot_right.u,
            v: self.top_left.v,
        }
    }

    #[inline]
    pub fn bot_left(&self) -> TexCoord {
        TexCoord {
            u: self.top_left.u,
            v: self.bot_right.v,
        }
    }

    implement_map!(TexCoord, top_left, bot_right);
}

impl From<[TexCoord; 2]> for TexRect {
    #[inline]
    fn from([top_left, bot_right]: [TexCoord; 2]) -> Self {
        Self { top_left, bot_right }
    }
}

impl From<(TexCoord, TexCoord)> for TexRect {
    #[inline]
    fn from((top_left, bot_right): (TexCoord, TexCoord)) -> Self {
        Self { top_left, bot_right }
    }
}

impl From<TexRect> for [TexCoord; 2] {
    #[inline]
    fn from(tr: TexRect) -> Self {
        [tr.top_left, tr.bot_right]
    }
}

impl From<TexRect> for (TexCoord, TexCoord) {
    #[inline]
    fn from(tr: TexRect) -> Self {
        (tr.top_left, tr.bot_right)
    }
}

impl Default for TexRect {
    #[inline]
    fn default() -> Self {
        TexRect::new(TexCoord::TOP_LEFT, TexCoord::BOTTOM_RIGHT)
    }
}

impl_from_unit_default!(TexRect);

impl std::ops::Add<TexCoord> for TexRect {
    type Output = Self;

    #[inline]
    fn add(self, rhs: TexCoord) -> Self::Output {
        self.map(|a| a + rhs)
    }
}

impl std::ops::Sub<TexCoord> for TexRect {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: TexCoord) -> Self::Output {
        self.map(|a| a - rhs)
    }
}

impl std::ops::Mul<TexCoord> for TexRect {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: TexCoord) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl std::ops::Div<TexCoord> for TexRect {
    type Output = Self;

    #[inline]
    fn div(self, rhs: TexCoord) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

impl std::ops::Mul<f32> for TexRect {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl std::ops::Div<f32> for TexRect {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.map(|a| a / rhs)
    }
}
