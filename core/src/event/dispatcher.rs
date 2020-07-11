use crate::event::{Axis, ButtonState, Event, EventContext, KeyModState, MouseButtonsState};
use crate::geometry::{Point, Position, Rect, Size};
use crate::visitor::{ParentData, Visitor};
use crate::widget::{Widget, WidgetId};

/// Sends an event to all widgets (until consumed).
struct EventDispatchVisitor {
    event: Event,
    ctx: EventContext,
}

impl Visitor for EventDispatchVisitor {
    type Return = EventContext;
    type Context = (Position, WidgetId);

    fn visit<W: Widget>(&mut self, widget: &mut W, &(abs_pos, parent_id): &Self::Context) -> Result<(), Self::Return> {
        let ctx = self.ctx.update(abs_pos.cast(), widget.get_id(), parent_id);
        widget.handle_event(&self.event, ctx).then_some(ctx).map_or(Ok(()), Err)
    }

    fn new_context<W: Widget>(&self, child: &W, &(parent_pos, _): &Self::Context, pdata: &ParentData) -> Option<Self::Context> {
        let pos = parent_pos - pdata.vp_orig + child.get_position();
        Some((pos, pdata.id))
    }
}

/// Sends an event to the widget under cursor.
struct PositionDispatchVisitor {
    event: Event,
    ctx: EventContext,
}

impl Visitor for PositionDispatchVisitor {
    type Return = EventContext;
    type Context = (Rect, WidgetId);

    fn visit<W: Widget>(&mut self, widget: &mut W, &(abs_bounds, parent): &Self::Context) -> Result<(), Self::Return> {
        if self.ctx.abs_pos.inside(abs_bounds) {
            let ctx = self.ctx.update(abs_bounds.pos.cast(), widget.get_id(), parent);
            widget.handle_event(&self.event, ctx).then_some(ctx).map_or(Ok(()), Err)
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, &(parent_vp, _): &Self::Context, pdata: &ParentData) -> Option<Self::Context> {
        child
            .get_bounds()
            .offset(parent_vp.pos - pdata.vp_orig)
            .clip_inside(parent_vp)
            .map(|vp| (vp, pdata.id))
    }
}

/// Checks what widget is under the cursor.
struct InsideCheckVisitor {
    ctx: EventContext,
    last_inside: WidgetId,
    in_res: Option<EventContext>,
}

impl InsideCheckVisitor {
    fn check_inside<W: Widget>(
        root: &mut W, last_inside: Option<WidgetId>, parent_size: Size, ctx: EventContext,
    ) -> (Option<WidgetId>, Option<EventContext>) {
        let mut visitor = InsideCheckVisitor {
            ctx,
            last_inside: last_inside.unwrap_or_default(),
            in_res: None,
        };
        let inside = visitor
            .visit_child_rev(root, &(parent_size.into(), WidgetId::EMPTY), &Default::default())
            .err();

        (inside, visitor.in_res)
    }
}

impl Visitor for InsideCheckVisitor {
    type Return = WidgetId;
    type Context = (Rect, WidgetId);

    fn visit<W: Widget>(&mut self, widget: &mut W, &(abs_bounds, parent): &Self::Context) -> Result<(), Self::Return> {
        let inside = self.ctx.abs_pos.inside(abs_bounds);
        if inside && self.last_inside != widget.get_id() {
            let ctx = self.ctx.update(abs_bounds.pos.cast(), widget.get_id(), parent);
            self.in_res = widget.handle_event(&Event::PointerInside(true), ctx).then_some(ctx);
        }
        if inside {
            Err(widget.get_id())
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, &(parent_vp, _): &Self::Context, pdata: &ParentData) -> Option<Self::Context> {
        child
            .get_bounds()
            .offset(parent_vp.pos - pdata.vp_orig)
            .clip_inside(parent_vp)
            .map(|vp| (vp, pdata.id))
    }
}

/// Sends an event to a single target.
struct TargetedDispatchVisitor {
    target: WidgetId,
    event: Event,
    ctx: EventContext,
}

impl TargetedDispatchVisitor {
    fn dispatch_to<W: Widget>(target: WidgetId, root: &mut W, event: Event, ctx: EventContext) -> Option<EventContext> {
        let mut visitor = TargetedDispatchVisitor { target, event, ctx };
        visitor.visit_child(root, &Default::default(), &Default::default()).err()
    }
}

impl Visitor for TargetedDispatchVisitor {
    type Return = EventContext;
    type Context = (Position, WidgetId);

