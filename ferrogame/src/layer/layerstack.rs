use crate::{event::Event, logger};
use super::layer::Layer;

pub struct LayerStack {
    layers: Vec<Layer>,
    overlays: Vec<Layer>,
}

impl LayerStack {
    pub fn new() -> LayerStack {
        LayerStack {
            layers: Vec::new(),
            overlays: Vec::new(),
        }
    }

    //TODO: Add a way to remove layers

    pub fn push_layer(&mut self, layer: Layer) {
        layer.on_attach();
        self.layers.push(layer);
    }

    pub fn push_overlay(&mut self, overlay: Layer) {
        overlay.on_attach();
        self.layers.insert(0, overlay);
    }

    pub(crate) fn dispatch_event(&self, event: &Event) {
        for overlay in self.overlays.iter().rev() {
            overlay.on_event(event);
        }
        for layer in self.layers.iter().rev() {
            logger::debug("LayerStack::dispatch_event()".to_string()); //this is called
            layer.on_event(event); //this is not
        }
    }

    pub(crate) fn on_update(&self) {
        for layer in self.layers.iter() {
            layer.on_update();
        }
        for overlay in self.overlays.iter() {
            overlay.on_update();
        }
    }
}