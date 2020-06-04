use crate::geometry::Size;

/// Defines a Size value with defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SizeRequest {
    pub w: Option<u32>,
    pub h: Option<u32>,
}

impl SizeRequest {
    #[inline]
    pub fn new(w: impl Into<Option<u32>>, h: impl Into<Option<u32>>) -> Self {
        Self {
            w: w.into(),
            h: h.into(),
        }
    }

    #[inline]
    pub fn or(self, other: Self) -> Self {
        Self {
            w: self.w.or(other.w),
            h: self.h.or(other.h),
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: Size) -> Size {
        Size {
            w: self.w.unwrap_or(default.w),
            h: self.h.unwrap_or(default.h),
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> Size
    where
        F: FnOnce() -> Size,
    {
        match (self.w, self.h) {
            (Some(w), Some(h)) => Size { w, h },
            (Some(w), None) => Size { w, h: f().h },
            (None, Some(h)) => Size { w: f().w, h },
            (None, None) => f(),
        }
    }
}

impl From<Size> for SizeRequest {
    #[inline]
    fn from(Size { w, h }: Size) -> Self {
        SizeRequest { w: Some(w), h: Some(h) }
    }
}

impl From<[Option<u32>; 2]> for SizeRequest {
    #[inline]
    fn from([w, h]: [Option<u32>; 2]) -> Self {
        SizeRequest { w, h }
    }
}

impl From<[u32; 2]> for SizeRequest {
    #[inline]
    fn from([w, h]: [u32; 2]) -> Self {
        SizeRequest { w: Some(w), h: Some(h) }
    }
}

impl From<(Option<u32>, Option<u32>)> for SizeRequest {
    #[inline]
    fn from((w, h): (Option<u32>, Option<u32>)) -> Self {
        SizeRequest { w, h }
    }
}

impl From<(Option<u32>, u32)> for SizeRequest {
    #[inline]
    fn from((w, h): (Option<u32>, u32)) -> Self {
        SizeRequest { w, h: Some(h) }
    }
}

impl From<(u32, Option<u32>)> for SizeRequest {
    #[inline]
    fn from((w, h): (u32, Option<u32>)) -> Self {
        SizeRequest { w: Some(w), h }
    }
}

impl From<(u32, u32)> for SizeRequest {
    #[inline]
    fn from((w, h): (u32, u32)) -> Self {
        SizeRequest { w: Some(w), h: Some(h) }
    }
}
