use crate::event::{Axis, ButtonState, Event, EventContext, EventResult, KeyModState, MouseButtonsState};
use crate::geometry::{Point, Rect, Size};
use crate::visitor::Visitor;
use crate::widget::{Widget, WidgetId};

/// Sends an event to all widgets (until consumed).
struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
}

impl Visitor for EventDispatchVisitor {
    type Return = WidgetId;
    type Context = Point<f64>;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_pos: &Self::Context) -> Result<(), Self::Return> {
        widget
            .handle_event(&self.event, self.ctx.adj_local_pos(*abs_pos))
            .then(|| widget.get_id())
            .map_or(Ok(()), |id| Err(id))
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

impl Visitor for PositionDispatchVisitor {
    type Return = WidgetId;
    type Context = Rect;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_bounds: &Self::Context) -> Result<(), Self::Return> {
        if self.ctx.abs_pos.inside(*abs_bounds) {
            widget
                .handle_event(&self.event, self.ctx.adj_local_pos(abs_bounds.pos.cast()))
                .then(|| widget.get_id())
                .map_or(Ok(()), |id| Err(id))
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, parent_vp: &Self::Context) -> Option<Self::Context> {
        child.get_bounds().offset(parent_vp.pos).clip_inside(*parent_vp)
    }
}

/// Checks what widget is under the cursor.
struct InsideCheckVisitor {
    ctx: EventContext,
    last_inside: WidgetId,
    in_res: Option<WidgetId>,
}

impl Visitor for InsideCheckVisitor {
    type Return = WidgetId;
    type Context = Rect;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_bounds: &Self::Context) -> Result<(), Self::Return> {
        let inside = self.ctx.abs_pos.inside(*abs_bounds);
        if inside && self.last_inside != widget.get_id() {
            self.in_res = widget
                .handle_event(&Event::PointerInside(true), self.ctx.adj_local_pos(abs_bounds.pos.cast()))
                .then(|| widget.get_id());
        }
        if inside {
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

impl TargetedDispatchVisitor {
    fn dispatch_to<W: Widget>(target: WidgetId, root: &mut W, event: Event, ctx: EventContext) -> Option<WidgetId> {
        let mut visitor = TargetedDispatchVisitor { target, event, ctx };
        visitor
            .visit_child(root, &Default::default())
            .err()
            .and_then(|res| res.then_some(target))
    }
}

impl Visitor for TargetedDispatchVisitor {
    type Return = EventResult;
    type Context = Point<f64>;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_pos: &Self::Context) -> Result<(), Self::Return> {
        if self.target == widget.get_id() {
            Err(widget.handle_event(&self.event, self.ctx.adj_local_pos(*abs_pos)))
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, parent_pos: &Self::Context) -> Option<Self::Context> {
        child.get_position().cast_checked().map(|pos| *parent_pos + pos)
    }
}

/// Sends an event consumed notification to every widget in the tree.
struct ConsumedNotifyVisitor {
    id: WidgetId,
    event: Event,
    ctx: EventContext,
}

impl ConsumedNotifyVisitor {
    fn notify_consumed<W: Widget>(root: &mut W, id: WidgetId, event: Event, ctx: EventContext) {
        let mut visitor = ConsumedNotifyVisitor { id, event, ctx };
        let _ = visitor.visit_child(root, &Default::default());
    }
}

impl Visitor for ConsumedNotifyVisitor {
    type Return = ();
    type Context = Point<f64>;

    fn visit<W: Widget>(&mut self, widget: &mut W, abs_pos: &Self::Context) -> Result<(), Self::Return> {
        widget.event_consumed(self.id, &self.event, self.ctx.adj_local_pos(*abs_pos));
        Ok(())
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
        let outside_target;
        let in_res = match event {
            Event::MouseMoved(Axis::Position(_)) => {
                let mut visitor = InsideCheckVisitor {
                    ctx,
                    last_inside: self.last_inside.unwrap_or(WidgetId::EMPTY),
                    in_res: None,
                };
                let inside = visitor.visit_child_rev(root, &parent_size.into()).err();
                if inside != self.last_inside {
                    outside_target = self.last_inside;
                    self.last_inside = inside;
                    visitor.in_res
                } else {
                    outside_target = None;
                    None
                }
            }
            Event::PointerInside(false) => {
                outside_target = self.last_inside;
                self.last_inside = None;
                None
            }
            _ => {
                outside_target = None;
                None
            }
        };
        // dispatch "outside changed" event
        let out_res = outside_target
            .and_then(|target| TargetedDispatchVisitor::dispatch_to(target, root, Event::PointerInside(false), ctx));

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
                let mut visitor = EventDispatchVisitor {
                    event: event.clone(),
                    ctx,
                };
                visitor.visit_child_rev(root, &Default::default()).err()
            }
            // position dependant events
            Event::MouseMoved(_) | Event::MouseButton(_, _) | Event::FileDropped(_) => {
                let mut visitor = PositionDispatchVisitor {
                    event: event.clone(),
                    ctx,
                };
                visitor.visit_child_rev(root, &parent_size.into()).err()
            }
            // already handled
            Event::PointerInside(_) => None,
        };

        // send the event consumed notification
        if let Some(id) = in_res {
            ConsumedNotifyVisitor::notify_consumed(root, id, Event::PointerInside(true), ctx)
        }
        if let Some(id) = out_res {
            ConsumedNotifyVisitor::notify_consumed(root, id, Event::PointerInside(false), ctx)
        }
        if let Some(id) = res {
            ConsumedNotifyVisitor::notify_consumed(root, id, event, ctx)
        }

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
}
