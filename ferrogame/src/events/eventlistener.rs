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

    /// # on_key_event
    /// This function should be called when an event in the category Key is created.
    fn on_key_event(&mut self, event: &super::Event);

    /// # on_mouse_event
    /// This function should be called when an event in the category Mouse is created.
    fn on_mouse_event(&mut self, event: &super::Event);

    /// # on_window_event
    /// This function should be called when an event in the category Window is created.
    fn on_window_event(&mut self, event: &super::Event);
}