/// The Event enum contains variants for all events
/// 
/// # What they store
/// - 'KeyPressed(i32, bool)' (keycode, repeat)
/// - 'KeyReleased(i32)' (keycode)
/// - 'MouseButtonPressed(i32, (f32, f32))' (button, (x, y))
/// - 'MouseButtonReleased(i32, (f32, f32))' (button, (x, y))
/// - 'MouseMoved((f32, f32))' ((x, y))
/// - 'MouseScrolled(i32)' (xOffset, yOffset)
pub enum Event {
    KeyPressed(i32, bool),
    KeyReleased(i32),
    MouseButtonPressed(i32, (f32, f32)),
    MouseButtonReleased(i32, (f32, f32)),
    MouseMoved((f32, f32)),
    MouseScrolled(f32, f32),
    WindowFocused,
    WindowUnfocused,
    WindowMinimized,
    WindowMoved,
    WindowClosed,
}

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
            Event::MouseButtonPressed(_, _) => EventCategory::Mouse,
            Event::MouseButtonReleased(_, _) => EventCategory::Mouse,
            Event::MouseMoved(_) => EventCategory::Mouse,
            Event::MouseScrolled(_, _) => EventCategory::Mouse,
            Event::WindowFocused => EventCategory::Window,
            Event::WindowUnfocused => EventCategory::Window,
            Event::WindowMinimized => EventCategory::Window,
            Event::WindowMoved => EventCategory::Window,
            Event::WindowClosed => EventCategory::Window,
        }
    }

    pub fn dispatch_event() {
        //TODO
    }
}