//! Input event definitions.
use crate::geometry::{Point, Position, Size};
use crate::widget::WidgetId;
use std::path::PathBuf;
use std::time::Instant;

mod dispatcher;
pub use dispatcher::*;

pub use ButtonState::*;

/// Raw key id from hardware.
pub type ScanCode = u32;

/// Input events that come from the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Raw keyboard input.
    Keyboard { state: ButtonState, key: Key, scancode: ScanCode },
    /// Processed keyboard input as an unicode character.
    Character(char),
    /// Keyboard modifier state changed.
    ModifiersChanged(KeyModState),
    /// Mouse pointer motion.
    MouseMoved(Axis),
    /// Mouse button input.
    MouseButton(ButtonState, MouseButton),
    /// Pointer has crossed the window boundaries.
    PointerInside(bool),
    /// A file has been dropped into the window.
    FileDropped(PathBuf),
    /// Window resized.
    Resized(Size),
    /// Window moved.
    Moved(Position),
    /// Window focused state.
    Focused(bool),
    /// Window close button pressed.
    CloseRequest,
    /// Window has been created.
    Created,
    /// Window has been destroyed.
    Destroyed,
}

/// Data associated with an event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventContext {
    /// Instant when the event was received.
    pub timestamp: Instant,
    /// Last known cursor position, relative to the widget.
    pub local_pos: Point<f64>,
    /// Last known cursor position, relative to the window.
    pub abs_pos: Point<f64>,
    /// Current mouse button state.
    pub button_state: MouseButtonsState,
    /// Current keyboard modifier state.
    pub mod_state: KeyModState,
    /// The widget that has received the event.
    pub widget: WidgetId,
    /// The receiver widget's parent.
    pub parent: WidgetId,
}

impl EventContext {
    #[inline]
    fn update(self, offset: Point<f64>, widget: WidgetId, parent: WidgetId) -> Self {
        EventContext {
            local_pos: self.abs_pos - offset,
            widget,
            parent,
            ..self
        }
    }
}

/// State of keys or mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ButtonState {
    Released,
    Pressed,
}

impl Default for ButtonState {
    #[inline]
    fn default() -> Self {
        ButtonState::Released
    }
}

/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8),
}

impl MouseButton {
    /// Gets the button number.
    #[inline]
    pub fn number(self) -> u8 {
        match self {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
            MouseButton::Other(n) => n,
        }
    }

    /// Gets the bitmask for this button.
    #[inline]
    fn mask(self) -> u64 {
        match self {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 4,
            MouseButton::Other(n) => 1u64 << n,
        }
    }
}

/// Axis of movement for mouse pointer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    Position(Point<f64>),
    Scroll(f32, f32),
    Pressure(f64),
    Tilt(f64, f64),
}

/// Keyboard modifier state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyModState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Mouse button state for all buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MouseButtonsState(u64);

impl MouseButtonsState {
    /// Sets the specificed button as pressed.
    #[inline]
    pub fn set(&mut self, button: MouseButton) {
        self.0 |= button.mask();
    }

    /// Sets the specificed button as released.
    #[inline]
    pub fn unset(&mut self, button: MouseButton) {
        self.0 &= !button.mask();
    }

    /// Checks if the specified button is pressed.
    #[inline]
    pub fn is_set(self, button: MouseButton) -> bool {
        self.0 & button.mask() != 0
    }

    /// Checks if the left button is pressed.
    #[inline]
    pub fn left(self) -> bool {
        self.is_set(MouseButton::Left)
    }

    /// Checks if the middle button is pressed.
    #[inline]
    pub fn middle(self) -> bool {
        self.is_set(MouseButton::Middle)
    }

    /// Checks if the right button is pressed.
    #[inline]
    pub fn right(self) -> bool {
        self.is_set(MouseButton::Right)
    }
}

/// Side for duplicated modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeySide {
    Left,
    Right,
}

/// Defines if it's a key from the numpad.
pub type IsNumpad = bool;

/// Symbolic key definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// unknown key, raw id in scancode
    Unk,
    /// Number keys
    Num(u8, IsNumpad),
    /// Letters
    Letter(char),
    /// Function keys
    Fn(u8),
    /// The space bar
    Space,
    // Main control keys
    Escape,
    BackSpace,
    Tab,
    Enter(IsNumpad),
    CapsLock,
    Shift(KeySide),
    Control(KeySide),
    Alt(KeySide),
    Super(KeySide),
    Meta(KeySide),
    Compose,
    // Secondary control keys
    PrintScr,
    ScrollLock,
    Pause,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    // Arrow keys
    Up,
    Down,
    Left,
    Right,
    // Numpad extra
    NumLock,
    NumpadDelete,
    NumpadEnter,
    // Other stuff
    Plus(IsNumpad),
    Minus(IsNumpad),
    Multiply(IsNumpad),
    Slash(IsNumpad),
    Backslash,
    Comma(IsNumpad),
    Period,
    Colon,
    Semicolon,
    Apostrophe,
    Grave,
    LBracket,
    RBracket,
    Equals(IsNumpad),
}

/// The result of processing an input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventResult {
    /// Event was completely ignored, will propagate.
    Pass,
    /// Event was consumed, do not propagate. Will trigger a redraw.
    Consumed,
}

impl EventResult {
    #[inline]
    pub fn consumed(self) -> bool {
        matches!(self, EventResult::Consumed)
    }

    /// Returns `Some(val)` if the event was consumed, or `None` otherwise.
    #[inline]
    pub fn then_some<T>(self, val: T) -> Option<T> {
        match self {
            EventResult::Pass => None,
            EventResult::Consumed => Some(val),
        }
    }

    /// Returns `Some(f())` if the event was consumed, or `None` otherwise.
    #[inline]
    pub fn then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        match self {
            EventResult::Pass => None,
            EventResult::Consumed => Some(f()),
        }
    }
}

impl Default for EventResult {
    #[inline]
    fn default() -> Self {
        EventResult::Pass
    }
}
