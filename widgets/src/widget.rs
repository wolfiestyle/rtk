use crate::draw::DrawContext;
use crate::event::{Event, EventContext, EventResult};
use crate::geometry::{Bounds, Rect};
use crate::visitor::Visitable;

mod id;
pub use id::WidgetId;
mod toplevel;
pub use toplevel::TopLevel;
mod window;
pub use window::*;

/// Defines an object that can be drawn and viewed inside a window.
pub trait Widget: Bounds + Visitable {
    /// Gets the widget id.
    fn get_id(&self) -> WidgetId;

    /// Update this object's size.
    fn update_size(&mut self, parent_rect: Rect);

    /// Draws the contents of this object.
    //TODO: invalidate mechanics to avoid overdraw
    fn draw(&self, dc: DrawContext);

    /// Handles an event sent to this widget.
    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult;
}

impl Widget for () {
    #[inline]
    fn get_id(&self) -> WidgetId {
        Default::default()
    }

    #[inline]
    fn update_size(&mut self, _parent_rect: Rect) {}

    #[inline]
    fn draw(&self, _dc: DrawContext) {}

    #[inline]
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
}

impl<T: Widget> Widget for Option<T> {
    fn get_id(&self) -> WidgetId {
        self.as_ref().map_or_else(Default::default, Widget::get_id)
    }

    fn update_size(&mut self, parent_rect: Rect) {
        if let Some(widget) = self {
            widget.update_size(parent_rect)
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
    fn get_id(&self) -> WidgetId {
        (**self).get_id()
    }

    #[inline]
    fn update_size(&mut self, parent_rect: Rect) {
        (**self).update_size(parent_rect)
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
