use std::sync::mpsc::{channel, Sender};
use widgets::draw::{DrawContext, Text, TextSection};
use widgets::prelude::*;
use widgets::testing::{TestBackend, TestDrawCmd};
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
    events: Sender<Event>,
}

impl Widget for TestWidget {
    fn update_layout<R: Resources>(&mut self, parent_rect: Rect, _resources: &mut R) {
        self.bounds = parent_rect;
    }
    fn draw<B: DrawBackend>(&self, mut dc: DrawContext<B>) {
        dc.draw_text(TextSection::default().add_text(Text::new("asdf")));
    }

    fn handle_event(&mut self, event: &Event, _ctx: EventContext) -> EventResult {
        self.events.send(event.clone()).unwrap();
        EventResult::Consumed
    }

    fn event_consumed(&mut self, event: &Event, _ctx: &EventContext) {
        self.events.send(event.clone()).unwrap();
    }

    fn viewport_origin(&self) -> Position {
        [11, 22].into()
    }
}

#[test]
fn widget() {
    let rect1 = Rect::new([0, 1], [13, 42]);
    let rect2 = Rect::new([10, 20], [320, 240]);
    let (events, ev_recv) = channel();

    let mut e1: TestEnum<TestWidget> = TestEnum::Empty(rect1.into());
    let mut e2 = TestEnum::Other(TestWidget {
        id: WidgetId::new(),
        bounds: rect1,
        events,
    });

    let mut backend = TestBackend::default();

    e1.update_layout(rect2, &mut backend);
    e2.update_layout(rect2, &mut backend);
    assert_eq!(e1.get_bounds(), rect1);
    assert_eq!(e2.get_bounds(), rect2);

    e2.draw(DrawContext::new(&mut backend, Rect::new_at_origin([800, 600])));
    if let Some(TestDrawCmd::Text {
        text: TextSection { text, .. },
        ..
    }) = &backend.draw_cmd.get(0)
    {
        assert_eq!(text[0].text, "asdf");
    } else {
        panic!("backend.draw_cmd empty")
    }

    let ctx = unsafe { std::mem::zeroed() }; // value is never read, so it should be safe
    let res = e2.handle_event(&Event::Created, ctx);
    assert_eq!(res, EventResult::Consumed);
    assert_eq!(ev_recv.try_recv(), Ok(Event::Created));
    e2.event_consumed(&Event::CloseRequest, &ctx);
    assert_eq!(ev_recv.try_recv(), Ok(Event::CloseRequest));
    std::mem::forget(ctx); // make sure it's not calling Drop

    let vp_orig = e2.viewport_origin();
    assert_eq!(vp_orig, Position::new(11, 22));
}
