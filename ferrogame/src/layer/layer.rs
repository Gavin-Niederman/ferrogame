use crate::{event::Event};

pub struct Layer {
}

impl Layer {
    pub fn new() -> Layer {
        Layer {}
    }

    pub(crate) fn on_event(&self, _event: &Event) {
    
    }

    pub(crate) fn on_attach(&self) {

    }

    // pub(crate) fn on_detach(&self) {
    //
    // }

    pub(crate) fn on_update(&self) {
        
    }
}