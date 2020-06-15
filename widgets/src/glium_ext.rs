use crate::draw::{Color, TexCoord, Vertex};
use crate::geometry::Point;
use glium::vertex::{Attribute, AttributeType};

glium::implement_vertex!(Vertex, pos, color, texc);

unsafe impl Attribute for Point<f32> {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}

unsafe impl Attribute for Point<i32> {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::I32I32
    }
}

unsafe impl Attribute for Color {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}

unsafe impl Attribute for TexCoord {
    #[inline]
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}
