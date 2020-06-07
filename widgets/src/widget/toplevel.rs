use crate::draw::DrawQueue;
use crate::event::{Event, EventContext};
use crate::geometry::{Pointi, Size};
use crate::widget::{WidgetId, WindowAttributes};

/// Defines an object that can be a top level window.
pub trait TopLevel {
    fn get_position(&self) -> Pointi;

    fn set_position(&mut self, position: Pointi);

    fn get_size(&self) -> Size;

    fn set_size(&mut self, size: Size);

    fn update(&mut self);

    fn draw(&self, dq: &mut DrawQueue);

    fn push_event(&mut self, event: Event, ctx: EventContext) -> Option<WidgetId>;

    fn get_window_attributes(&self) -> &WindowAttributes;
}
