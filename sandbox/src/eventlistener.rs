use ferrogame::{event::Event, logger};

pub struct EventListener {

}

impl ferrogame::event::eventlistener::EventListener for EventListener {

    fn on_key_event(&mut self, event: &Event) {
        match event.get_event_type() {
            ferrogame::event::EventType::KeyPressed(_key, _repeat) => {
                logger::info("Key pressed".to_string());
            }
            _ => {}
        }
    }

    fn on_mouse_event(&mut self, event: &Event) {
        match event.get_event_type() {
            ferrogame::event::EventType::MouseButtonPressed(_button) => {
                logger::info("Mouse pressed".to_string());
            }
            _ => {}
        }
    }

    fn on_window_event(&mut self, event: &Event) {
        match event.get_event_type() {
            ferrogame::event::EventType::WindowFocused => {
                logger::info("Window focused".to_string());
            }
            _ => {}
        }
    }
}