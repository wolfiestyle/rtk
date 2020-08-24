use rtk::prelude::*;
use rtk_derive::{Bounds, Visitable};

/// The empty widget.
///
/// It's a "null" widget that does nothing (it only occupies space).
/// Can be used as a filler in layout definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Bounds, Visitable)]
pub struct Empty {
    bounds: Rect,
}

impl Empty {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn with_bounds(bounds: impl Into<Rect>) -> Self {
        Empty { bounds: bounds.into() }
    }

    #[inline]
    pub fn with_size(size: impl Into<Size>) -> Self {
        Empty {
            bounds: size.into().into(),
        }
    }
}

impl ObjectId for Empty {
    #[inline]
    fn get_id(&self) -> WidgetId {
        WidgetId::NONE
    }
}

impl Widget for Empty {
    #[inline]
    fn update_layout<R: Resources>(&mut self, _parent_rect: Rect, _resources: &mut R) {}

    #[inline]
    fn draw<B: DrawBackend>(&self, _dc: DrawContext<B>) {}

    #[inline]
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }

    #[inline]
    fn event_consumed(&mut self, _event: &Event, _ctx: &EventContext) {}
}

impl From<Rect> for Empty {
    #[inline]
    fn from(bounds: Rect) -> Self {
        Empty::with_bounds(bounds)
    }
}

impl From<Size> for Empty {
    #[inline]
    fn from(size: Size) -> Self {
        Empty::with_size(size)
    }
}
