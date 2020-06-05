use crate::widget::Widget;

pub trait Visitor {
    type Error;

    fn visit<W: Widget + ?Sized>(&mut self, widget: &W) -> Result<(), Self::Error>;
}
