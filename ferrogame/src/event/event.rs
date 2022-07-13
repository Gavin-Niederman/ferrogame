/// # Event
/// The event struct contains all the information about an event.
/// Including the type, type data, and if it is handled or not.
pub struct Event {
    pub event_type: EventType,
    pub handled: bool,
}

pub enum EventType {
    KeyPressed(super::Key, bool),
    KeyReleased(super::Key),
    MouseButtonPressed(super::MouseButton),
    MouseButtonReleased(super::MouseButton),
    MouseMoved((f64, f64)),
    MouseScrolled((f64, f64)),
    WindowFocused,
    WindowUnfocused,
    WindowMinimized,
    WindowClosed,
}

/// # EventCategory
/// The EventCategory enum contains three variants:
/// - Key(Key being pressed or released etc.)
/// - Mouse(Mouse button being pressed or released etc.)
/// - Window(closed, focused etc.)
pub enum EventCategory {
    Key,
    Mouse,
    Window,
}

impl Event {

    pub fn new(event_type: EventType) -> Event {
        Event {
            event_type: event_type,
            handled: false,
        }
    }

    pub fn get_event_category(&self) -> EventCategory {
        match self.event_type {
            EventType::KeyPressed(_, _) => EventCategory::Key,
            EventType::KeyReleased(_) => EventCategory::Key,
            EventType::MouseButtonPressed(_) => EventCategory::Mouse,
            EventType::MouseButtonReleased(_) => EventCategory::Mouse,
            EventType::MouseMoved((_, _)) => EventCategory::Mouse,
            EventType::MouseScrolled((_, _)) => EventCategory::Mouse,
            EventType::WindowFocused => EventCategory::Window,
            EventType::WindowUnfocused => EventCategory::Window,
            EventType::WindowMinimized => EventCategory::Window,
            EventType::WindowClosed => EventCategory::Window,
        }
    }

    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn is_handled(&self) -> &bool {
        &self.handled
    }
}