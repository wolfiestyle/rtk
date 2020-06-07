use crate::event::{AxisValue, Event, EventContext, EventResult};
use crate::geometry::{Pointd, Rect};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

#[derive(Debug)]
pub struct EventDispatcher {
    pub event: Event,
    pub ctx: EventContext,
    pub inside: Option<WidgetId>,
    pub outside: Option<WidgetId>,
}

impl EventDispatcher {
    fn dispatch<W: Widget>(&mut self, widget: &mut W, abs_bounds: Rect) -> EventResult {
        let pos = self.ctx.abs_pos;

        if self.inside.map_or(false, |wid| wid == widget.get_id()) {
            widget.handle_event(&Event::PointerInside(true), self.ctx);
        }
        if self.outside.map_or(false, |wid| wid == widget.get_id()) {
            widget.handle_event(&Event::PointerInside(false), self.ctx);
        }

        //TODO: keyboard focus
        match self.event {
            Event::Keyboard { .. } => widget.handle_event(&self.event, self.ctx),
            Event::Character(_) => widget.handle_event(&self.event, self.ctx),
            Event::MouseMoved(AxisValue::Position(_)) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&Event::MouseMoved(AxisValue::Position(self.ctx.pointer_pos)), self.ctx)
                } else {
                    EventResult::Pass
                }
            }
            Event::MouseMoved(_) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event, self.ctx)
                } else {
                    EventResult::Pass
                }
            }
            Event::MouseButton { .. } => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event, self.ctx)
                } else {
                    EventResult::Pass
                }
            }
            Event::FileDropped(_) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event, self.ctx)
                } else {
                    EventResult::Pass
                }
            }
            _ => EventResult::Pass,
        }
    }
}

impl Visitor for EventDispatcher {
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

pub struct InsideCheck {
    pub pos: Pointd,
}

impl Visitor for InsideCheck {
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
