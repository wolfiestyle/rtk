//! Geometry related types.
mod point;
pub use point::Point;
mod size;
pub use size::Size;
mod rect;
pub use rect::Rect;
mod border;
pub use border::Border;
mod bounds;
pub use bounds::Bounds;
mod align;
pub use align::*;

pub type Position = Point<i32>;
