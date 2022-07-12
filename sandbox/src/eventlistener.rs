use ferrogame::{events::Event, logger};

pub struct EventListener {

}

impl ferrogame::events::eventlistener::EventListener for EventListener {

    fn on_key_event(&mut self, event: &Event) {
        match event {
            ferrogame::events::Event::KeyPressed(_key, _repeat) => {
                logger::info("Key pressed".to_string());
            }
            _ => {}
        }
    }

    fn on_mouse_event(&mut self, event: &Event) {
        match event {
            ferrogame::events::Event::MouseButtonPressed(_button) => {
                logger::info("Mouse pressed".to_string());
            }
            _ => {}
        }
    }

    fn on_window_event(&mut self, event: &Event) {
        match event {
            ferrogame::events::Event::WindowFocused => {
                logger::info("Window focused".to_string());
            }
            _ => {}
        }
    }
}