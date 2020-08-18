use crate::geometry::{Position, Rect, Size};

/// Defines the drawing bounds of an object.
pub trait Bounds {
    /// Gets the object position.
    fn get_position(&self) -> Position;

    /// Gets the object size.
    fn get_size(&self) -> Size;

    // Gets bounds as a Rect.
    fn get_bounds(&self) -> Rect {
        Rect {
            pos: self.get_position(),
            size: self.get_size(),
        }
    }
}

/// Writable bounds.
pub trait BoundsMut: Bounds {
    /// Sets the object position.
    fn set_position(&mut self, position: Position);

    /// Sets the object size.
    fn set_size(&mut self, size: Size);

    /// Sets the object bounds.
    fn set_bounds(&mut self, bounds: Rect) {
        self.set_position(bounds.pos);
        self.set_size(bounds.size);
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
    fn get_bounds(&self) -> Rect {
        *self
    }
}

impl BoundsMut for Rect {
    #[inline]
    fn set_position(&mut self, position: Position) {
        self.pos = position;
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    #[inline]
    fn set_bounds(&mut self, bounds: Rect) {
        *self = bounds;
    }
}
