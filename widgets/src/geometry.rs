mod point;
pub use point::Point;
mod size;
pub use size::Size;
mod rect;
pub use rect::Rect;
mod border;
pub use border::Border;
mod size_request;
pub use size_request::SizeRequest;

pub type Pointi = Point<i32>;
pub type Pointl = Point<i64>;
pub type Pointf = Point<f32>;
pub type Pointd = Point<f64>;

#[inline]
pub fn point<T>(x: T, y: T) -> Point<T> {
    Point { x, y }
}
