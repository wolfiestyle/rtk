use widgets::draw::{Color, DrawContext};
use widgets::event::{EvState, Event, EventContext, EventResult, MouseButton};
use widgets::geometry::{Position, Rect, Size};
use widgets::implement_visitable;
use widgets::widget::{TopLevel, Widget, WidgetId, Window};

mod backend;
use backend::*;

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
    fn get_id(&self) -> WidgetId {
        self.id
    }

    fn get_position(&self) -> Position {
        self.bounds.pos
    }

    fn get_size(&self) -> Size {
        self.bounds.size
    }

    fn set_position(&mut self, position: Position) {
        self.bounds.pos = position
    }

    fn update_size(&mut self, _parent_rect: Rect) {
        self.child.update_size(self.bounds);
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
            Event::MouseButton {
                state: EvState::Pressed,
                button: MouseButton::Left,
            } => {
                println!(
                    "TestWidget({}, {:?}) clicked! (pos={:?})",
                    self.label, self.id, ctx.pointer_pos
                );
                EventResult::Consumed
            }
            Event::PointerInside(inside) => {
                self.hover = *inside;
                println!("TestWidget({}, {:?}) inside={}", self.label, self.id, inside);
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }
}

implement_visitable!(TestWidget<A: Widget, B: Widget>, child, child2);

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

    let mut win2 = Window::new(());
    win2.set_size([100, 100].into());

    let mut app = GliumApplication::new_dyn();
    app.add_window(Box::new(window));
    app.add_window(Box::new(win2));
    app.run();
}
