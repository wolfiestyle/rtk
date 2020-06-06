use crate::draw::{DrawContext, DrawQueue};
use crate::event::{Event, EventResult};
use crate::geometry::{Pointi, Rect, Size};
use crate::visitor::Visitable;

mod id;
pub use id::WidgetId;
mod window;
pub use window::*;

/// Defines an object that can be drawn and viewed inside a window.
pub trait Widget: Visitable {
    /// Gets the widget id.
    fn get_id(&self) -> WidgetId;

    /// Gets the current position.
    fn get_position(&self) -> Pointi;

    /// Gets the current size.
    fn get_size(&self) -> Size;

    // Gets the drawing bounds of this object.
    fn get_bounds(&self) -> Rect {
        Rect::new(self.get_position(), self.get_size())
    }

    /// Sets the current object position.
    fn set_position(&mut self, position: Pointi);

    /// Update this object's size.
    fn update_size(&mut self, parent_rect: Rect);

    /// Draws the contents of this object.
    //TODO: invalidate mechanics to avoid overdraw
    fn draw(&self, dc: DrawContext);

    /// Handles an event sent to this widget.
    fn handle_event(&mut self, event: &Event) -> EventResult;
}

impl Widget for () {
    #[inline]
    fn get_id(&self) -> WidgetId {
        Default::default()
    }

    #[inline]
    fn get_position(&self) -> Pointi {
        Default::default()
    }

    #[inline]
    fn get_size(&self) -> Size {
        Default::default()
    }

    #[inline]
    fn set_position(&mut self, _position: Pointi) {}

    #[inline]
    fn update_size(&mut self, _parent_rect: Rect) {}

    #[inline]
    fn draw(&self, _dc: DrawContext) {}

    #[inline]
    fn handle_event(&mut self, _event: &Event) -> EventResult {
        EventResult::Pass
    }
}

impl<T: Widget> Widget for Option<T> {
    fn get_id(&self) -> WidgetId {
        self.as_ref().map_or_else(Default::default, Widget::get_id)
    }

    fn get_position(&self) -> Pointi {
        self.as_ref().map(Widget::get_position).unwrap_or_default()
    }

    fn get_size(&self) -> Size {
        self.as_ref().map(Widget::get_size).unwrap_or_default()
    }

    fn get_bounds(&self) -> Rect {
        self.as_ref().map(Widget::get_bounds).unwrap_or_default()
    }

    fn set_position(&mut self, position: Pointi) {
        if let Some(widget) = self {
            widget.set_position(position)
        }
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

    fn handle_event(&mut self, event: &Event) -> EventResult {
        self.as_mut().map_or(EventResult::Pass, |w| w.handle_event(event))
    }
}

/// Defines an object that can be a top level window.
pub trait TopLevel {
    fn get_position(&self) -> Pointi;

    fn set_position(&mut self, position: Pointi);

    fn get_size(&self) -> Size;

    fn set_size(&mut self, size: Size);

    fn update(&mut self);

    fn draw(&self, dq: &mut DrawQueue);

    fn push_event(&mut self, event: Event) -> Option<WidgetId>;

    fn get_window_attributes(&self) -> &WindowAttributes;
}
