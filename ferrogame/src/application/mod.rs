use crate::logger;

pub struct Application {

}

impl Application {
    pub fn new() -> Application {
        Application {

        }
    }

    pub fn run(&self) {
        logger::info("This is info".to_string());
        logger::warn("This is a warning".to_string());
        logger::error("This is an error".to_string());
        logger::debug("This is a debug message".to_string());
        
        logger::info("Starting application".to_string());
    }
}