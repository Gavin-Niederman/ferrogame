use std::rc::Rc;
use crate::{logger, window, layer::{layer , layerstack}, event::eventdispatcher::EventDispatcher};

pub struct Application {
    window: window::Window,
    layerstack: Rc<layerstack::LayerStack>,
}

impl Application {
    pub fn new() -> Application {
        logger::setup_logger().unwrap();
        let mut layerstack = Rc::new(layerstack::LayerStack::new());

        if let Some(layerstack) = Rc::get_mut(&mut layerstack) {
            log::info!("World layer pushed.");
            layerstack.push_layer(layer::Layer::new());
        }

        let eventdispatcher = EventDispatcher::new(Rc::clone(&layerstack));

        let window = window::Window::new(
            "sandbox",
            1080, 
            720, 
            true,
            144,
            eventdispatcher,
        );
        let application = Application {
            window,
            layerstack: Rc::clone(&layerstack),
        };

        application
    }
    
    pub fn run(&mut self) {
        log::info!("Started application");
        while !self.window.should_close() {
            self.window.on_update();
        }
    }
}