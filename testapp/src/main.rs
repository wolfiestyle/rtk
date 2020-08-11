use widgets::draw::{Text, TextLayout, TextSection, TextureId};
use widgets::event::*;
use widgets::geometry::{HAlign, VAlign};
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
    texture: TextureId,
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
        let hover = if self.hover { Color::gray(0.05) } else { Default::default() };

        dc.draw_rect((dc.origin(), self.bounds.size), self.texture * self.color + hover);
        dc.draw_triangle([10, 110], [100, 150], [50, 200], Color::BLUE.with_alpha(0.5));
        dc.draw_text(
            TextSection::default()
                .add_text(Text::new(&format!("{:?}", self.color)))
                .with_bounds(self.bounds.size.as_point())
                .with_screen_position((self.bounds.size / 2).as_point())
                .with_layout(TextLayout::default_wrap().h_align(HAlign::Center).v_align(VAlign::Center)),
        );

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
    text: String,
}

impl Widget for TestWidget2 {
    fn update_layout(&mut self, _parent_rect: Rect) {}

    fn draw<B: DrawBackend>(&self, mut dc: DrawContext<B>) {
        //dc.clear(self.color);
        dc.draw_rect((dc.origin(), self.bounds.size), self.color);
        dc.draw_text(TextSection::default().add_text(Text::new(&self.text)));
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
    let mut app = GliumApplication::new();

    let image = Image::from_file("image.jpg").unwrap();
    let texture = app.create_texture(&image);

    let mut widget = TestWidget {
        bounds: Rect::new([20, 10], [320, 240]),
        color: Color::WHITE,
        hover: false,
        vp_orig: Default::default(),
        id: WidgetId::new(),
        texture,
        childs: Vec::new(),
    };

    for i in 0..20 {
        let v = i as f32 / 19.0;
        let s = i % 7;
        widget.childs.push(TestWidget2 {
            id: WidgetId::new(),
            bounds: Rect::new([0, 0], [30 + s, 30 + s * 2]),
            color: Color::hsl(v * 360.0, 1.0, 0.5),
            text: i.to_string(),
        });
        //widget.childs.push(Empty::with_size([10, 10]).into());
    }

    let mut window = Window::new(widget);
    window.set_title("awoo");
    window.set_background([0.1, 0.1, 0.1]);

    app.add_window(window);
    app.run();
}
