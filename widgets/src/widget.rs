use crate::draw::DrawContext;
use crate::event::{Event, EventContext, EventResult};
use crate::geometry::{Bounds, Rect};
use crate::visitor::Visitable;

mod id;
pub use id::*;
mod window;
pub use window::*;

/// Defines an object that can be drawn and viewed inside a window.
pub trait Widget: ObjectId + Bounds + Visitable {
    /// Update the object's layout.
    fn update_layout(&mut self, parent_rect: Rect);

    /// Draws the contents of this object.
    //TODO: invalidate mechanics to avoid overdraw
    fn draw(&self, dc: DrawContext);

    /// Handles an event sent to this widget.
    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult;
}

impl Widget for Rect {
    #[inline]
    fn update_layout(&mut self, _parent_rect: Rect) {}

    #[inline]
    fn draw(&self, _dc: DrawContext) {}

    #[inline]
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
}

impl<T: Widget> Widget for Box<T> {
    #[inline]
    fn update_layout(&mut self, parent_rect: Rect) {
        (**self).update_layout(parent_rect)
    }

    #[inline]
    fn draw(&self, dc: DrawContext) {
        (**self).draw(dc)
    }

    #[inline]
    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        (**self).handle_event(event, ctx)
    }
}

#[macro_export]
macro_rules! make_widget_enum {
    ($(#[$($attr:tt)*])* pub $(($vis:ident))? enum $name:ident { $($type:ident),+ $(,)? }) => {
        $crate::make_widget_enum!(@do_impl $(#[$($attr)*])*; pub, $($vis)?; $name, $($type),+);
    };

    ($(#[$($attr:tt)*])* enum $name:ident { $($type:ident),+ $(,)? }) => {
        $crate::make_widget_enum!(@do_impl $(#[$($attr)*])*;,; $name, $($type),+);
    };

    (@do_impl $(#[$($attr:tt)*])*; $($pub:ident)?, $($vis:ident)?; $name:ident, $($type:ident),+) => {
        $(#[$($attr)*])*
        $($pub)? $(($vis))? enum $name {
            $($type($type)),+
        }

        impl $crate::widget::ObjectId for $name {
            fn get_id(&self) -> $crate::widget::WidgetId {
                match self {
                    $($name::$type(a) => a.get_id(),)+
                }
            }
        }

        impl $crate::geometry::Bounds for $name {
            fn get_position(&self) -> $crate::geometry::Position {
                match self {
                    $($name::$type(a) => a.get_position(),)+
                }
            }

            fn get_size(&self) -> $crate::geometry::Size {
                match self {
                    $($name::$type(a) => a.get_size(),)+
                }
            }

            fn set_position(&mut self, position: $crate::geometry::Position) {
                match self {
                    $($name::$type(a) => a.set_position(position),)+
                }
            }

            fn set_size(&mut self, size: $crate::geometry::Size) {
                match self {
                    $($name::$type(a) => a.set_size(size),)+
                }
            }

            fn get_bounds(&self) -> $crate::geometry::Rect {
                match self {
                    $($name::$type(a) => a.get_bounds(),)+
                }
            }
        }

        impl $crate::visitor::Visitable for $name {
            fn accept<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Return> {
                match self {
                    $($name::$type(a) => a.accept(visitor, ctx),)+
                }
            }

            fn accept_rev<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Return> {
                match self {
                    $($name::$type(a) => a.accept_rev(visitor, ctx),)+
                }
            }
        }

        impl $crate::widget::Widget for $name {
            fn update_layout(&mut self, parent_rect: $crate::geometry::Rect) {
                match self {
                    $($name::$type(a) => a.update_layout(parent_rect),)+
                }
            }

            fn draw(&self, dc: $crate::draw::DrawContext) {
                match self {
                    $($name::$type(a) => a.draw(dc),)+
                }
            }

            fn handle_event(&mut self, event: &$crate::event::Event, ctx: $crate::event::EventContext) -> $crate::event::EventResult {
                match self {
                    $($name::$type(a) => a.handle_event(event, ctx),)+
                }
            }
        }

        $(
            impl From<$type> for $name {
                fn from(val: $type) -> Self {
                    $name::$type(val)
                }
            }
        )+
    }
}
