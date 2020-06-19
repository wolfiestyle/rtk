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

impl Bounds for () {
    #[inline]
    fn get_position(&self) -> Position {
        Default::default()
    }

    #[inline]
    fn get_size(&self) -> Size {
        Default::default()
    }

    #[inline]
    fn set_position(&mut self, _position: Position) {}

    #[inline]
    fn set_size(&mut self, _size: Size) {}

    #[inline]
    fn get_bounds(&self) -> Rect {
        Default::default()
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

impl<T: Bounds> Bounds for Option<T> {
    #[inline]
    fn get_position(&self) -> Position {
        self.as_ref().map(Bounds::get_position).unwrap_or_default()
    }

    #[inline]
    fn get_size(&self) -> Size {
        self.as_ref().map(Bounds::get_size).unwrap_or_default()
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        if let Some(widget) = self {
            widget.set_position(position)
        }
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        if let Some(widget) = self {
            widget.set_size(size)
        }
    }

    #[inline]
    fn get_bounds(&self) -> Rect {
        self.as_ref().map(Bounds::get_bounds).unwrap_or_default()
    }
}

impl<T: Bounds, E> Bounds for Result<T, E> {
    #[inline]
    fn get_position(&self) -> Position {
        self.as_ref().map(Bounds::get_position).unwrap_or_default()
    }

    #[inline]
    fn get_size(&self) -> Size {
        self.as_ref().map(Bounds::get_size).unwrap_or_default()
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        if let Ok(widget) = self {
            widget.set_position(position)
        }
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        if let Ok(widget) = self {
            widget.set_size(size)
        }
    }

    #[inline]
    fn get_bounds(&self) -> Rect {
        self.as_ref().map(Bounds::get_bounds).unwrap_or_default()
    }
}

impl<T: Bounds + ?Sized> Bounds for Box<T> {
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

#[macro_export]
macro_rules! implement_bounds {
    ($type:tt $(< $($gen:ident $(: $bound:tt)? ),+ >)? , pos: $pos:ident , size: $size:ident) => {
        impl $(< $($gen $(: $bound)? ),+ >)? $crate::geometry::Bounds for $type $(<$($gen),+>)? {
            fn get_position(&self) -> $crate::geometry::Position {
                self.$pos
            }

            fn get_size(&self) -> $crate::geometry::Size {
                self.$size
            }

            fn set_position(&mut self, position: $crate::geometry::Position) {
                self.$pos = position;
            }

            fn set_size(&mut self, size: $crate::geometry::Size) {
                self.$size = size;
            }
        }
    };

    ($type:tt $(< $($gen:ident $(: $bound:tt)? ),+ >)? , rect: $rect:ident) => {
        impl $(< $($gen $(: $bound)? ),+ >)? $crate::geometry::Bounds for $type $(<$($gen),+>)? {
            fn get_position(&self) -> $crate::geometry::Position {
                self.$rect.pos
            }

            fn get_size(&self) -> $crate::geometry::Size {
                self.$rect.size
            }

            fn set_position(&mut self, position: $crate::geometry::Position) {
                self.$rect.pos = position;
            }

            fn set_size(&mut self, size: $crate::geometry::Size) {
                self.$rect.size = size;
            }

            fn get_bounds(&self) -> Rect {
                self.$rect
            }
        }
    };
}
