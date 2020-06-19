use crate::geometry::Bounds;

pub trait Layout: Bounds {
    fn left_of<B: Bounds>(&mut self, other: &B, margin: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_x(other.get_position().x - self.get_size().w as i32 - margin);
        self.set_position(pos);
        self
    }

    fn right_of<B: Bounds>(&mut self, other: &B, margin: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_x(other.get_position().x + other.get_size().w as i32 + margin);
        self.set_position(pos);
        self
    }

    fn above<B: Bounds>(&mut self, other: &B, margin: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_y(other.get_position().y - self.get_size().h as i32 - margin);
        self.set_position(pos);
        self
    }

    fn below<B: Bounds>(&mut self, other: &B, margin: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_y(other.get_position().y + other.get_size().h as i32 + margin);
        self.set_position(pos);
        self
    }

    fn align_left<B: Bounds>(&mut self, other: &B, offset: i32) -> &mut Self {
        let pos = self.get_position().with_x(other.get_position().x + offset);
        self.set_position(pos);
        self
    }

    fn align_right<B: Bounds>(&mut self, other: &B, offset: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_x(other.get_position().x + other.get_size().w as i32 - self.get_size().w as i32 + offset);
        self.set_position(pos);
        self
    }

    fn align_top<B: Bounds>(&mut self, other: &B, offset: i32) -> &mut Self {
        let pos = self.get_position().with_y(other.get_position().y + offset);
        self.set_position(pos);
        self
    }

    fn align_bottom<B: Bounds>(&mut self, other: &B, offset: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_y(other.get_position().y + other.get_size().h as i32 - self.get_size().h as i32 + offset);
        self.set_position(pos);
        self
    }

    fn align_hcenter<B: Bounds>(&mut self, other: &B, offset: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_x(other.get_position().x + (other.get_size().w as i32 - self.get_size().w as i32) / 2 + offset);
        self.set_position(pos);
        self
    }

    fn align_vcenter<B: Bounds>(&mut self, other: &B, offset: i32) -> &mut Self {
        let pos = self
            .get_position()
            .with_y(other.get_position().y + (other.get_size().h as i32 - self.get_size().h as i32) / 2 + offset);
        self.set_position(pos);
        self
    }

    fn center_inside<B: Bounds>(&mut self, other: &B) -> &mut Self {
        let b = other.get_bounds();
        self.set_position(b.pos + (b.size.as_position() - self.get_size().as_position()) / 2);
        self
    }
}

impl<T: Bounds> Layout for T {}
