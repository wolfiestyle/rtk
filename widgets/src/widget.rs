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

impl Widget for () {
    #[inline]
    fn update_layout(&mut self, _parent_rect: Rect) {}

    #[inline]
    fn draw(&self, _dc: DrawContext) {}

    #[inline]
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
}

impl<T: Widget> Widget for Option<T> {
    fn update_layout(&mut self, parent_rect: Rect) {
        if let Some(widget) = self {
            widget.update_layout(parent_rect)
        }
    }

    fn draw(&self, dc: DrawContext) {
        if let Some(widget) = self {
            widget.draw(dc)
        }
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        self.as_mut().map_or(EventResult::Pass, |w| w.handle_event(event, ctx))
    }
}

impl<T: Widget + ?Sized> Widget for Box<T> {
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
