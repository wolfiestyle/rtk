use crate::event::{Axis, ButtonState, Event, EventContext, EventResult, KeyModState, MouseButtonsState};
use crate::geometry::{Point, Position, Rect, Size};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

/// Sends an event to all widgets (until consumed).
struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
    consumed: bool,
    ev_res: EventResult,
}

impl Visitor for EventDispatchVisitor {
    type Context = PosContext;

    fn visit_after<W: Widget>(mut self, widget: &mut W, this: &Self::Context) -> Self {
        let ctx = self.ctx.update(this.abs_pos.cast(), this.id, this.parent_id);
        let ev_res = widget.handle_event(&self.event, ctx);
        if ev_res.consumed() {
            self.ctx = ctx;
            self.consumed = true;
            self.ev_res = ev_res;
        }
        self
    }

    fn new_context<W: Widget>(&self, widget: &W, parent_ctx: &Self::Context) -> Option<Self::Context> {
        PosContext::from_parent(parent_ctx, widget)
    }

    #[inline]
    fn finished(&self) -> bool {
        self.consumed
    }
}

/// Sends an event to the widget under cursor.
struct PositionDispatchVisitor {
    event: Event,
    ctx: EventContext,
    consumed: bool,
    ev_res: EventResult,
}

impl Visitor for PositionDispatchVisitor {
    type Context = BoundsContext;

    fn visit_after<W: Widget>(mut self, widget: &mut W, this: &Self::Context) -> Self {
        if self.ctx.abs_pos.inside(this.abs_bounds) {
            let ctx = self.ctx.update(this.abs_bounds.pos.cast(), this.id, this.parent_id);
            let ev_res = widget.handle_event(&self.event, ctx);
            if ev_res.consumed() {
                self.ctx = ctx;
                self.consumed = true;
                self.ev_res = ev_res;
            }
        }
        self
    }

    fn new_context<W: Widget>(&self, widget: &W, parent_ctx: &Self::Context) -> Option<Self::Context> {
        BoundsContext::from_parent(parent_ctx, widget)
    }

    #[inline]
    fn finished(&self) -> bool {
        self.consumed
    }
}

/// Checks what widget is under the cursor.
struct InsideCheckVisitor {
    ctx: EventContext,
    last_inside: WidgetId,
    inside: Option<WidgetId>,
    consumed: bool,
    ev_res: EventResult,
}

impl Visitor for InsideCheckVisitor {
    type Context = BoundsContext;

    fn visit_after<W: Widget>(mut self, widget: &mut W, this: &Self::Context) -> Self {
        let inside = self.ctx.abs_pos.inside(this.abs_bounds);
        if inside {
            if self.last_inside != this.id {
                let ctx = self.ctx.update(this.abs_bounds.pos.cast(), this.id, this.parent_id);
                let ev_res = widget.handle_event(&Event::PointerInside(true), ctx);
                if ev_res.consumed() {
                    self.ctx = ctx;
                    self.consumed = true;
                    self.ev_res = ev_res;
                }
            }
            self.inside = Some(this.id);
        }
        self
    }

    fn new_context<W: Widget>(&self, widget: &W, parent_ctx: &Self::Context) -> Option<Self::Context> {
        BoundsContext::from_parent(parent_ctx, widget)
    }

    #[inline]
    fn finished(&self) -> bool {
        self.inside.is_some()
    }
}

/// Sends an event to a single target.
struct TargetedDispatchVisitor {
    target: WidgetId,
    event: Event,
    ctx: EventContext,
    consumed: bool,
    ev_res: EventResult,
}

impl Visitor for TargetedDispatchVisitor {
    type Context = PosContext;

    fn visit_before<W: Widget>(mut self, widget: &mut W, this: &Self::Context) -> Self {
        if self.target == this.id {
            let ctx = self.ctx.update(this.abs_pos.cast(), this.id, this.parent_id);
            let ev_res = widget.handle_event(&self.event, ctx);
            if ev_res.consumed() {
                self.ctx = ctx;
                self.consumed = true;
                self.ev_res = ev_res;
            }
        }
        self
    }

    fn new_context<W: Widget>(&self, widget: &W, parent_ctx: &Self::Context) -> Option<Self::Context> {
        PosContext::from_parent(parent_ctx, widget)
    }

    #[inline]
    fn finished(&self) -> bool {
        self.consumed
    }
}

/// Sends an event consumed notification to every widget in the tree.
struct BroadcastNotifyVisitor {
    event: Event,
    ctx: EventContext,
}

impl Visitor for BroadcastNotifyVisitor {
    type Context = ();

    fn visit_before<W: Widget>(self, widget: &mut W, _: &Self::Context) -> Self {
        widget.event_consumed(&self.event, &self.ctx);
        self
    }

    fn new_context<W: Widget>(&self, _: &W, _: &Self::Context) -> Option<Self::Context> {
        Some(())
    }
}

/// Sends an event consumed notification to a specific widget.
struct TargetNotifyVisitor {
    target: WidgetId,
    event: Event,
    ctx: EventContext,
}

