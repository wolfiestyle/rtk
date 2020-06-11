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

pub type Position = Point<i32>;

#[inline]
pub fn point<T>(x: T, y: T) -> Point<T> {
    Point { x, y }
}
