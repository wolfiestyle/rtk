use widgets::image::{Image, ImageRef};
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
    fn update_layout(&mut self, _parent_rect: Rect) {
        use widgets::layout::{foreach, Layout};

        for child in &mut self.childs {
            child.update_layout(self.bounds);
        }

        if let Some(first) = self.childs.first_mut() {
            first.set_position([10, 10].into());
        }

        layout::foreach(&mut self.childs, |this, prev, first| {
            this.right_of(prev, 0).align_vcenter(first, 0)
        });
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
}

#[derive(Debug, ObjectId, Bounds, Visitable)]
struct TestWidget2 {
    id: WidgetId,
    bounds: Rect,
    image: ImageRef,
}

impl Widget for TestWidget2 {
    fn update_layout(&mut self, _parent_rect: Rect) {}

    fn draw(&self, mut dc: DrawContext) {
        dc.draw_rect([0, 0], self.bounds.size, Color::WHITE, Some(self.image.clone()));
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        //println!("Window2: {:?}", event);
        match event {
            Event::MouseButton(Pressed, MouseButton::Left) => {
                println!("TestWidget2({:?}) clicked! (pos={:?})", self.id, ctx.local_pos);
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }
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
        color: Color::blue(0.25),
        hover: false,
        id: WidgetId::new(),
        childs: Vec::new(),
    };

    let image: ImageRef = Image::from_file("image2.jpg").unwrap().into();

    widget.childs.push(
        TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([10, 20], [100, 100]),
            image: Image::from_file("image.png").unwrap().into(),
        }
        .into(),
    );

    widget.childs.push(
        TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([0, 0], [64, 64]),
            image: image.clone(),
        }
        .into(),
    );
    widget.childs.push(Empty::with_size([20, 20]).into());
    widget.childs.push(
        TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([0, 0], [45, 30]),
            image,
        }
        .into(),
    );

    let mut window = Window::new(widget);
    window.set_title("awoo");
    window.set_background([0.1, 0.1, 0.1]);

    let mut app = GliumApplication::new();
    app.add_window(window);
    app.run();
}
