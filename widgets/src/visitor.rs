use crate::widget::Widget;

/// Defines an action that can be executed on a widget tree.
pub trait Visitor {
    type Return;
    type Context;

    /// Performs the action on the specified widget.
    fn visit<W: Widget>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Return>;

    /// Derives a new context for a child widget.
    fn new_context<W: Widget>(&self, child: &W, parent_ctx: &Self::Context) -> Option<Self::Context>;

    /// Visits a child widget with a new context (using accept).
    #[inline]
    fn visit_child<W: Widget>(&mut self, child: &mut W, parent_ctx: &Self::Context) -> Result<(), Self::Return>
    where
        Self: Sized,
    {
        self.new_context(child, parent_ctx)
            .map_or(Ok(()), |ctx| child.accept(self, &ctx))
    }

    /// Visits a child widget with a new context (using accept_rev).
    #[inline]
    fn visit_child_rev<W: Widget>(&mut self, child: &mut W, parent_ctx: &Self::Context) -> Result<(), Self::Return>
    where
        Self: Sized,
    {
        self.new_context(child, parent_ctx)
            .map_or(Ok(()), |ctx| child.accept_rev(self, &ctx))
    }
}

/// Allows an object to accept visitors.
pub trait Visitable {
    /// Accepts a visitor in forward mode (parent, then child).
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return>;

    /// Accepts a visitor in reverse mode (child, then parent).
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return>;
}

impl Visitable for () {
    #[inline]
    fn accept<V: Visitor>(&mut self, _visitor: &mut V, _ctx: &V::Context) -> Result<(), V::Return> {
        Ok(())
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, _visitor: &mut V, _ctx: &V::Context) -> Result<(), V::Return> {
        Ok(())
    }
}

impl Visitable for crate::geometry::Rect {
    #[inline]
    fn accept<V: Visitor>(&mut self, _visitor: &mut V, _ctx: &V::Context) -> Result<(), V::Return> {
        Ok(())
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, _visitor: &mut V, _ctx: &V::Context) -> Result<(), V::Return> {
        Ok(())
    }
}

impl<T: Visitable> Visitable for Option<T> {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept(visitor, ctx))
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept_rev(visitor, ctx))
    }
}

impl<T: Visitable, E> Visitable for Result<T, E> {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept(visitor, ctx))
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        self.as_mut().map_or(Ok(()), |widget| widget.accept_rev(visitor, ctx))
    }
}

impl<T: Visitable> Visitable for Box<T> {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        (**self).accept(visitor, ctx)
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        (**self).accept_rev(visitor, ctx)
    }
}

/// Helper for implementing Visitable on a widget.
#[macro_export]
macro_rules! implement_visitable {
    // no child
    ($type:tt $(< $($gen:ident $(: $bound:tt)? ),+ >)?) => {
        impl  $(< $($gen $(: $bound)? ),+ >)? $crate::visitor::Visitable for $type $(<$($gen),+>)? {
            fn accept<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                visitor.visit(self, ctx)
            }

            fn accept_rev<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                visitor.visit(self, ctx)
            }
        }
    };

    // array child
    ($type:tt $(< $($gen:ident $(: $bound:tt)? ),+ >)? , $field:ident [] ) => {
        impl  $(< $($gen $(: $bound)? ),+ >)? $crate::visitor::Visitable for $type $(<$($gen),+>)? {
            fn accept<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                visitor.visit(self, &ctx)?;
                for child in &mut self.$field {
                    visitor.visit_child(child, ctx)?;
                }
                Ok(())
            }

            fn accept_rev<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                for child in self.$field.iter_mut().rev() {
                    visitor.visit_child_rev(child, ctx)?;
                }
                visitor.visit(self, ctx)
            }
        }
    };

    // list of childs
    ($type:tt $(< $($gen:ident $(: $bound:tt)? ),+ >)? , $($field:ident),+ ) => {
        impl  $(< $($gen $(: $bound)? ),+ >)? $crate::visitor::Visitable for $type $(<$($gen),+>)? {
            fn accept<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                visitor.visit(self, ctx)?;
                $crate::implement_visitable!(@accept self, visitor, ctx; $($field),+)
            }

            fn accept_rev<V: $crate::visitor::Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
                $crate::implement_visitable!(@accept_rev self, visitor, ctx; $($field),+ ;);
                visitor.visit(self, ctx)
            }
        }
    };

    (@accept $self:ident, $visitor:ident, $ctx:ident ; $field:ident $(, $tail:ident)+) => ({
        $visitor.visit_child(&mut $self.$field, $ctx)?;
        $crate::implement_visitable!(@accept $self, $visitor, $ctx; $($tail),+)
    });

    (@accept $self:ident, $visitor:ident, $ctx:ident ; $field:ident) => {
        $visitor.visit_child(&mut $self.$field, $ctx)
    };

    (@accept_rev $self:ident, $visitor:ident, $ctx:ident ; $field:ident $(, $tail:ident)* ; $($reversed:ident)*) => {
        $crate::implement_visitable!(@accept_rev $self, $visitor, $ctx; $($tail),*; $field $($reversed)*);
    };

    (@accept_rev $self:ident, $visitor:ident, $ctx:ident ; ; $($field:ident)*) => {
        $($visitor.visit_child_rev(&mut $self.$field, $ctx)?;)*
    };
}
