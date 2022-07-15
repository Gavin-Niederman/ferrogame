use std::rc::Rc;

use crate::layer::layerstack::LayerStack;
use super::Event;

pub struct EventDispatcher {
    layerstack: Rc<LayerStack>,
}

impl EventDispatcher {
    pub fn new(layerstack: Rc<LayerStack>) -> EventDispatcher {
        EventDispatcher {
            layerstack,
        }
    }

    pub fn dispatch_event(&mut self, event: &Event) {
        self.layerstack.dispatch_event(event);
    }

    pub fn dispatch_update(&mut self) {
        self.layerstack.on_update();
    }
}

