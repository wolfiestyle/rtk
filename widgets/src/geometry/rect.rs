use crate::geometry::{Border, Point, Pointi, Size};

/// Defines a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Rect {
    /// Position of the top left corner.
    pub pos: Pointi,
    /// Rectangle size.
    pub size: Size,
}

impl Rect {
    #[inline]
    pub fn new(pos: impl Into<Pointi>, size: impl Into<Size>) -> Self {
        Rect {
            pos: pos.into(),
            size: size.into(),
        }
    }

    #[inline]
    pub fn new_centered(center: impl Into<Pointi>, size: impl Into<Size>) -> Self {
        let center = center.into();
        let size = size.into();
        Rect {
            pos: center - size.as_point() / 2,
            size,
        }
    }

    #[inline]
    pub fn new_at_origin(size: impl Into<Size>) -> Self {
        Rect {
            pos: Default::default(),
            size: size.into(),
        }
    }

    #[inline]
    pub fn from_components(x: i32, y: i32, w: u32, h: u32) -> Self {
        Rect {
            pos: Point { x, y },
            size: Size { w, h },
        }
    }

    #[inline]
    pub fn from_coords(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Rect {
            pos: Point {
                x: x0.min(x1),
                y: y0.min(y1),
            },
            size: Size {
                w: (x1 - x0).abs() as u32 + 1,
                h: (y1 - y0).abs() as u32 + 1,
            },
        }
    }

    #[inline]
    pub fn x(self) -> i32 {
        self.pos.x
    }

    #[inline]
    pub fn y(self) -> i32 {
        self.pos.y
    }

    #[inline]
    pub fn w(self) -> u32 {
        self.size.w
    }

    #[inline]
    pub fn h(self) -> u32 {
        self.size.h
    }

    #[inline]
    pub fn end_x(self) -> i32 {
        self.pos.x + self.size.w as i32 - 1
    }

    #[inline]
    pub fn end_y(self) -> i32 {
        self.pos.y + self.size.h as i32 - 1
    }

    #[inline]
    pub fn top_left(self) -> Pointi {
        self.pos
    }

    #[inline]
    pub fn top_right(self) -> Pointi {
        Point {
            x: self.end_x(),
            y: self.y(),
        }
    }

    #[inline]
    pub fn bottom_left(self) -> Pointi {
        Point {
            x: self.x(),
            y: self.end_y(),
        }
    }

    #[inline]
    pub fn bottom_right(self) -> Pointi {
        Point {
            x: self.end_x(),
            y: self.end_y(),
        }
    }

    #[inline]
    pub fn components(self) -> (i32, i32, u32, u32) {
        (self.pos.x, self.pos.y, self.size.w, self.size.h)
    }

    #[inline]
    pub fn contains(self, p: Pointi) -> bool {
        p.x >= self.x() && p.x <= self.end_x() && p.y >= self.y() && p.y <= self.end_y()
    }

    #[inline]
    pub fn intersects(self, other: Rect) -> bool {
        self.x() <= other.end_x() && self.end_x() >= other.x() && self.y() <= other.end_y() && self.end_y() >= other.y()
    }

    #[inline]
    pub fn inside(self, other: Rect) -> bool {
        self.x() >= other.x() && self.end_x() <= other.end_x() && self.y() >= other.y() && self.end_y() <= other.end_y()
    }

    #[inline]
    pub fn moved_to(self, pos: Pointi) -> Self {
        Rect { pos, size: self.size }
    }

    #[inline]
    pub fn resized(self, size: Size) -> Self {
        Rect { pos: self.pos, size }
    }

    #[inline]
    pub fn at_origin(self) -> Self {
        Rect {
            pos: Default::default(),
            size: self.size,
        }
    }

    #[inline]
    pub fn expand_to_origin(self) -> Self {
        Rect {
            pos: Default::default(),
            size: (self.pos + self.size.as_point()).as_size(),
        }
    }

    #[inline]
    pub fn offset(self, dx: i32, dy: i32) -> Self {
        Rect {
            pos: self.pos.offset(dx, dy),
            size: self.size,
        }
    }

    #[inline]
    pub fn add_border(self, border: Border) -> Self {
        Rect {
            pos: self.pos.offset(-(border.left as i32), -(border.top as i32)),
            size: self.size
                + Size {
                    w: border.total_width(),
                    h: border.total_height(),
                },
        }
    }

    #[inline]
    pub fn remove_border(self, border: Border) -> Self {
        Rect {
            pos: self.pos.offset(border.left as i32, border.top as i32),
            size: Size {
                w: self.size.w.saturating_sub(border.total_width()),
                h: self.size.h.saturating_sub(border.total_height()),
            },
        }
    }

    #[inline]
    pub fn clip_inside(self, bounds: Rect) -> Option<Self> {
        if self.intersects(bounds) {
            let dpos = Point {
                x: (bounds.x() - self.x()).max(0),
                y: (bounds.y() - self.y()).max(0),
            };
            let dsize = Size {
                w: ((self.end_x() - bounds.end_x()).max(0) + dpos.x) as u32,
                h: ((self.end_y() - bounds.end_y()).max(0) + dpos.y) as u32,
            };
            Some(Rect {
                pos: self.pos + dpos,
                size: self.size - dsize,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn map_pos<F>(self, f: F) -> Rect
    where
        F: FnOnce(Pointi) -> Pointi,
    {
        Rect {
            pos: f(self.pos),
            size: self.size,
        }
    }

    #[inline]
    pub fn map_size<F>(self, f: F) -> Rect
    where
        F: FnOnce(Size) -> Size,
    {
        Rect {
            pos: self.pos,
            size: f(self.size),
        }
    }
}

impl From<(Pointi, Size)> for Rect {
    #[inline]
    fn from((pos, size): (Pointi, Size)) -> Self {
        Rect { pos, size }
    }
}

impl From<[Pointi; 2]> for Rect {
    #[inline]
    fn from([p0, p1]: [Pointi; 2]) -> Self {
        Rect::from_coords(p0.x, p0.y, p1.x, p1.y)
    }
}

impl From<(Pointi, Pointi)> for Rect {
    #[inline]
    fn from((p0, p1): (Pointi, Pointi)) -> Self {
        Rect::from_coords(p0.x, p0.y, p1.x, p1.y)
    }
}

impl From<Size> for Rect {
    #[inline]
    fn from(size: Size) -> Self {
        Rect::new_at_origin(size)
    }
}

impl From<[i32; 4]> for Rect {
    #[inline]
    fn from([x0, y0, x1, y1]: [i32; 4]) -> Self {
        Rect::from_coords(x0, y0, x1, y1)
    }
}

impl From<([i32; 2], [u32; 2])> for Rect {
    #[inline]
    fn from(([x, y], [w, h]): ([i32; 2], [u32; 2])) -> Self {
        Rect::from_components(x, y, w, h)
    }
}

impl From<((i32, i32), (u32, u32))> for Rect {
    #[inline]
    fn from(((x, y), (w, h)): ((i32, i32), (u32, u32))) -> Self {
        Rect::from_components(x, y, w, h)
    }
}

impl From<(i32, i32, u32, u32)> for Rect {
    #[inline]
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Rect::from_components(x, y, w, h)
    }
}
