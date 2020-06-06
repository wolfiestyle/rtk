use widgets::draw::{Color, DrawContext};
use widgets::event::{EvData, EvState, Event, EventResult, MouseButton};
use widgets::geometry::{Pointi, Rect, Size};
use widgets::visitor::Visitor;
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

    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
        visitor.visit(self, &ctx)?;
        self.child.accept(visitor, visitor.new_context(&self.child, &ctx))?;
        self.child2.accept(visitor, visitor.new_context(&self.child2, &ctx))
    }

    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: V::Context) -> Result<(), V::Error> {
        self.child.accept_rev(visitor, visitor.new_context(&self.child, &ctx))?;
        self.child2
            .accept_rev(visitor, visitor.new_context(&self.child2, &ctx))?;
        visitor.visit(self, &ctx)
    }
}

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

                if let Some(_) = gl_win.push_event(event) {
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
