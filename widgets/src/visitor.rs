use crate::widget::Widget;

pub trait Visitor {
    type Error;
    type Context;

    fn visit<W: Widget + ?Sized>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error>;

    fn new_context<W: Widget + ?Sized>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context;
}
