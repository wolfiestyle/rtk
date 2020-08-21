use crate::event::{Axis, ButtonState, Event, EventContext, KeyModState, MouseButtonsState};
use crate::geometry::{Point, Position, Rect, Size};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

/// Sends an event to all widgets (until consumed).
struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
    result: Option<EventContext>,
}

impl Visitor for EventDispatchVisitor {
    type Context = (Position, Position, WidgetId, WidgetId);

    fn visit_after<W: Widget>(self, widget: &mut W, &(abs_pos, _, my_id, parent_id): &Self::Context) -> Self {
        let ctx = self.ctx.update(abs_pos.cast(), my_id, parent_id);
        let result = widget.handle_event(&self.event, ctx).then_some(ctx);
        Self { result, ..self }
    }

    fn new_context<W: Widget>(&self, widget: &W, &(parent_pos, parent_orig, parent_id, _): &Self::Context) -> Option<Self::Context> {
        let abs_pos = parent_pos - parent_orig + widget.get_position();
        Some((abs_pos, widget.viewport_origin(), widget.get_id(), parent_id))
    }

    #[inline]
    fn finished(&self) -> bool {
        self.result.is_some()
    }
}

/// Sends an event to the widget under cursor.
struct PositionDispatchVisitor {
    event: Event,
    ctx: EventContext,
    result: Option<EventContext>,
}

impl Visitor for PositionDispatchVisitor {
    type Context = (Rect, Position, WidgetId, WidgetId);

    fn visit_after<W: Widget>(self, widget: &mut W, &(abs_bounds, _, my_id, parent_id): &Self::Context) -> Self {
        if self.ctx.abs_pos.inside(abs_bounds) {
            let ctx = self.ctx.update(abs_bounds.pos.cast(), my_id, parent_id);
            let result = widget.handle_event(&self.event, ctx).then_some(ctx);
            Self { result, ..self }
        } else {
            self
        }
    }

    fn new_context<W: Widget>(&self, widget: &W, &(parent_vp, parent_orig, parent_id, _): &Self::Context) -> Option<Self::Context> {
        widget
            .get_bounds()
            .offset(parent_vp.pos - parent_orig)
            .clip_inside(parent_vp)
            .map(|abs_bounds| (abs_bounds, widget.viewport_origin(), widget.get_id(), parent_id))
    }

    #[inline]
    fn finished(&self) -> bool {
        self.result.is_some()
    }
}

/// Checks what widget is under the cursor.
struct InsideCheckVisitor {
    ctx: EventContext,
    last_inside: WidgetId,
    inside: Option<WidgetId>,
    in_res: Option<EventContext>,
}

impl Visitor for InsideCheckVisitor {
    type Context = (Rect, Position, WidgetId, WidgetId);

    fn visit_after<W: Widget>(self, widget: &mut W, &(abs_bounds, _, my_id, parent_id): &Self::Context) -> Self {
        let inside = self.ctx.abs_pos.inside(abs_bounds);
        if inside {
            let in_res = if self.last_inside != widget.get_id() {
                let ctx = self.ctx.update(abs_bounds.pos.cast(), my_id, parent_id);
                widget.handle_event(&Event::PointerInside(true), ctx).then_some(ctx)
            } else {
                None
            };
            Self {
                inside: Some(widget.get_id()),
                in_res,
                ..self
            }
        } else {
            self
        }
    }

    fn new_context<W: Widget>(&self, widget: &W, &(parent_vp, parent_orig, parent_id, _): &Self::Context) -> Option<Self::Context> {
        widget
            .get_bounds()
            .offset(parent_vp.pos - parent_orig)
            .clip_inside(parent_vp)
            .map(|abs_bounds| (abs_bounds, widget.viewport_origin(), widget.get_id(), parent_id))
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
    result: Option<EventContext>,
}

impl Visitor for TargetedDispatchVisitor {
    type Context = (Position, Position, WidgetId, WidgetId);

    fn visit_before<W: Widget>(self, widget: &mut W, &(abs_pos, _, my_id, parent_id): &Self::Context) -> Self {
        if self.target == my_id {
            let ctx = self.ctx.update(abs_pos.cast(), my_id, parent_id);
            let result = widget.handle_event(&self.event, ctx).then_some(ctx);
            Self { result, ..self }
        } else {
            self
        }
    }

    fn new_context<W: Widget>(&self, widget: &W, &(parent_pos, parent_orig, parent_id, _): &Self::Context) -> Option<Self::Context> {
        let abs_pos = parent_pos - parent_orig + widget.get_position();
        Some((abs_pos, widget.viewport_origin(), widget.get_id(), parent_id))
    }

    #[inline]
    fn finished(&self) -> bool {
        self.result.is_some()
    }
}

/// Sends an event consumed notification to every widget in the tree.
struct ConsumedNotifyVisitor {
    event: Event,
    ctx: EventContext,
}

impl Visitor for ConsumedNotifyVisitor {
    type Context = ();

    fn visit_before<W: Widget>(self, widget: &mut W, _: &Self::Context) -> Self {
        widget.event_consumed(&self.event, &self.ctx);
        self
    }

    fn new_context<W: Widget>(&self, _: &W, _: &Self::Context) -> Option<Self::Context> {
        Some(())
    }
}

fn notify_consumed<W: Widget>(root: &mut W, event: Event, ctx: EventContext) {
    let visitor = ConsumedNotifyVisitor { event, ctx };
    root.accept(visitor, &Default::default());
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

        // check if pointer inside/outside changed, also dispatch inside event
        let mut in_res = None;
        let mut outside_target = None;
        match event {
            Event::MouseMoved(Axis::Position(_)) => {
                let visitor = InsideCheckVisitor {
                    ctx,
                    last_inside: self.last_inside.unwrap_or_default(),
                    inside: None,
                    in_res: None,
                };
                let result = root.accept(visitor, &(parent_size.into(), Default::default(), WidgetId::EMPTY, WidgetId::EMPTY));
                if result.inside != self.last_inside {
                    outside_target = std::mem::replace(&mut self.last_inside, result.inside);
                    in_res = result.in_res;
                }
            }
            Event::PointerInside(false) => {
                outside_target = self.last_inside.take();
            }
            _ => (),
        };
        // dispatch "outside changed" event
        let out_res = outside_target.and_then(|target| {
            let visitor = TargetedDispatchVisitor {
                target,
                event: Event::PointerInside(false),
                ctx,
                result: None,
            };
            root.accept(visitor, &Default::default()).result
        });

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
                let visitor = EventDispatchVisitor {
                    event: event.clone(),
                    ctx,
                    result: None,
                };
                root.accept(visitor, &Default::default()).result
            }
            // position dependant events
            Event::MouseMoved(_) | Event::MouseButton(_, _) | Event::FileDropped(_) => {
                let visitor = PositionDispatchVisitor {
                    event: event.clone(),
                    ctx,
                    result: None,
                };
                root.accept(visitor, &(parent_size.into(), Default::default(), WidgetId::EMPTY, WidgetId::EMPTY))
                    .result
            }
            // already handled
            Event::PointerInside(_) => None,
        };

        // send the event consumed notification
        if let Some(ctx) = in_res {
            notify_consumed(root, Event::PointerInside(true), ctx)
        }
        if let Some(ctx) = out_res {
            notify_consumed(root, Event::PointerInside(false), ctx)
        }
        if let Some(ctx) = res {
            notify_consumed(root, event, ctx)
        }

        res.or(in_res).or(out_res).is_some()
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
