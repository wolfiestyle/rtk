use crate::event::{AxisValue, EvData, Event, EventResult};
use crate::geometry::Rect;
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

#[derive(Debug)]
pub struct EventDispatcher {
    event: Event,
}

impl EventDispatcher {
    pub fn new(event: Event) -> Self {
        EventDispatcher { event }
    }

    fn dispatch<W: Widget + ?Sized>(&self, widget: &mut W, abs_bounds: Rect) -> EventResult {
        let pos = self.event.abs_pos;

        //TODO: keyboard focus, proper inside/outside
        match self.event.data {
            EvData::Keyboard { .. } => widget.handle_event(&self.event),
            EvData::Character(_) => widget.handle_event(&self.event),
            EvData::MouseMoved(AxisValue::Position(_)) => {
                if pos.inside(abs_bounds) {
                    /*if !self.was_inside {
                        self.was_inside = true;
                        widget.handle_event(&self.event.with_data(EvData::PointerInside(true)))?;
                    }*/
                    widget.handle_event(
                        &self
                            .event
                            .with_data(EvData::MouseMoved(AxisValue::Position(self.event.pointer_pos))),
                    )
                } else {
                    /*if self.was_inside {
                        self.was_inside = false;
                        widget.handle_event(&self.event.with_data(EvData::PointerInside(false)))
                    } else {
                        Ok(())
                    }*/
                    EventResult::Pass
                }
            }
            EvData::MouseMoved(_) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event)
                } else {
                    EventResult::Pass
                }
            }
            EvData::MouseButton { .. } => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event)
                } else {
                    EventResult::Pass
                }
            }
            /*EvData::PointerInside(_) => {
                if self.was_inside {
                    self.was_inside = false;
                    widget.handle_event(&self.event)
                } else {
                    EventResult::Pass
                }
            }*/
            EvData::FileDropped(_) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&self.event)
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

    fn visit<W: Widget + ?Sized>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error> {
        ctx.map_or(Ok(()), |vp| match self.dispatch(widget, vp) {
            EventResult::Pass => Ok(()),
            EventResult::Consumed => Err(widget.get_id()),
        })
    }

    fn new_context<W: Widget + ?Sized>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context {
        parent_ctx.and_then(|vp| child.get_bounds().offset(vp.pos).clip_inside(vp))
    }
}
