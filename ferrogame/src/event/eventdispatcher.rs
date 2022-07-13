use super::{eventlistener::EventListener, Event};

pub struct EventDispatcher {
    eventlistener: Box<dyn EventListener>,
}

impl EventDispatcher {
    pub fn new(eventlistener: Box<dyn EventListener>) -> EventDispatcher {
        EventDispatcher {
            eventlistener,
        }
    }

    //TODO: Rework event dispatching to use layers

    pub fn dispatch_event(&mut self, event: &Event) {
        self.eventlistener.on_event(event);
    }
}

