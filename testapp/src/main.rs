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
    label: &'static str,
    id: WidgetId,
    hover: bool,
    child: T,
    child2: U,
}

impl<T: Widget, U: Widget> Widget for TestWidget<T, U> {
    fn update_layout(&mut self, _parent_rect: Rect) {
        self.child.update_layout(self.bounds);
    }

    fn draw(&self, mut dc: DrawContext) {
        let color = if self.hover {
            self.color.mix(Color::WHITE, 0.25)
        } else {
            self.color
        };
        dc.draw_rect([0, 0], self.bounds.size, color, None);
        dc.draw_child(&self.child);
        dc.draw_child(&self.child2);
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        //println!("TestWidget({:?}): {:?}", self.label, event);
        match event {
            Event::MouseButton(EvState::Pressed, MouseButton::Left) => {
                println!(
                    "TestWidget({}, {:?}) clicked! (pos={:?})",
                    self.label, self.id, ctx.pointer_pos
                );
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
implement_visitable!(TestWidget<A: Widget, B: Widget>, child, child2);

struct TestWidget2 {
    id: WidgetId,
    bounds: Rect,
    image: ImageRef,
}

impl Widget for TestWidget2 {
    fn update_layout(&mut self, parent_rect: Rect) {
        self.bounds.size = parent_rect.size;
    }

    fn draw(&self, mut dc: DrawContext) {
        dc.draw_rect([0, 0], self.bounds.size, Color::WHITE, Some(self.image.clone()));
    }

    fn handle_event(&mut self, event: &Event, _ctx: EventContext) -> EventResult {
        //println!("Window2: {:?}", event);
        match event {
            Event::MouseButton(EvState::Pressed, MouseButton::Left) => {
                println!("clicked!");
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
        // 1
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::red(0.25),
        label: "red",
        hover: false,
        id: WidgetId::new(),
        child: TestWidget {
            // 2
            bounds: Rect::new([50, 20], [210, 120]),
            color: Color::BLUE,
            label: "blue",
            id: WidgetId::new(),
            hover: false,
            child: TestWidget {
                // 3
                bounds: Rect::new([70, 100], [70, 50]),
                color: Color::green(0.5),
                label: "green",
                id: WidgetId::new(),
                hover: false,
                child: (),
                child2: (),
            },
            child2: (),
        },
        child2: TestWidget {
            // 4
            bounds: Rect::new([70, 160], [120, 100]),
            color: Color::yellow(0.5),
            label: "yellow",
            id: WidgetId::new(),
            hover: false,
            child: (),
            child2: (),
        },
    };

    let mut window = Window::new(widget);
    window.set_title("awoo");
    window.set_background([0.1, 0.1, 0.1]);
    window.update();

    let widget2 = TestWidget2 {
        id: WidgetId::new(),
        bounds: Rect::new([0, 0], [128, 128]),
        image: Image::from_file("image.png").unwrap().into(),
    };
    let mut win2 = Window::new(widget2);
    win2.set_title("window2");
    win2.update();

    let mut app = GliumApplication::new_dyn();
    app.add_window(Box::new(window));
    app.add_window(Box::new(win2));
    app.run();
}
