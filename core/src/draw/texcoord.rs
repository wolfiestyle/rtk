use crate::geometry::Point;

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

    #[inline]
    pub fn components(self) -> [f32; 2] {
        [self.u, self.v]
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

implement_ops!(TexCoord, f32);
