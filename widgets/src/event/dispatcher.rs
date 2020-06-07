use crate::event::{AxisValue, Event, EventContext, EventResult};
use crate::geometry::Rect;
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

#[derive(Debug)]
pub struct EventDispatcher {
    pub event: Event,
    pub ctx: EventContext,
    pub last_inside: Option<WidgetId>,
    pub inside: Option<WidgetId>,
}

impl EventDispatcher {
    fn dispatch<W: Widget>(&mut self, widget: &mut W, abs_bounds: Rect) -> EventResult {
        let pos = self.ctx.abs_pos;

        //TODO: keyboard focus, proper inside/outside
        match self.event {
            Event::Keyboard { .. } => widget.handle_event(&self.event, self.ctx),
            Event::Character(_) => widget.handle_event(&self.event, self.ctx),
            Event::MouseMoved(AxisValue::Position(_)) => {
                let my_id = Some(widget.get_id());
                if pos.inside(abs_bounds) {
                    if self.inside.is_none() {
                        self.inside = my_id;
                    }
                    /*if !self.was_inside {
                        self.was_inside = true;
                        widget.handle_event(&self.event.with_data(Event::PointerInside(true)))?;
                    }*/
                    widget.handle_event(&Event::MouseMoved(AxisValue::Position(self.ctx.pointer_pos)), self.ctx)
                } else {
                    /*if self.was_inside {
                        self.was_inside = false;
                        widget.handle_event(&self.event.with_data(Event::PointerInside(false)))
                    } else {
                        Ok(())
                    }*/
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
            /*Event::PointerInside(_) => {
                if self.was_inside {
                    self.was_inside = false;
                    widget.handle_event(&self.event)
                } else {
                    EventResult::Pass
                }
            }*/
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
