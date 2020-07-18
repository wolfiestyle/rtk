use crate::draw::{Color, DrawContext, DrawQueue, Vertex};
use crate::event::{Event, EventDispatcher};
use crate::geometry::{Position, Size};
use crate::toplevel::TopLevel;
use crate::widget::Widget;
use std::ops;

pub const DEFAULT_WINDOW_SIZE: Size = Size::new(320, 240);

/// Top level Window.
#[derive(Debug, Clone, Default)]
pub struct Window<T> {
    /// The window attributes.
    pub attr: WindowAttributes,
    /// Event dispatcher
    dispatcher: EventDispatcher,
    /// Window content.
    pub child: T,
}

impl<T> Window<T> {
    /// Creates a new window with default attributes.
    pub fn new(child: T) -> Self {
        Window {
            attr: Default::default(),
            dispatcher: Default::default(),
            child,
        }
    }

    /// Creates a new window with the specified attributes.
    pub fn new_with_attr(child: T, attr: WindowAttributes) -> Self {
        Window {
            attr,
            dispatcher: Default::default(),
            child,
        }
    }
}

impl<T: Widget> TopLevel for Window<T> {
    fn update_layout(&mut self) {
        if self.size.is_zero_area() {
            // our size is unset, first try to get the default content size
            let initial = self
                .child
                .get_bounds()
                .expand_to_origin()
                .map_size(|s| s.nonzero_or(DEFAULT_WINDOW_SIZE)); // if we failed to get a size then use a default

            // update the child's size using this size as our viewport
            self.child.update_layout(initial);

            // set our size to the calculated content size
            let updated = self.child.get_bounds().expand_to_origin().size.nonzero_or(DEFAULT_WINDOW_SIZE);
            self.set_size(updated);
        } else {
            // we alread have a size, only update child
            self.child.update_layout(self.size.into());
        }
    }

    fn draw<V: Vertex>(&self, drawq: &mut DrawQueue<V>) {
        let mut dc = DrawContext::new(drawq, self.size.into());
        if let Some(bg) = self.attr.background {
            dc.clear(bg);
        }
        dc.draw_child(&self.child);
    }

    fn push_event(&mut self, event: Event) -> bool {
        self.dispatcher.dispatch_event(event, self.size, &mut self.child)
    }

    #[inline]
    fn get_attr(&self) -> &WindowAttributes {
        &self.attr
    }

    #[inline]
    fn get_attr_mut(&mut self) -> &mut WindowAttributes {
        &mut self.attr
    }
}

impl<T> ops::Deref for Window<T> {
    type Target = WindowAttributes;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.attr
    }
}

impl<T> ops::DerefMut for Window<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attr
    }
}

/// The attributes of a window.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowAttributes {
    pub title: Option<String>,
    pub position: Option<Position>,
    pub size: Size,
    pub min_size: Size,
    pub max_size: Size,
    pub background: Option<Color>,
    pub resizable: bool,
    pub maximized: bool,
    pub transparent: bool,
    pub always_on_top: bool,
    pub decorations: bool,
}

impl WindowAttributes {
    #[inline]
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }

    #[inline]
    pub fn set_position(&mut self, position: impl Into<Position>) {
        self.position = Some(position.into())
    }

    #[inline]
    pub fn set_size(&mut self, size: impl Into<Size>) {
        self.size = size.into();
    }

    #[inline]
    pub fn set_min_size(&mut self, size: impl Into<Size>) {
        self.min_size = size.into();
    }

    #[inline]
    pub fn set_max_size(&mut self, size: impl Into<Size>) {
        self.max_size = size.into();
    }

    #[inline]
    pub fn set_background(&mut self, background: impl Into<Color>) {
        self.background = Some(background.into())
    }
}

impl Default for WindowAttributes {
    #[inline]
    fn default() -> Self {
        WindowAttributes {
            title: None,
            position: None,
            size: Size::zero(),
            min_size: Size::zero(),
            max_size: Size::zero(),
            background: Some(Color::BLACK),
            resizable: true,
            maximized: false,
            transparent: false,
            always_on_top: false,
            decorations: true,
        }
    }
}
