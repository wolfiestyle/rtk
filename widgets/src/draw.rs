mod color;
pub use color::Color;
mod vertex;
pub use vertex::Vertex;
mod queue;
pub use queue::*;
mod context;
pub use context::DrawContext;

pub type TexCoord = [f32; 2];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image; //TODO: implement. Image reference?
