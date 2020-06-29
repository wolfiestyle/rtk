#![allow(dead_code)]
use widgets::draw::DrawContext;
use widgets::event::{Event, EventContext, EventResult};
use widgets::geometry::Rect;
use widgets::visitor::Visitor;
use widgets::widget::{Empty, Widget, WidgetId};
use widgets_derive::{Bounds, ObjectId, Visitable};

#[derive(ObjectId, Bounds, Visitable)]
#[impl_generics(T)]
struct TestWidget1<T> {
    id: WidgetId,
    bounds: Rect,
    #[visit_child]
    child: T,
}

impl<T: Widget> Widget for TestWidget1<T> {
    fn update_layout(&mut self, _parent_rect: Rect) {}
    fn draw(&self, _dc: DrawContext) {}
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
    fn event_consumed(&mut self, _wid: WidgetId, _event: &Event, _ctx: EventContext) {}
}

#[derive(ObjectId, Bounds, Visitable)]
#[impl_generics(T)]
struct TestWidget2<T> {
    id: WidgetId,
    bounds: Rect,
    #[visit_iter]
    child: Vec<T>,
}

impl<T: Widget> Widget for TestWidget2<T> {
    fn update_layout(&mut self, _parent_rect: Rect) {}
    fn draw(&self, _dc: DrawContext) {}
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
    fn event_consumed(&mut self, _wid: WidgetId, _event: &Event, _ctx: EventContext) {}
}

#[derive(Default)]
struct TestVisitor {
    ids: Vec<WidgetId>,
}

impl Visitor for TestVisitor {
    type Return = ();
    type Context = ();

    fn visit<W: Widget>(&mut self, widget: &mut W, _ctx: &Self::Context) -> Result<(), Self::Return> {
        self.ids.push(widget.get_id());
        Ok(())
    }

    fn new_context<W: Widget>(&self, _child: &W, _parent_ctx: &Self::Context) -> Option<Self::Context> {
        Some(())
    }
}

#[test]
fn visitable() {
    let mut ids = vec![];
    ids.resize_with(3, WidgetId::new);

    let mut widget = TestWidget2 {
        id: ids[0],
        bounds: Rect::default(),
        child: vec![
            TestWidget1 {
                id: ids[1],
                bounds: Rect::default(),
                child: Empty::new(),
            },
            TestWidget1 {
                id: ids[2],
                bounds: Rect::default(),
                child: Empty::new(),
            },
        ],
    };
    let mut visitor = TestVisitor::default();
    visitor.visit_child(&mut widget, &()).unwrap();
    assert_eq!(visitor.ids, ids);

    ids.reverse();
    visitor.ids.clear();
    visitor.visit_child_rev(&mut widget, &()).unwrap();
    assert_eq!(visitor.ids, ids);
}
