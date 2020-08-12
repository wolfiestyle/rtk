//! Widget type and definitions.
use crate::backend::{BackendResources, DrawBackend};
use crate::draw::DrawContext;
use crate::event::{Event, EventContext, EventResult};
use crate::geometry::{Bounds, Position, Rect};
use crate::visitor::Visitable;

mod id;
pub use id::*;
mod empty;
pub use empty::*;

/// Defines an object that can be drawn and viewed inside a window.
pub trait Widget: ObjectId + Bounds + Visitable {
    /// Update the object's layout.
    fn update_layout<R: BackendResources>(&mut self, parent_rect: Rect, resources: &mut R);

    /// Draws the contents of this object.
    //TODO: invalidate mechanics to avoid overdraw
    fn draw<B: DrawBackend>(&self, dc: DrawContext<B>);

    /// Handles an event sent to this widget.
    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult;

    /// Event consumed notification.
    fn event_consumed(&mut self, event: &Event, ctx: &EventContext);

    /// Coordinate of the widget's viewport origin (top-left).
    ///
    /// The default implementation returns `(0, 0)`.
    fn viewport_origin(&self) -> Position {
        Default::default()
    }
}

impl<T: Widget> Widget for Box<T> {
    #[inline]
    fn update_layout<R: BackendResources>(&mut self, parent_rect: Rect, resources: &mut R) {
        (**self).update_layout(parent_rect, resources)
    }

    #[inline]
    fn draw<B: DrawBackend>(&self, dc: DrawContext<B>) {
        (**self).draw(dc)
    }

    #[inline]
    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        (**self).handle_event(event, ctx)
    }

    #[inline]
    fn event_consumed(&mut self, event: &Event, ctx: &EventContext) {
        (**self).event_consumed(event, ctx)
    }

    #[inline]
    fn viewport_origin(&self) -> Position {
        (**self).viewport_origin()
    }
}
