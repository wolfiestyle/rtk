use crate::event::{Axis, ButtonState, Event, EventContext, EventResult, KeyModState, MouseButtonsState};
use crate::geometry::{Point, Rect, Size};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

/// Sends an event to all widgets (until consumed).
struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
}

impl EventDispatchVisitor {
    fn dispatch<W: Widget>(&mut self, widget: &mut W, abs_pos: Point<f64>) -> EventResult {
        let ctx = EventContext {
            local_pos: self.ctx.local_pos - abs_pos,
            ..self.ctx
        };
        widget.handle_event(&self.event, ctx)
    }
}

impl Visitor for EventDispatchVisitor {
    type Return = WidgetId;
    type Context = Point<f64>;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_pos: &Self::Context) -> Result<(), Self::Return> {
        self.dispatch(widget, *abs_pos)
            .as_opt()
            .map_or(Ok(()), |_| Err(widget.get_id()))
    }

    fn new_context<W: Widget>(&self, child: &W, parent_pos: &Self::Context) -> Option<Self::Context> {
        child.get_position().cast_checked().map(|pos| *parent_pos + pos)
    }
}

/// Sends an event to the widget under cursor.
struct PositionDispatchVisitor {
    event: Event,
    ctx: EventContext,
}

impl PositionDispatchVisitor {
    fn dispatch<W: Widget>(&mut self, widget: &mut W, abs_bounds: Rect) -> EventResult {
        if self.ctx.abs_pos.inside(abs_bounds) {
            let ctx = EventContext {
                local_pos: self.ctx.local_pos - abs_bounds.pos.cast(),
                ..self.ctx
            };
            widget.handle_event(&self.event, ctx)
        } else {
            EventResult::Pass
        }
    }
}

impl Visitor for PositionDispatchVisitor {
    type Return = WidgetId;
    type Context = Rect;

    fn visit<W: Widget>(&mut self, widget: &mut W, viewport: &Self::Context) -> Result<(), Self::Return> {
        self.dispatch(widget, *viewport)
            .as_opt()
            .map_or(Ok(()), |_| Err(widget.get_id()))
    }

    fn new_context<W: Widget>(&self, child: &W, parent_vp: &Self::Context) -> Option<Self::Context> {
        child.get_bounds().offset(parent_vp.pos).clip_inside(*parent_vp)
    }
}

/// Checks what widget is under the cursor.
struct InsideCheckVisitor {
    pos: Point<f64>,
    ctx: EventContext,
    last_inside: WidgetId,
    in_res: EventResult,
}

impl InsideCheckVisitor {
    fn check_inside<W: Widget>(&mut self, widget: &mut W, bounds: Rect) -> bool {
        let inside = self.pos.inside(bounds);
        if inside && self.last_inside != widget.get_id() {
            let ctx = EventContext {
                local_pos: self.pos - bounds.pos.cast(),
                ..self.ctx
            };
            self.in_res = widget.handle_event(&Event::PointerInside(true), ctx);
        }
        inside
    }
}

impl Visitor for InsideCheckVisitor {
    type Return = WidgetId;
    type Context = Rect;

    fn visit<W: Widget>(&mut self, widget: &mut W, viewport: &Self::Context) -> Result<(), Self::Return> {
        if self.check_inside(widget, *viewport) {
            Err(widget.get_id())
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, parent_vp: &Self::Context) -> Option<Self::Context> {
        child.get_bounds().offset(parent_vp.pos).clip_inside(*parent_vp)
    }
}

/// Sends an event to a single target.
struct TargetedDispatchVisitor {
    target: WidgetId,
    event: Event,
    ctx: EventContext,
}

impl Visitor for TargetedDispatchVisitor {
    type Return = EventResult;
    type Context = Point<f64>;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_pos: &Self::Context) -> Result<(), Self::Return> {
        if self.target == widget.get_id() {
            let ctx = EventContext {
                local_pos: self.ctx.local_pos - *abs_pos,
                ..self.ctx
            };
            Err(widget.handle_event(&self.event, ctx))
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, parent_pos: &Self::Context) -> Option<Self::Context> {
        child.get_position().cast_checked().map(|pos| *parent_pos + pos)
    }
}

/// Helper to dispatch toplevel events into a widget tree.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EventDispatcher {
    last_pos: Point<f64>,
    mod_state: KeyModState,
    button_state: MouseButtonsState,
    last_inside: Option<WidgetId>,
}

impl EventDispatcher {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn dispatch_event<W: Widget>(&mut self, event: Event, parent_size: Size, root: &mut W) -> Option<WidgetId> {
        self.update_state(&event);
        let ctx = self.make_context();

        // check if pointer inside/outside changed, also dispatch inside event
        let mut in_res = None;
        let mut outside_target = None;
        match event {
            Event::MouseMoved(Axis::Position(pos)) => {
                let mut visitor = InsideCheckVisitor {
                    pos,
                    ctx,
                    last_inside: self.last_inside.unwrap_or_default(),
                    in_res: Default::default(),
                };
                let inside = visitor.visit_child_rev(root, &parent_size.into()).err();
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
        // TODO: keyboard focus, mouse grab
        let res = match event {
            // position independant events
            Event::Keyboard { .. }
            | Event::Character(_)
            | Event::ModifiersChanged(_)
            | Event::CloseRequest
            | Event::Resized(_)
            | Event::Moved(_)
            | Event::Focused(_)
            | Event::Created
            | Event::Destroyed => {
                let mut visitor = EventDispatchVisitor { event, ctx };
                visitor.visit_child_rev(root, &Default::default()).err()
            }
            // position dependant events
            Event::MouseMoved(_) | Event::MouseButton(_, _) | Event::FileDropped(_) => {
                let mut visitor = PositionDispatchVisitor { event, ctx };
                visitor.visit_child_rev(root, &parent_size.into()).err()
            }
            // already handled
            Event::PointerInside(_) => None,
        };

        res.or(in_res).or(out_res)
    }

    /// Update input state.
    fn update_state(&mut self, event: &Event) {
        match *event {
            Event::MouseMoved(Axis::Position(pos)) => {
                self.last_pos = pos;
            }
            Event::MouseButton(ButtonState::Pressed, button) => {
                self.button_state.set(button);
            }
            Event::MouseButton(ButtonState::Released, button) => {
                self.button_state.unset(button);
            }
            Event::ModifiersChanged(mod_state) => {
                self.mod_state = mod_state;
            }
            _ => (),
        }
    }

    /// Creates an event context
    fn make_context(&self) -> EventContext {
        EventContext {
            timestamp: std::time::Instant::now(),
            local_pos: self.last_pos,
            abs_pos: self.last_pos,
            button_state: self.button_state,
            mod_state: self.mod_state,
        }
    }

    /// Dispatch an event to a single widget.
    fn dispatch_targeted<W: Widget>(&self, target: WidgetId, root: &mut W, event: Event) -> Option<WidgetId> {
        let mut visitor = TargetedDispatchVisitor {
            target,
            event,
            ctx: self.make_context(),
        };
        visitor
            .visit_child(root, &Default::default())
            .err()
            .and_then(EventResult::as_opt)
            .map(|_| target)
    }
}
