use std::sync::atomic::{AtomicUsize, Ordering};

static WIDGET_ID: AtomicUsize = AtomicUsize::new(1);

/// Unique widget global id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(usize);

impl WidgetId {
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
        Default::default()
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
        self.as_ref().map_or_else(Default::default, ObjectId::get_id)
    }
}

impl<T: ObjectId, E> ObjectId for Result<T, E> {
    #[inline]
    fn get_id(&self) -> WidgetId {
        self.as_ref().map_or_else(|_| Default::default(), ObjectId::get_id)
    }
}

impl<T: ObjectId> ObjectId for Box<T> {
    #[inline]
    fn get_id(&self) -> WidgetId {
        (**self).get_id()
    }
}
