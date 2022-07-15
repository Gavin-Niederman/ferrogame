use std::rc::Rc;

use crate::{logger, window, layer::{layer , layerstack}, event::eventdispatcher::EventDispatcher};

pub struct Application {
    window: window::Window,
    layerstack: Rc<layerstack::LayerStack>,
}

impl Application {
    pub fn new() -> Application {
        let mut layerstack = Rc::new(layerstack::LayerStack::new());
        if let Some(layerstack) = Rc::get_mut(&mut layerstack) {
            logger::info("World layer pushed.".to_string());
            layerstack.push_layer(layer::Layer::new());
        }
        let eventdispatcher = EventDispatcher::new(Rc::clone(&layerstack));
        let window = window::Window::new(
            "sandbox",
            1080, 
            720, 
            false,
            eventdispatcher,
        );
        let application = Application {
            window,
            layerstack: Rc::clone(&layerstack),
        };
        application
    }
    
    pub fn run(&mut self) {
        //self.layerstack.push_layer(layer::Layer::new("gamelayer"));
        logger::info("Started application".to_string());
        while !self.window.should_close() {
            self.window.on_update();
        }
    }
}