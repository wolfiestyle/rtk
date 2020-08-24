//! Visitor pattern for widgets.
use crate::widget::Widget;

/// Defines an action that can be executed on a widget tree.
#[allow(unused_variables)]
pub trait Visitor: Sized {
    type Context;

    fn new_context<W: Widget>(&self, widget: &W, prev_ctx: &Self::Context) -> Option<Self::Context>;

    fn visit_before<W: Widget>(self, widget: &mut W, ctx: &Self::Context) -> Self {
        self
    }

    fn visit_after<W: Widget>(self, widget: &mut W, ctx: &Self::Context) -> Self {
        self
    }

    fn finished(&self) -> bool {
        false
    }
}

/// Allows an object to accept visitors.
pub trait Visitable {
    fn accept<V: Visitor>(&mut self, visitor: V, prev_ctx: &V::Context) -> V;
}

impl Visitable for () {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: V, prev_ctx: &V::Context) -> V {
        if visitor.finished() {
            return visitor;
        }
        if let Some(ctx) = visitor.new_context(self, prev_ctx) {
            visitor.visit_before(self, &ctx).visit_after(self, &ctx)
        } else {
            visitor
        }
    }
}
