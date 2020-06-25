use widgets::draw::DrawContext;
use widgets::event::{Event, EventContext, EventResult};
use widgets::geometry::{Bounds, Rect};
use widgets::widget::{Empty, Widget, WidgetId};
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
    fn update_layout(&mut self, parent_rect: Rect) {
        self.bounds = parent_rect;
    }
    fn draw(&self, _dc: DrawContext) {}
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }
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
    e1.update_layout(rect2);
    e2.update_layout(rect2);
    assert_eq!(e1.get_bounds(), rect1);
    assert_eq!(e2.get_bounds(), rect2);
}
