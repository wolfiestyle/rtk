use widgets::draw::{Color, DrawContext};
use widgets::event::{EvData, EvState, Event, EventResult, MouseButton};
use widgets::geometry::{Pointi, Rect, Size};
use widgets::implement_visitable;
use widgets::widget::{TopLevel, Widget, WidgetId, Window};

mod backend;
use backend::GliumWindow;

#[derive(Debug)]
struct TestWidget<T, U> {
    bounds: Rect,
    color: Color,
    label: &'static str,
    id: WidgetId,
    child: T,
    child2: U,
}

impl<T: Widget, U: Widget> Widget for TestWidget<T, U> {
    fn get_id(&self) -> WidgetId {
        self.id
    }

    fn get_position(&self) -> Pointi {
        self.bounds.pos
    }

    fn get_size(&self) -> Size {
        self.bounds.size
    }

    fn set_position(&mut self, position: Pointi) {
        self.bounds.pos = position
    }

    fn update_size(&mut self, _parent_rect: Rect) {
        self.child.update_size(self.bounds);
    }

    fn draw(&self, mut dc: DrawContext) {
        dc.draw_rect([0, 0], self.bounds.size, self.color, None);
        dc.draw_child(&self.child);
        dc.draw_child(&self.child2);
    }

    fn handle_event(&mut self, event: &Event) -> EventResult {
        //println!("TestWidget({:?}): {:?}", self.label, event);

        match event.data {
            EvData::MouseButton {
                state: EvState::Pressed,
                button: MouseButton::Left,
            } => {
                println!(
                    "TestWidget({}, {:?}) clicked! (pos={:?})",
                    self.label, self.id, event.pointer_pos
                );
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }
}

implement_visitable!(TestWidget<A: Widget, B: Widget>, child, child2);

fn main() {
    use glium::glutin::event::{Event, WindowEvent};
    use glium::glutin::event_loop::{ControlFlow, EventLoop};

    let event_loop = EventLoop::new();

    let widget = TestWidget {
        // 1
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::red(0.25),
        label: "red",
        id: WidgetId::new(),
        child: TestWidget {
            // 2
            bounds: Rect::new([50, 20], [210, 120]),
            color: Color::BLUE,
            label: "blue",
            id: WidgetId::new(),
            child: TestWidget {
                // 3
                bounds: Rect::new([70, 100], [70, 50]),
                color: Color::green(0.5),
                label: "green",
                id: WidgetId::new(),
                child: (),
                child2: (),
            },
            child2: (),
        },
        child2: TestWidget {
            // 4
            bounds: Rect::new([70, 160], [120, 100]),
            color: Color::YELLOW,
            label: "yellow",
            id: WidgetId::new(),
            child: (),
            child2: (),
        },
    };

    let mut window = Window::new(widget);
    window.set_title("awoo");
    //window.set_background([0.1, 0.1, 0.1]);
    window.attr.background = None;
    window.update();

    let mut gl_win = GliumWindow::new(window, &event_loop);

    event_loop.run(move |event, _, cf| {
        *cf = ControlFlow::Wait;
        //println!("{:?}", event);

        match event {
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::CloseRequested = event {
                    *cf = ControlFlow::Exit;
                }

                if let Some(id) = gl_win.push_event(event) {
                    println!("recv by {:?}", id);
                    gl_win.redraw();
                }
            }
            Event::MainEventsCleared => {
                gl_win.update();
            }
            Event::RedrawRequested(_) => {
                gl_win.draw();
            }
            _ => (),
        }
    });
}
