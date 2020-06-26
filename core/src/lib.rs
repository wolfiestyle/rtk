//! Widget library.
#[macro_use]
mod macros;

pub mod draw;
pub mod event;
pub mod geometry;
pub mod image;
pub mod layout;
pub mod prelude;
pub mod toplevel;
pub mod visitor;
pub mod widget;

#[cfg(feature = "glium")]
mod glium_ext;

pub use toplevel::DEFAULT_WINDOW_SIZE;