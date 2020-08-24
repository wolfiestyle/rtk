use std::sync::atomic::{AtomicUsize, Ordering};

static WIDGET_ID: AtomicUsize = AtomicUsize::new(1);

/// Unique widget global id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WidgetId(usize);

impl WidgetId {
    /// Null widget id.
    ///
    /// This is the return value of `WidgetId::default()`, also the id of the Empty widget.
    pub const NONE: WidgetId = WidgetId(0);

    /// Creates a new widget id.
    #[inline]
    pub fn new() -> Self {
        let id = WIDGET_ID.fetch_add(1, Ordering::Relaxed);
        WidgetId(id)
    }
}

/// Objects that contain an unique Id.
pub trait ObjectId {
    /// Gets the widget id.
    fn get_id(&self) -> WidgetId;
}

impl ObjectId for () {
    #[inline]
    fn get_id(&self) -> WidgetId {
        WidgetId::NONE
    }
}

impl ObjectId for WidgetId {
    #[inline]
    fn get_id(&self) -> WidgetId {
        *self
    }
}

impl<T: ObjectId> ObjectId for Option<T> {
    #[inline]
    fn get_id(&self) -> WidgetId {
        self.as_ref().map_or(WidgetId::NONE, ObjectId::get_id)
    }
}

impl<T: ObjectId, E> ObjectId for Result<T, E> {
    #[inline]
    fn get_id(&self) -> WidgetId {
        self.as_ref().map_or(WidgetId::NONE, ObjectId::get_id)
    }
}
