use crate::draw::{Color, DrawContext, DrawQueue};
use crate::event::{self, AxisValue, EvData, Event, EventResult};
use crate::geometry::{Pointi, Size};
use crate::widget::{TopLevel, Widget};
use std::ops;

pub const DEFAULT_WINDOW_SIZE: Size = Size::new(320, 240);

#[derive(Debug, Clone)]
pub struct Window<T> {
    /// The window attributes.
    pub attr: WindowAttributes,
    /// If the pointer is currently inside/outside.
    was_inside: bool,
    /// Window content.
    pub child: T,
}

impl<T> Window<T> {
    /// Creates a new window with default attributes.
    pub fn new(child: T) -> Self {
        Window {
            attr: Default::default(),
            was_inside: false,
            child,
        }
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

impl<T: Widget> TopLevel for Window<T> {
    fn get_position(&self) -> Pointi {
        self.attr.position.unwrap_or_default()
    }

    fn set_position(&mut self, position: Pointi) {
        self.attr.position = Some(position);
    }

    fn get_size(&self) -> Size {
        self.attr.size
    }

    fn set_size(&mut self, size: Size) {
        self.attr.size = size;
    }

    fn update(&mut self) {
        if self.attr.size.is_zero_area() {
            // our size is unset, first try to get the default content size
            let initial = self
                .child
                .get_bounds()
                .expand_to_origin()
                .map_size(|s| s.nonzero_or(DEFAULT_WINDOW_SIZE)); // if we failed to get a size then use a default

            // update the child's size using this size as our viewport
            self.child.update_size(initial);

            // set our size to the calculated content size
            let updated = self
                .child
                .get_bounds()
                .expand_to_origin()
                .size
                .nonzero_or(DEFAULT_WINDOW_SIZE);
            self.set_size(updated);
        } else {
            // we alread have a size, only update child
            self.child.update_size(self.get_size().into());
        }
    }

    fn draw(&self, drawq: &mut DrawQueue) {
        let mut dc = DrawContext::new(drawq, self.get_size().into());
        if let Some(bg) = self.attr.background {
            dc.clear(bg);
        }
        dc.draw_child(&self.child);
    }

    fn push_event(&mut self, event: Event) -> EventResult {
        // `bounds.pos` is relative to child's parent
        let bounds = self.child.get_bounds();
        // `pos` is relative to this object
        let pos = event.pointer_pos;
        // event.pointer_pos is now adjusted to child coordinates
        let event = Event {
            pointer_pos: event.pointer_pos - bounds.pos.cast().unwrap_or_default(),
            ..event
        };

        //TODO: keyboard focus
        match event.data {
            EvData::Keyboard { .. } => self.child.push_event(&event),
            EvData::Character(_) => self.child.push_event(&event),
            EvData::MouseMoved(AxisValue::Position(_)) => {
                if pos.inside(bounds) {
                    if !self.was_inside {
                        self.was_inside = true;
                        self.child.push_event(&event.with_data(EvData::PointerInside(true)))?;
                    }
                    self.child
                        .push_event(&event.with_data(EvData::MouseMoved(AxisValue::Position(event.pointer_pos))))
                } else {
                    if self.was_inside {
                        self.was_inside = false;
                        self.child.push_event(&event.with_data(EvData::PointerInside(false)))
                    } else {
                        event::EVENT_PASS
                    }
                }
            }
            EvData::MouseMoved(_) => {
                if pos.inside(bounds) {
                    self.child.push_event(&event)
                } else {
                    event::EVENT_PASS
                }
            }
            EvData::MouseButton { .. } => {
                if pos.inside(bounds) {
                    self.child.push_event(&event)
                } else {
                    event::EVENT_PASS
                }
            }
            EvData::PointerInside(_) => {
                if self.was_inside {
                    self.was_inside = false;
                    self.child.push_event(&event)
                } else {
                    event::EVENT_PASS
                }
            }
            EvData::FileDropped(_) => {
                if pos.inside(bounds) {
                    self.child.push_event(&event)
                } else {
                    event::EVENT_PASS
                }
            }
            _ => event::EVENT_PASS,
        }
    }

    fn get_window_attributes(&self) -> &WindowAttributes {
        &self.attr
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowAttributes {
    pub title: Option<String>,
    pub position: Option<Pointi>,
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
    pub fn set_position(&mut self, position: impl Into<Pointi>) {
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
