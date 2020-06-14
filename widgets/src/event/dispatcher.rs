use crate::event::{AxisValue, ButtonState, EvState, Event, EventContext, EventResult, ModState};
use crate::geometry::{Point, Rect};
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
        let ctx = EventContext {
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
            Event::Keyboard { .. } | Event::Character(_) | Event::ModifiersChanged(_) => {
                widget.handle_event(&self.event, ctx)
            }
            Event::MouseMoved(AxisValue::Position(_)) => {
                if pos.inside(abs_bounds) {
                    widget.handle_event(&Event::MouseMoved(AxisValue::Position(ctx.pointer_pos)), ctx)
                } else {
                    EventResult::Pass
                }
            }
            Event::MouseMoved(_) | Event::MouseButton(_, _) | Event::FileDropped(_) => {
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
    pos: Point<f64>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EventDispatcher {
    last_pos: Point<f64>,
    mod_state: ModState,
    button_state: ButtonState,
    last_inside: Option<WidgetId>,
}

impl EventDispatcher {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn dispatch_event<W: Widget>(&mut self, widget: &mut W, event: Event, parent_vp: Rect) -> Option<WidgetId> {
        let child_vp = widget.get_bounds().clip_inside(parent_vp);

        let mut inside_target = None;
        let mut outside_target = None;

        match event {
            Event::MouseMoved(AxisValue::Position(pos)) => {
                self.last_pos = pos;
                let inside = widget.accept_rev(&mut InsideCheckVisitor { pos }, child_vp).err();
                if inside != self.last_inside {
                    inside_target = inside;
                    outside_target = self.last_inside;
                    self.last_inside = inside;
                }
            }
            Event::PointerInside(false) => {
                outside_target = self.last_inside;
                self.last_inside = None;
            }
            Event::MouseButton(EvState::Pressed, button) => {
                self.button_state.set(button);
            }
            Event::MouseButton(EvState::Released, button) => {
                self.button_state.unset(button);
            }
            Event::ModifiersChanged(mod_state) => {
                self.mod_state = mod_state;
            }
            _ => (),
        }

        let mut dispatcher = EventDispatchVisitor {
            event,
            ctx: EventContext::new(self.last_pos, self.button_state, self.mod_state),
            inside_target,
            outside_target,
            inout_result: None,
        };

        widget
            .accept_rev(&mut dispatcher, child_vp)
            .err()
            .or(dispatcher.inout_result)
    }
}
