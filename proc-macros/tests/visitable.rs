use widgets::prelude::*;
use widgets::visitor::{Visitable, Visitor};
use widgets::widget::Empty;
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
    fn update_layout<R: Resources>(&mut self, _parent_rect: Rect, _resources: &mut R) {}
    fn draw<B: DrawBackend>(&self, _dc: DrawContext<B>) {}
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
    fn event_consumed(&mut self, _event: &Event, _ctx: &EventContext) {}
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
    fn update_layout<R: Resources>(&mut self, _parent_rect: Rect, _resources: &mut R) {}
    fn draw<B: DrawBackend>(&self, _dc: DrawContext<B>) {}
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
    fn event_consumed(&mut self, _event: &Event, _ctx: &EventContext) {}
}

#[derive(Default)]
struct TestVisitor {
    ids: Vec<WidgetId>,
}

impl Visitor for TestVisitor {
    type Context = ();

    fn visit_before<W: Widget>(mut self, widget: &mut W, _: &Self::Context) -> Self {
        self.ids.push(widget.get_id());
        self
    }

    fn new_context<W: Widget>(&self, _: &W, _: &Self::Context) -> Option<Self::Context> {
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

    let expected = [ids[0], ids[1], WidgetId::EMPTY, ids[2], WidgetId::EMPTY];
    let visitor = widget.accept(TestVisitor::default(), &());
    assert_eq!(visitor.ids, expected);
}
