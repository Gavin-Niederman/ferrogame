use std::rc::Rc;
use std::thread;
use std::time::Duration;
use crate::{logger, window, layer::{layer , layerstack}, event::eventdispatcher::EventDispatcher, renderer};

///# Application
///The application struct is used to run anything made in Ferrogame.
///It stores the layerstack(used for rendering and event dispatching), the Window(gets events and dispatches them to the layerstack), and eventually the renderer.
///In order to run your application you should create a new application with Application::new() and then call Application.run() on it.
pub struct Application {
    window: window::Window,
    layerstack: Rc<layerstack::LayerStack>,
    renderer: renderer::Renderer,
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

        let mut window = window::Window::new(
            "sandbox",
            900, 
            900, 
            false,
            25,
            eventdispatcher,
        );

        let renderer = renderer::Renderer::new(window.get_glfw_window(), Rc::clone(&layerstack));

        let application = Application {
            window,
            layerstack: Rc::clone(&layerstack),
            renderer
        };

        application
    }
    
    pub fn run(&mut self) {
        log::info!("Started application");
        while !self.window.should_close() {
            self.window.on_update();
            self.renderer.render_frame();
            if !self.window.get_vsync() {
                thread::sleep(Duration::from_millis((1000.0 * 1.0 / self.window.get_frameratecap() as f64) as u64))
            }
        }
    }
}