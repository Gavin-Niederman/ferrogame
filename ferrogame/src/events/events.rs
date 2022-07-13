/// # Event
/// The event enum contains variants for all events, each variant stores important data for the event.
/// For instance the MouseMoved event stores the mouse position moved to. 
pub enum Event {
    KeyPressed(super::Key, bool),
    KeyReleased(super::Key),
    MouseButtonPressed(super::MouseButton,),
    MouseButtonReleased(super::MouseButton,),
    MouseMoved((f64, f64)),
    MouseScrolled(f64, f64),
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
    pub fn get_event_category(&self) -> EventCategory {
        match self {
            Event::KeyPressed(_, _) => EventCategory::Key,
            Event::KeyReleased(_) => EventCategory::Key,
            Event::MouseButtonPressed(_) => EventCategory::Mouse,
            Event::MouseButtonReleased(_) => EventCategory::Mouse,
            Event::MouseMoved(_) => EventCategory::Mouse,
            Event::MouseScrolled(_, _) => EventCategory::Mouse,
            Event::WindowFocused => EventCategory::Window,
            Event::WindowUnfocused => EventCategory::Window,
            Event::WindowMinimized => EventCategory::Window,
            Event::WindowClosed => EventCategory::Window,
        }
    }
}