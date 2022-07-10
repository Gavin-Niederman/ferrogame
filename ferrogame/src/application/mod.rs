use crate::{logger, window};

pub struct Application {
    window: window::Window,
}

impl Application {
    pub fn new() -> Application {
        let mut window = window::Window::new("Ferrogame", 800, 600, true);
        Application {
            window 
        }
    }

    pub fn run(&mut self) {
        logger::info("This is info".to_string());
        logger::warn("This is a warning".to_string());
        logger::error("This is an error".to_string());
        logger::debug("This is a debug message".to_string());
        
        logger::info("Started application".to_string());
        loop {
            self.window.on_update();
        }
    }
}