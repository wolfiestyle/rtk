use widgets::backend::NullResources;
use widgets::prelude::*;
use widgets::widget::Empty;
use widgets_derive::{Bounds, ObjectId, Visitable, Widget};

#[derive(ObjectId, Bounds, Visitable, Widget)]
#[impl_generics(T)]
enum TestEnum<T> {
    Empty(Empty),
    Other(T),
}

#[derive(ObjectId, Bounds, Visitable)]
struct TestWidget {
    id: WidgetId,
    bounds: Rect,
}

impl Widget for TestWidget {
    fn update_layout<R: BackendResources>(&mut self, parent_rect: Rect, _resources: &mut R) {
        self.bounds = parent_rect;
    }
    fn draw<B: DrawBackend>(&self, _dc: DrawContext<B>) {}
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
    fn event_consumed(&mut self, _event: &Event, _ctx: &EventContext) {}
}

#[test]
fn widget() {
    let rect1 = Rect::new([0, 1], [13, 42]);
    let rect2 = Rect::new([10, 20], [320, 240]);
    let mut e1: TestEnum<Empty> = TestEnum::Empty(rect1.into());
    let mut e2 = TestEnum::Other(TestWidget {
        id: WidgetId::new(),
        bounds: rect1,
    });
    e1.update_layout(rect2, &mut NullResources);
    e2.update_layout(rect2, &mut NullResources);
    assert_eq!(e1.get_bounds(), rect1);
    assert_eq!(e2.get_bounds(), rect2);
}
