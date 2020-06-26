use crate::geometry::{Position, Rect, Size};

/// Defines the drawing bounds of an object.
pub trait Bounds {
    /// Gets the object position.
    fn get_position(&self) -> Position;

    /// Gets the object size.
    fn get_size(&self) -> Size;

    /// Sets the object position.
    fn set_position(&mut self, position: Position);

    /// Sets the object size.
    fn set_size(&mut self, size: Size);

    // Gets bounds as a Rect.
    fn get_bounds(&self) -> Rect {
        Rect {
            pos: self.get_position(),
            size: self.get_size(),
        }
    }
}

impl Bounds for Rect {
    #[inline]
    fn get_position(&self) -> Position {
        self.pos
    }

    #[inline]
    fn get_size(&self) -> Size {
        self.size
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        self.pos = position;
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    #[inline]
    fn get_bounds(&self) -> Rect {
        *self
    }
}

impl<T: Bounds> Bounds for Box<T> {
    #[inline]
    fn get_position(&self) -> Position {
        (**self).get_position()
    }

    #[inline]
    fn get_size(&self) -> Size {
        (**self).get_size()
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        (**self).set_position(position)
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        (**self).set_size(size)
    }

    #[inline]
    fn get_bounds(&self) -> Rect {
        (**self).get_bounds()
    }
}