    fn visit<W: Widget>(&mut self, widget: &mut W, &(abs_pos, parent): &Self::Context) -> Result<(), Self::Return> {
        if self.target == widget.get_id() {
            let ctx = self.ctx.update(abs_pos.cast(), widget.get_id(), parent);
            widget.handle_event(&self.event, ctx).then_some(ctx).map_or(Ok(()), Err)
        } else {
            Ok(())
        }
    }

    fn new_context<W: Widget>(&self, child: &W, &(parent_pos, _): &Self::Context, pdata: &ParentData) -> Option<Self::Context> {
        let pos = parent_pos - pdata.vp_orig + child.get_position();
        Some((pos, pdata.id))
    }
}

/// Sends an event consumed notification to every widget in the tree.
struct ConsumedNotifyVisitor {
    event: Event,
    ctx: EventContext,
}

impl ConsumedNotifyVisitor {
    fn notify_consumed<W: Widget>(root: &mut W, event: Event, ctx: EventContext) {
        let mut visitor = ConsumedNotifyVisitor { event, ctx };
        let _ = visitor.visit_child(root, &Default::default(), &Default::default());
    }
}

impl Visitor for ConsumedNotifyVisitor {
    type Return = ();
    type Context = ();

    fn visit<W: Widget>(&mut self, widget: &mut W, _ctx: &Self::Context) -> Result<(), Self::Return> {
        widget.event_consumed(&self.event, &self.ctx);
        Ok(())
    }

    fn new_context<W: Widget>(&self, _child: &W, _ctx: &Self::Context, _pdata: &ParentData) -> Option<Self::Context> {
        Some(())
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

        // check if pointer inside/outside changed, also dispatch inside event
        let mut in_res = None;
        let mut outside_target = None;
        match event {
            Event::MouseMoved(Axis::Position(_)) => {
                let (inside, res) = InsideCheckVisitor::check_inside(root, self.last_inside, parent_size, ctx);
                if inside != self.last_inside {
                    outside_target = std::mem::replace(&mut self.last_inside, inside);
                    in_res = res;
                }
            }
            Event::PointerInside(false) => {
                outside_target = self.last_inside.take();
            }
            _ => (),
        };
        // dispatch "outside changed" event
        let out_res =
            outside_target.and_then(|target| TargetedDispatchVisitor::dispatch_to(target, root, Event::PointerInside(false), ctx));

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
                let mut visitor = EventDispatchVisitor { event: event.clone(), ctx };
                visitor.visit_child_rev(root, &Default::default(), &Default::default()).err()
            }
            // position dependant events
            Event::MouseMoved(_) | Event::MouseButton(_, _) | Event::FileDropped(_) => {
                let mut visitor = PositionDispatchVisitor { event: event.clone(), ctx };
                visitor
                    .visit_child_rev(root, &(parent_size.into(), WidgetId::EMPTY), &Default::default())
                    .err()
            }
            // already handled
            Event::PointerInside(_) => None,
        };

        // send the event consumed notification
        if let Some(ctx) = in_res {
            ConsumedNotifyVisitor::notify_consumed(root, Event::PointerInside(true), ctx)
        }
        if let Some(ctx) = out_res {
            ConsumedNotifyVisitor::notify_consumed(root, Event::PointerInside(false), ctx)
        }
        if let Some(ctx) = res {
            ConsumedNotifyVisitor::notify_consumed(root, event, ctx)
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
