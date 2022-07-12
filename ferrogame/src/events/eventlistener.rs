pub trait EventListener {
    fn on_event(&mut self, event: &super::Event) {
        match event.get_event_category() {
            super::EventCategory::Key => {
                self.on_key_event(event);
            }
            super::EventCategory::Mouse => {
                self.on_mouse_event(event);
            }
            super::EventCategory::Window => {
                self.on_window_event(event);
            }
        }
    }

    fn on_key_event(&mut self, event: &super::Event);

    fn on_mouse_event(&mut self, event: &super::Event);

    fn on_window_event(&mut self, event: &super::Event);
}