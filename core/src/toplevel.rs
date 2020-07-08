//! TopLevel type that interfaces with the backend.
use crate::draw::DrawQueue;
use crate::event::Event;

mod window;
pub use window::*;

/// Defines an object that can be a top level window.
pub trait TopLevel {
    fn update_layout(&mut self);

    fn draw(&self, dq: &mut DrawQueue);

    fn push_event(&mut self, event: Event) -> bool;

    fn get_attr(&self) -> &WindowAttributes;

    fn get_attr_mut(&mut self) -> &mut WindowAttributes;
}

impl<T: TopLevel> TopLevel for Box<T> {
    #[inline]
    fn update_layout(&mut self) {
        (**self).update_layout()
    }

    #[inline]
    fn draw(&self, dq: &mut DrawQueue) {
        (**self).draw(dq)
    }

    #[inline]
    fn push_event(&mut self, event: Event) -> bool {
        (**self).push_event(event)
    }

    #[inline]
    fn get_attr(&self) -> &WindowAttributes {
        (**self).get_attr()
    }

    #[inline]
    fn get_attr_mut(&mut self) -> &mut WindowAttributes {
        (**self).get_attr_mut()
    }
}
