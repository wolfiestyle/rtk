use widgets::prelude::*;
use widgets::widget::Empty;
use widgets_derive::{Bounds, ObjectId, Visitable, Widget};
use widgets_glium::GliumApplication;

#[derive(Debug, ObjectId, Bounds, Visitable)]
#[impl_generics(T)]
struct TestWidget<T> {
    bounds: Rect,
    color: Color,
    id: WidgetId,
    hover: bool,
    #[visit_iter]
    childs: Vec<T>,
}

impl<T: Widget> Widget for TestWidget<T> {
    fn update_layout(&mut self, parent_rect: Rect) {
        use widgets::layout;

        for child in &mut self.childs {
            child.update_layout(self.bounds);
        }

        self.bounds.size = parent_rect.size - self.bounds.pos.as_size();

        if let Some(first) = self.childs.first_mut() {
            first.set_position([0, 0].into());
        }

        /*
        layout::foreach(&mut self.childs, |this, prev, first| {
            this.right_of(prev, 0).align_vcenter(first, 0)
        });
        */
        layout::flow_horiz(&mut self.childs, VAlign::Bottom, self.bounds.size.w, 0, 0);
    }

    fn draw(&self, mut dc: DrawContext) {
        let color = if self.hover {
            self.color.mix(Color::WHITE, 0.01)
        } else {
            self.color
        };
        dc.draw_rect([0, 0], self.bounds.size, color, None);

        for child in &self.childs {
            dc.draw_child(child);
        }
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        //println!("TestWidget({:?}): {:?} {:?}", self.id, event, ctx.local_pos);
        match event {
            Event::MouseButton(Pressed, MouseButton::Left) => {
                println!("TestWidget({:?}) clicked! (pos={:?})", self.id, ctx.local_pos);
                EventResult::Consumed
            }
            Event::PointerInside(inside) => {
                self.hover = *inside;
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }

    fn event_consumed(&mut self, event: &Event, ctx: EventContext) {
        println!("event consumed by {:?} (parent {:?}): {:?}", ctx.widget, ctx.parent, event);
    }
}

#[derive(Debug, ObjectId, Bounds, Visitable)]
struct TestWidget2 {
    id: WidgetId,
    bounds: Rect,
    color: Color,
}

impl Widget for TestWidget2 {
    fn update_layout(&mut self, _parent_rect: Rect) {}

    fn draw(&self, mut dc: DrawContext) {
        dc.draw_rect([0, 0], self.bounds.size, self.color, None);
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        //println!("Window2: {:?}", event);
        match event {
            Event::MouseButton(Pressed, MouseButton::Left) => {
                println!("TestWidget2({:?}) clicked! ({:?}, {:?})", self.id, self.color, ctx.local_pos);
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }

    fn event_consumed(&mut self, _event: &Event, _ctx: EventContext) {}
}

#[derive(Debug, ObjectId, Bounds, Visitable, Widget)]
enum TestEnum {
    TestWidget2(TestWidget2),
    Empty(Empty),
}

impl From<TestWidget2> for TestEnum {
    fn from(val: TestWidget2) -> Self {
        TestEnum::TestWidget2(val)
    }
}

impl From<Empty> for TestEnum {
    fn from(val: Empty) -> Self {
        TestEnum::Empty(val)
    }
}

fn main() {
    let mut widget: TestWidget<TestEnum> = TestWidget {
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::BLACK,
        hover: false,
        id: WidgetId::new(),
        childs: Vec::new(),
    };

    for i in 0..20 {
        let v = i as f32 / 19.0;
        let s = i % 7;
        widget.childs.push(
            TestWidget2 {
                id: WidgetId::new(),
                bounds: Rect::new([0, 0], [30 + s, 30 + s * 2]),
                color: Color::hsl(v * 360.0, 1.0, 0.5),
            }
            .into(),
        );
        //widget.childs.push(Empty::with_size([10, 10]).into());
    }

    let mut window = Window::new(widget);
    window.set_title("awoo");
    window.set_background([0.1, 0.1, 0.1]);

    let mut app = GliumApplication::new();
    app.add_window(window);
    app.run();
}
