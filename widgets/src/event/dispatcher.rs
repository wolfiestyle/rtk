use crate::event::{AxisValue, ButtonState, EvState, Event, EventContext, EventResult, ModState};
use crate::geometry::{Point, Rect};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
}

impl EventDispatchVisitor {
    fn dispatch<W: Widget>(&mut self, widget: &mut W, abs_bounds: Rect) -> EventResult {
        let pos = self.ctx.abs_pos;
        let ctx = EventContext {
            pointer_pos: self.ctx.pointer_pos - abs_bounds.pos.cast().unwrap_or_default(),
            ..self.ctx
        };

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
            Event::CloseRequest | Event::Created | Event::Destroyed => widget.handle_event(&self.event, ctx),
            _ => EventResult::Pass,
        }
    }
}

impl Visitor for EventDispatchVisitor {
    type Error = WidgetId;
    type Context = Option<Rect>;

    fn visit<W: Widget>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error> {
        ctx.and_then(|vp| self.dispatch(widget, vp).as_opt())
            .map_or(Ok(()), |_| Err(widget.get_id()))
    }

    fn new_context<W: Widget>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context {
        parent_ctx.and_then(|vp| child.get_bounds().offset(vp.pos).clip_inside(vp))
    }
}

struct InsideCheckVisitor {
    pos: Point<f64>,
    ctx: EventContext,
    last_inside: WidgetId,
    in_res: EventResult,
}

impl Visitor for InsideCheckVisitor {
    type Error = WidgetId;
    type Context = Option<Rect>;

    fn visit<W: Widget>(&mut self, widget: &mut W, ctx: &Self::Context) -> Result<(), Self::Error> {
        if let &Some(bounds) = ctx {
            if self.pos.inside(bounds) {
                if self.last_inside != widget.get_id() {
                    let ctx = EventContext {
                        pointer_pos: self.pos - bounds.pos.cast().unwrap_or_default(),
                        ..self.ctx
                    };
                    self.in_res = widget.handle_event(&Event::PointerInside(true), ctx);
                }
                return Err(widget.get_id());
            }
        }
        Ok(())
    }

    fn new_context<W: Widget>(&self, child: &W, parent_ctx: &Self::Context) -> Self::Context {
        parent_ctx.and_then(|vp| child.get_bounds().offset(vp.pos).clip_inside(vp))
    }
}

struct TargetedDispatchVisitor {
    target: WidgetId,
    event: Event,
    ctx: EventContext,
}

impl Visitor for TargetedDispatchVisitor {
    type Error = EventResult;
    type Context = Point<f64>;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_pos: &Self::Context) -> Result<(), Self::Error> {
        if self.target == widget.get_id() {
            let ctx = EventContext {
                pointer_pos: self.ctx.pointer_pos - *abs_pos,
                ..self.ctx
            };
            Err(widget.handle_event(&self.event, ctx))
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, parent_pos: &Self::Context) -> Self::Context {
        *parent_pos + child.get_position().cast().unwrap_or_default()
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

    pub fn dispatch_event<W: Widget>(&mut self, root: &mut W, event: Event, parent_vp: Rect) -> Option<WidgetId> {
        let child_vp = root.get_bounds().clip_inside(parent_vp);
        let ctx = EventContext::new(self.last_pos, self.button_state, self.mod_state);
        self.update_state(&event);

        // check if pointer inside/outside status changed, and dispatch "inside" event
        let mut in_res = None;
        let mut outside_target = None;
        match event {
            Event::MouseMoved(AxisValue::Position(pos)) => {
                let mut visitor = InsideCheckVisitor {
                    pos,
                    ctx,
                    last_inside: self.last_inside.unwrap_or_default(),
                    in_res: Default::default(),
                };
                let inside = root.accept_rev(&mut visitor, child_vp).err();
                if inside != self.last_inside {
                    in_res = visitor.in_res.as_opt().and(inside);
                    outside_target = self.last_inside;
                    self.last_inside = inside;
                }
            }
            Event::PointerInside(false) => {
                outside_target = self.last_inside;
                self.last_inside = None;
            }
            _ => (),
        }
        // dispatch "outside changed" event
        let out_res =
            outside_target.and_then(|target| self.dispatch_targeted(target, root, Event::PointerInside(false)));

        // dispatch other events

        let mut dispatcher = EventDispatchVisitor { event, ctx };
        root.accept_rev(&mut dispatcher, child_vp).err().or(in_res).or(out_res)
    }

    /// Update input state.
    fn update_state(&mut self, event: &Event) {
        match event {
            Event::MouseMoved(AxisValue::Position(pos)) => {
                self.last_pos = *pos;
            }
            Event::MouseButton(EvState::Pressed, button) => {
                self.button_state.set(*button);
            }
            Event::MouseButton(EvState::Released, button) => {
                self.button_state.unset(*button);
            }
            Event::ModifiersChanged(mod_state) => {
                self.mod_state = *mod_state;
            }
            _ => (),
        }
    }

    /// Dispatch an event to a single widget.
    fn dispatch_targeted<W: Widget>(&self, target: WidgetId, root: &mut W, event: Event) -> Option<WidgetId> {
        let mut dispatcher = TargetedDispatchVisitor {
            target,
            event,
            ctx: EventContext::new(self.last_pos, self.button_state, self.mod_state),
        };
        let pos = root.get_position().cast().unwrap_or_default();
        root.accept(&mut dispatcher, pos)
            .err()
            .and_then(EventResult::as_opt)
            .map(|_| target)
    }
}
