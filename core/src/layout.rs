//! Helper methods for composing widget layouts.
use crate::geometry::{Bounds, VAlign};

/// Bounds extension for placing widgets relative to others.
pub trait Layout: Bounds {
    fn left_of<B: Bounds>(&mut self, other: &B, margin: u32) -> &mut Self {
        let pos = self
            .get_position()
            .with_x(other.get_position().x - (self.get_size().w + margin) as i32);
        self.set_position(pos);
        self
    }

    fn right_of<B: Bounds>(&mut self, other: &B, margin: u32) -> &mut Self {
        let pos = self
            .get_position()
            .with_x(other.get_position().x + (other.get_size().w + margin) as i32);
        self.set_position(pos);
        self
    }

    fn above<B: Bounds>(&mut self, other: &B, margin: u32) -> &mut Self {
        let pos = self
            .get_position()
            .with_y(other.get_position().y - (self.get_size().h + margin) as i32);
        self.set_position(pos);
        self
    }

    fn below<B: Bounds>(&mut self, other: &B, margin: u32) -> &mut Self {
        let pos = self
            .get_position()
            .with_y(other.get_position().y + (other.get_size().h + margin) as i32);
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

    fn align_hf<B: Bounds>(&mut self, other: &B, val: f32, offset: i32) -> &mut Self {
        let dx = (other.get_size().w as i32 - self.get_size().w as i32) as f32 * val;
        let pos = self.get_position().with_x(other.get_position().x + dx as i32 + offset);
        self.set_position(pos);
        self
    }

    fn align_vf<B: Bounds>(&mut self, other: &B, val: f32, offset: i32) -> &mut Self {
        let dy = (other.get_size().h as i32 - self.get_size().h as i32) as f32 * val;
        let pos = self.get_position().with_y(other.get_position().y + dy as i32 + offset);
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

/// Runs a layout expression for a collection of widgets.
pub fn foreach<'a, T: Bounds + 'a, F>(items: impl IntoIterator<Item = &'a mut T>, mut f: F)
where
    F: FnMut(&'a mut T, &T, &T) -> &'a T,
{
    let mut iter = items.into_iter();
    if let Some(first) = iter.next() {
        let first = &*first;
        iter.fold(first, |prev, item| f(item, &prev, first));
    }
}

/// Places a collection of widgets horizontally in sequence, starting a new row if necessary.
pub fn flow_horiz<'a, T, I>(items: I, valign: VAlign, max_width: u32, hspacing: u32, vspacing: u32)
where
    T: Bounds + 'a,
    I: IntoIterator<Item = &'a mut T>,
{
    let align_f = match valign {
        VAlign::Top => Layout::align_top,
        VAlign::Center => Layout::align_vcenter,
        VAlign::Bottom => Layout::align_bottom,
    };

    let mut row_items = vec![];
    let mut iter = items.into_iter();

    if let Some(first_) = iter.next() {
        let mut first = first_.get_bounds();
        row_items.push(first_);

        let mut prev = first;
        let mut row = first;
        for item in iter {
            // if we exceeded the max_width, then place this widget on a new row
            if row.size.w + item.get_size().w + hspacing > max_width {
                // check if we're overlapping the previous row
                let offset = first.pos.y - row.pos.y;
                if offset > 0 {
                    row.pos.y += offset;
                    // displace the previous widgets to the fixed row position
                    for w in &mut row_items {
                        w.set_position(w.get_position().offset(0, offset));
                    }
                }
                // place this widget below the current row
                item.below(&row, vspacing).align_left(&first, 0);
                // start the next row
                first = item.get_bounds();
                row_items.clear();
                row_items.push(item);
                prev = first;
                row = first;
            } else {
                // place this widget next to the previous one
                align_f(item.right_of(&prev, hspacing), &prev, 0);
                // expand the current row with this widget's bounds
                let item_bounds = item.get_bounds();
                row_items.push(item);
                prev = item_bounds;
                row = row.merge(item_bounds);
            }
        }
        // fix the last row's position if needed
        let offset = first.pos.y - row.pos.y;
        if offset > 0 {
            for w in &mut row_items {
                w.set_position(w.get_position().offset(0, offset));
            }
        }
    }
}
