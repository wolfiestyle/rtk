use rtk::backend::{DrawBackend, Resources};
use rtk::draw::DrawContext;
use rtk::event::{Event, EventContext, EventResult};
use rtk::geometry::{Bounds, BoundsMut, Position, Rect, Size};
use rtk::visitor::{Visitable, Visitor};
use rtk::widget::{ObjectId, Widget, WidgetId};

/// The empty widget.
///
/// It's a "null" widget that does nothing (it only occupies space).
/// Can be used as a filler in layout definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

impl Bounds for Empty {
    #[inline]
    fn get_position(&self) -> Position {
        self.bounds.pos
    }

    #[inline]
    fn get_size(&self) -> Size {
        self.bounds.size
    }

    #[inline]
    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}

impl BoundsMut for Empty {
    #[inline]
    fn set_position(&mut self, position: Position) {
        self.bounds.pos = position;
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        self.bounds.size = size;
    }

    #[inline]
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
}

impl Visitable for Empty {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: V, prev_ctx: &V::Context) -> V {
        if visitor.finished() {
            return visitor;
        }
        if let Some(ctx) = visitor.new_context(self, prev_ctx) {
            visitor.visit_before(self, &ctx).visit_after(self, &ctx)
        } else {
            visitor
        }
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