impl Visitor for TargetNotifyVisitor {
    type Context = ();

    fn visit_before<W: Widget>(mut self, widget: &mut W, _: &Self::Context) -> Self {
        if widget.get_id() == self.target {
            widget.event_consumed(&self.event, &self.ctx);
            self.target = WidgetId::EMPTY;
        }
        self
    }

    fn new_context<W: Widget>(&self, _: &W, _: &Self::Context) -> Option<Self::Context> {
        Some(())
    }

    #[inline]
    fn finished(&self) -> bool {
        self.target == WidgetId::EMPTY
    }
}

fn notify_consumed<W: Widget>(root: &mut W, ev_res: EventResult, event: Event, ctx: EventContext) {
    match ev_res {
        EventResult::ConsumedNotifyBroadcast => {
            let visitor = BroadcastNotifyVisitor { event, ctx };
            root.accept(visitor, &());
        }
        EventResult::ConsumedNotifyTarget(target) => {
            let visitor = TargetNotifyVisitor { target, event, ctx };
            root.accept(visitor, &());
        }
        _ => (),
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
    pub fn dispatch_event<W: Widget>(&mut self, event: Event, parent_size: Size, root: &mut W) -> bool {
        self.update_state(&event);
        let ctx = self.make_context();

        // check if pointer inside/outside changed
        let mut in_res = false;
        let mut outside_target = None;
        match event {
            Event::MouseMoved(Axis::Position(_)) => {
                let visitor = InsideCheckVisitor {
                    ctx,
                    last_inside: self.last_inside.unwrap_or_default(),
                    inside: None,
                    consumed: false,
                    ev_res: EventResult::Pass,
                };
                // the PointerInside event is also dispatched here
                let result = root.accept(visitor, &parent_size.into());

                if result.inside != self.last_inside {
                    outside_target = std::mem::replace(&mut self.last_inside, result.inside);
                    notify_consumed(root, result.ev_res, Event::PointerInside(true), result.ctx);
                    in_res = result.consumed;
                }
            }
            Event::PointerInside(false) => {
                outside_target = self.last_inside.take();
            }
            _ => (),
        };

        // dispatch "outside changed" event
        let out_res = outside_target.map_or(false, |target| {
            let visitor = TargetedDispatchVisitor {
                target,
                event: Event::PointerInside(false),
                ctx,
                consumed: false,
                ev_res: EventResult::Pass,
            };
            let result = root.accept(visitor, &Default::default());
            notify_consumed(root, result.ev_res, Event::PointerInside(false), result.ctx);
            result.consumed
        });

        // dispatch other events
        // TODO: keyboard focus, mouse grab
        let ev_res = match event {
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
                let visitor = EventDispatchVisitor {
                    event,
                    ctx,
                    consumed: false,
                    ev_res: EventResult::Pass,
                };
                let result = root.accept(visitor, &Default::default());
                notify_consumed(root, result.ev_res, result.event, result.ctx);
                result.consumed
            }
            // position dependant events
            Event::MouseMoved(_) | Event::MouseButton(_, _) | Event::FileDropped(_) => {
                let visitor = PositionDispatchVisitor {
                    event: event,
                    ctx,
                    consumed: false,
                    ev_res: EventResult::Pass,
                };
                let result = root.accept(visitor, &parent_size.into());
                notify_consumed(root, result.ev_res, result.event, result.ctx);
                result.consumed
            }
            // already handled
            Event::PointerInside(_) => false,
        };

        ev_res | in_res | out_res
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
            widget: WidgetId::EMPTY,
            parent: WidgetId::EMPTY,
        }
    }
}

#[derive(Default)]
struct PosContext {
    abs_pos: Position,
    vp_orig: Position,
    id: WidgetId,
    parent_id: WidgetId,
}

impl PosContext {
    fn from_parent<W: Widget>(parent: &PosContext, widget: &W) -> Option<Self> {
        Some(Self {
            abs_pos: parent.abs_pos - parent.vp_orig + widget.get_position(),
            vp_orig: widget.viewport_origin(),
            id: widget.get_id(),
            parent_id: parent.id,
        })
    }
}

struct BoundsContext {
    abs_bounds: Rect,
    vp_orig: Position,
    id: WidgetId,
    parent_id: WidgetId,
}

impl BoundsContext {
    fn from_parent<W: Widget>(parent: &BoundsContext, widget: &W) -> Option<Self> {
        widget
            .get_bounds()
            .offset(parent.abs_bounds.pos - parent.vp_orig)
            .clip_inside(parent.abs_bounds)
            .map(|abs_bounds| Self {
                abs_bounds,
                vp_orig: widget.viewport_origin(),
                id: widget.get_id(),
                parent_id: parent.id,
            })
    }
}

impl From<Size> for BoundsContext {
    #[inline]
    fn from(size: Size) -> Self {
        Self {
            abs_bounds: size.into(),
            vp_orig: Default::default(),
            id: WidgetId::EMPTY,
            parent_id: WidgetId::EMPTY,
        }
    }
}
