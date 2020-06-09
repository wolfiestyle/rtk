use crate::draw::DrawQueue;
use crate::event::{Event, EventContext};
use crate::geometry::{Position, Size};
use crate::widget::{WidgetId, WindowAttributes};

/// Defines an object that can be a top level window.
pub trait TopLevel {
    fn get_position(&self) -> Position;

    fn set_position(&mut self, position: Position);

    fn get_size(&self) -> Size;

    fn set_size(&mut self, size: Size);

    fn update(&mut self);

    fn draw(&self, dq: &mut DrawQueue);

    fn push_event(&mut self, event: Event, ctx: EventContext) -> Option<WidgetId>;

    fn get_window_attributes(&self) -> &WindowAttributes;
}

impl<T: TopLevel + ?Sized> TopLevel for Box<T> {
    #[inline]
    fn get_position(&self) -> Position {
        (**self).get_position()
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        (**self).set_position(position)
    }

    #[inline]
    fn get_size(&self) -> Size {
        (**self).get_size()
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        (**self).set_size(size)
    }

    #[inline]
    fn update(&mut self) {
        (**self).update()
    }

    #[inline]
    fn draw(&self, dq: &mut DrawQueue) {
        (**self).draw(dq)
    }

    #[inline]
    fn push_event(&mut self, event: Event, ctx: EventContext) -> Option<WidgetId> {
        (**self).push_event(event, ctx)
    }

    #[inline]
    fn get_window_attributes(&self) -> &WindowAttributes {
        (**self).get_window_attributes()
    }
}
