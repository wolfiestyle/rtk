use widgets::draw::{TexCoord, TexRect};
use widgets::event::*;
use widgets::geometry::VAlign;
use widgets::image::Image;
use widgets::layout;
use widgets::prelude::*;
use widgets::toplevel::Window;
use widgets_derive::{Bounds, ObjectId, Visitable};
use widgets_glium::GliumApplication;

#[derive(Debug, ObjectId, Bounds, Visitable)]
#[impl_generics(T)]
struct TestWidget {
    bounds: Rect,
    color: Color,
    id: WidgetId,
    hover: bool,
    vp_orig: Position,
    image: Image,
    #[visit_iter]
    childs: Vec<TestWidget2>,
}

impl Widget for TestWidget {
    fn update_layout(&mut self, parent_rect: Rect) {
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

    fn draw<B: DrawBackend>(&self, mut dc: DrawContext<B>) {
        let color = if self.hover {
            self.color.mix(Color::WHITE, 0.01)
        } else {
            self.color
        };

        //dc.clear(color);
        let crop = TexRect::default() + TexCoord::new(0.1, 0.2);
        dc.draw_rect((dc.origin(), self.bounds.size), crop, (color, &self.image));

        for child in &self.childs {
            dc.draw_child(child);
        }
    }

    fn handle_event(&mut self, event: &Event, ctx: EventContext) -> EventResult {
        //println!("TestWidget({:?}): {:?} {:?}", self.id, event, ctx.local_pos);
        match event {
            Event::MouseButton(Pressed, MouseButton::Left) => {
                println!("TestWidget({:?}) clicked! (pos={:?})", self.id, ctx.local_pos);
                self.color = Color::WHITE;
                EventResult::Consumed
            }
            Event::Keyboard { state: Pressed, key, .. } => {
                match key {
                    Key::Left => self.vp_orig.x -= 1,
                    Key::Right => self.vp_orig.x += 1,
                    Key::Up => self.vp_orig.y -= 1,
                    Key::Down => self.vp_orig.y += 1,
                    _ => return EventResult::Pass,
                }
                EventResult::Consumed
            }
            Event::PointerInside(inside) => {
                self.hover = *inside;
                EventResult::Consumed
            }
            _ => EventResult::Pass,
        }
    }

    fn event_consumed(&mut self, event: &Event, ctx: &EventContext) {
        if ctx.parent != self.id {
            return;
        }

        if let Event::MouseButton(Pressed, _) = event {
            if let Some(child) = self.childs.iter().find(|w| w.get_id() == ctx.widget) {
                self.color = child.color;
            }
        }
    }

    fn viewport_origin(&self) -> Position {
        self.vp_orig
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

    fn draw<B: DrawBackend>(&self, mut dc: DrawContext<B>) {
        //dc.clear(self.color);
        dc.draw_rect((dc.origin(), self.bounds.size), None, self.color);
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

    fn event_consumed(&mut self, _event: &Event, _ctx: &EventContext) {}
}
/*
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
*/
fn main() {
    let mut widget = TestWidget {
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::WHITE,
        hover: false,
        vp_orig: Default::default(),
        id: WidgetId::new(),
        image: Image::from_file("image.jpg").unwrap(),
        childs: Vec::new(),
    };

    for i in 0..20 {
        let v = i as f32 / 19.0;
        let s = i % 7;
        widget.childs.push(TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([0, 0], [30 + s, 30 + s * 2]),
            color: Color::hsl(v * 360.0, 1.0, 0.5),
        });
        //widget.childs.push(Empty::with_size([10, 10]).into());
    }

    let mut window = Window::new(widget);
    window.set_title("awoo");
    window.set_background([0.1, 0.1, 0.1]);

    let mut app = GliumApplication::new();
    app.add_window(window);
    app.run();
}
