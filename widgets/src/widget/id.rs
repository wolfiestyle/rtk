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
