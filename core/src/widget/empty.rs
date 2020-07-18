use crate::draw::{DrawContext, Vertex};
use crate::event::{Event, EventContext, EventResult};
use crate::geometry::{Bounds, Position, Rect, Size};
use crate::visitor::{Visitable, Visitor};
use crate::widget::{ObjectId, Widget, WidgetId};

/// The empty widget.
///
/// It's a "null" widget that does nothing (it only occupies space).
/// Can be used as a filler.
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
        WidgetId::EMPTY
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
    fn set_position(&mut self, position: Position) {
        self.bounds.pos = position;
    }

    #[inline]
    fn set_size(&mut self, size: Size) {
        self.bounds.size = size;
    }

    #[inline]
    fn get_bounds(&self) -> Rect {
        self.bounds
    }
}

impl Visitable for Empty {
    #[inline]
    fn accept<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        visitor.visit(self, ctx)
    }

    #[inline]
    fn accept_rev<V: Visitor>(&mut self, visitor: &mut V, ctx: &V::Context) -> Result<(), V::Return> {
        visitor.visit(self, ctx)
    }
}

impl Widget for Empty {
    #[inline]
    fn update_layout(&mut self, _parent_rect: Rect) {}

    #[inline]
    fn draw<V: Vertex>(&self, _dc: DrawContext<V>) {}

    #[inline]
    fn handle_event(&mut self, _event: &Event, _ctx: EventContext) -> EventResult {
        EventResult::Pass
    }

    #[inline]
    fn event_consumed(&mut self, _event: &Event, _ctx: &EventContext) {}
}

impl From<()> for Empty {
    #[inline]
    fn from(_: ()) -> Self {
        Empty::new()
    }
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
