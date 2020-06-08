use crate::event::{AxisValue, Event, EventContext, EventResult};
use crate::geometry::{Pointd, Rect};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
    inside_target: Option<WidgetId>,
    outside_target: Option<WidgetId>,
    inout_result: Option<WidgetId>,
}

impl EventDispatchVisitor {
    fn dispatch<W: Widget>(&mut self, widget: &mut W, abs_bounds: Rect) -> EventResult {
        let pos = self.ctx.abs_pos;
        let ctx = EventContext{
            pointer_pos: self.ctx.pointer_pos - abs_bounds.pos.cast().unwrap_or_default(),
            ..self.ctx
        };

        if self.inside_target.map_or(false, |wid| wid == widget.get_id()) {
            if widget.handle_event(&Event::PointerInside(true), ctx).consumed() {
                self.inout_result = Some(widget.get_id());
            }
        }
        if self.outside_target.map_or(false, |wid| wid == widget.get_id()) {
            if widget.handle_event(&Event::PointerInside(false), ctx).consumed() {
                self.inout_result = Some(widget.get_id());
            }
        }

        //TODO: keyboard focus
        match self.event {
            Event::Keyboard { .. } | Event::Character(_) => widget.handle_event(&self.event, ctx),
            Event::MouseMoved(AxisValue::Position(_)) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&Event::MouseMoved(AxisValue::Position(ctx.pointer_pos)), ctx)
                } else {
                    EventResult::Pass
                }
            }
            Event::MouseMoved(_) | Event::MouseButton { .. } | Event::FileDropped(_) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event, ctx)
                } else {
                    EventResult::Pass
                }
            }
            _ => EventResult::Pass,
        }
    }
}

impl Visitor for EventDispatchVisitor {
    type Error = WidgetId;
    type Context = Option<Rect>;

    fn visit<W: Widget>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error> {
        ctx.map_or(Ok(()), |vp| match self.dispatch(widget, vp) {
            EventResult::Pass => Ok(()),
            EventResult::Consumed => Err(widget.get_id()),
        })
    }

    fn new_context<W: Widget>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context {
        parent_ctx.and_then(|vp| child.get_bounds().offset(vp.pos).clip_inside(vp))
    }
}

struct InsideCheckVisitor {
    pos: Pointd,
}

impl Visitor for InsideCheckVisitor {
    type Error = WidgetId;
    type Context = Option<Rect>;

    fn visit<W: Widget>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error> {
        ctx.map_or(Ok(()), |bounds| {
            if self.pos.inside(bounds) {
                Err(widget.get_id())
            } else {
                Ok(())
            }
        })
    }

    fn new_context<W: Widget>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context {
        parent_ctx.and_then(|vp| child.get_bounds().offset(vp.pos).clip_inside(vp))
    }
}

/// Helper to dispatch toplevel events into a widget tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EventDispatcher {
    last_inside: Option<WidgetId>,
}

impl EventDispatcher {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn dispatch_event<W: Widget>(
        &mut self, widget: &mut W, event: Event, ctx: EventContext, parent_vp: Rect,
    ) -> Option<WidgetId> {
        let child_vp = widget.get_bounds().clip_inside(parent_vp);

        let (inside, outside) = match event {
            Event::MouseMoved(AxisValue::Position(pos)) => {
                let inside = widget.accept_rev(&mut InsideCheckVisitor { pos }, child_vp).err();
                if inside != self.last_inside {
                    let outside = self.last_inside;
                    self.last_inside = inside;
                    (inside, outside)
                } else {
                    (None, None)
                }
            }
            Event::PointerInside(false) => {
                let outside = self.last_inside;
                self.last_inside = None;
                (None, outside)
            }
            _ => (None, None),
        };

        let mut dispatcher = EventDispatchVisitor {
            event,
            ctx,
            inside_target: inside,
            outside_target: outside,
            inout_result: None,
        };

        widget
            .accept_rev(&mut dispatcher, child_vp)
            .err()
            .or(dispatcher.inout_result)
    }
}
