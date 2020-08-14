//! TopLevel type that interfaces with the backend.
use crate::backend::{DrawBackend, Resources};
use crate::event::Event;

mod window;
pub use window::*;

/// Defines an object that can be a top level window.
pub trait TopLevel {
    fn update_layout<R: Resources>(&mut self, resources: &mut R);

    fn draw<B: DrawBackend>(&self, backend: &mut B);

    fn push_event(&mut self, event: Event) -> bool;

    fn get_attr(&self) -> &WindowAttributes;

    fn get_attr_mut(&mut self) -> &mut WindowAttributes;
}
