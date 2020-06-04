use widgets::draw::{Color, DrawContext};
use widgets::event::{self, EvData, Event, EventResult};
use widgets::geometry::{Pointi, Rect, Size};
use widgets::widget::{TopLevel, Widget, Window};

mod backend;
use backend::GliumWindow;

#[derive(Debug)]
struct TestWidget<T> {
    bounds: Rect,
    color: Color,
    label: &'static str,
    child: T,
}

impl<T: Widget> Widget for TestWidget<T> {
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
    }

    fn push_event(&mut self, event: &Event) -> EventResult {
        //println!("TestWidget: {:#?}", event);

        match event.data {
            EvData::PointerInside(inside) => {
                println!("TestWidget({}) inside={:?}", self.label, inside);
                event::EVENT_CONSUMED
            }
            _ => self.child.push_event(event),
        }
    }
}

fn main() {
    use glium::glutin::event::{Event, WindowEvent};
    use glium::glutin::event_loop::{ControlFlow, EventLoop};

    let event_loop = EventLoop::new();

    let widget = TestWidget {
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::red(0.25),
        label: "red",
        child: TestWidget {
            bounds: Rect::new([50, 20], [210, 120]),
            color: Color::BLUE,
            label: "blue",
            child: TestWidget {
                bounds: Rect::new([70, 100], [70, 50]),
                color: Color::green(0.5),
                label: "green",
                child: (),
            },
        },
    };
    let mut window = Window::new(widget);
    window.set_title("awoo");
    //window.set_background([0.1, 0.1, 0.1]);
    window.attr.background = None;
    window.update();
    println!("{:?}", window);

    let mut gl_win = GliumWindow::new(window, &event_loop);

    event_loop.run(move |event, _, cf| {
        *cf = ControlFlow::Wait;
        //println!("{:?}", event);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *cf = ControlFlow::Exit;
                    }
                    _ => (),
                }

                if let Err(_) = gl_win.push_event(event) {
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
