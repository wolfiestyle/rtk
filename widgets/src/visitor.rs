use crate::widget::Widget;

/// Defines an action that can be executed on a widget tree.
pub trait Visitor {
    type Error;
    type Context;

    /// Performs the action on the specified widget.
    fn visit<W: Widget + ?Sized>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error>;

    /// Derives a new context for a child widget.
    fn new_context<W: Widget + ?Sized>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context;
}

/// Allows an object to accept visitors.
pub trait Visitable {
    /// Accepts a visitor in forward mode (parent, then child).
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error>;

    /// Accepts a visitor in reverse mode (child, then parent).
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error>;
}

impl Visitable for () {
    #[inline]
    fn accept<V: Visitor>(&mut self, _visitor: &mut V, _ctx: V::Context) -> Result<(), V::Error> {
        Ok(())
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, _visitor: &mut V, _ctx: V::Context) -> Result<(), V::Error> {
        Ok(())
    }
}

impl<T: Visitable> Visitable for Option<T> {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept(visitor, ctx))
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept_rev(visitor, ctx))
    }
}

impl<T: Visitable, E> Visitable for Result<T, E> {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept(visitor, ctx))
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept_rev(visitor, ctx))
    }
}

/// Helper for implementing Visitable on a widget.
#[macro_export]
macro_rules! implement_visitable {
    ($type:tt $(< $($gen:ident $(: $bound:tt)? ),+ >)? , $($field:ident),* ) => {
        impl  $(< $($gen $(: $bound)? ),+ >)? $crate::visitor::Visitable for $type $(<$($gen),+>)? {
            fn accept<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
                visitor.visit(self, &ctx)?;
                $(self.$field.accept(visitor, visitor.new_context(&self.$field, &ctx))?;)*
                Ok(())
            }

            fn accept_rev<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
                $(self.$field.accept_rev(visitor, visitor.new_context(&self.$field, &ctx))?;)*
                visitor.visit(self, &ctx)
            }
        }
    };
}
