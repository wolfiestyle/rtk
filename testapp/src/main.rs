use widgets::draw::{Color, DrawContext, Image, ImageRef};
use widgets::event::{EvState, Event, EventContext, EventResult, MouseButton};
use widgets::geometry::Rect;
use widgets::toplevel::TopLevel;
use widgets::widget::{Widget, WidgetId, Window};
use widgets::{implement_bounds, implement_objectid, implement_visitable};
use widgets_glium::GliumApplication;

#[derive(Debug)]
struct TestWidget<T, U> {
    bounds: Rect,
    color: Color,
    id: WidgetId,
    hover: bool,
    child1: T,
    child2: U,
}

impl<T: Widget, U: Widget> Widget for TestWidget<T, U> {
    fn update_layout(&mut self, _parent_rect: Rect) {
        use widgets::layout::Layout;

        self.child1.update_layout(self.bounds);
        self.child2.update_layout(self.bounds);

        self.child1.center_inside(&self.bounds.at_origin());
        self.child2.right_of(&self.child1, 0).align_top(&self.child1, 0);
    }

    fn draw(&self, mut dc: DrawContext) {
        let color = if self.hover {
            self.color.mix(Color::WHITE, 0.01)
        } else {
            self.color
        };
        dc.draw_rect([0, 0], self.bounds.size, color, None);

        dc.draw_child(&self.child1);
        dc.draw_child(&self.child2);
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        //println!("TestWidget({:?}): {:?} {:?}", self.label, event, ctx.local_pos);
        match event {
            Event::MouseButton(EvState::Pressed, MouseButton::Left) => {
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

implement_objectid!(TestWidget<A, B>, id);
implement_bounds!(TestWidget<A, B>, rect: bounds);
implement_visitable!(TestWidget<A: Widget, B: Widget>, child1, child2);

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
            Event::MouseButton(EvState::Pressed, MouseButton::Left) => {
                println!("TestWidget2({:?}) clicked! (pos={:?})", self.id, ctx.local_pos);
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }
}

implement_objectid!(TestWidget2, id);
implement_bounds!(TestWidget2, rect: bounds);
implement_visitable!(TestWidget2);

fn main() {
    let widget = TestWidget {
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::blue(0.25),
        hover: false,
        id: WidgetId::new(),
        child1: TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([0, 0], [100, 100]),
            image: Image::from_file("image.png").unwrap().into(),
        },
        child2: TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([0, 0], [64, 64]),
            image: Image::from_file("image2.jpg").unwrap().into(),
        },
    };

    let mut window = Window::new(widget);
    window.set_title("awoo");
    window.set_background([0.1, 0.1, 0.1]);
    window.update();

    let mut app = GliumApplication::new();
    app.add_window(window);
    app.run();
}
