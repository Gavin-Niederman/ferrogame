use ferrogame::window;

fn main() {
    let eventlistener = sandbox::eventlistener::EventListener {};
    let window = window::Window::new(
        "sandbox",
        1080, 
        720, 
        false, 
        Box::new(eventlistener)
    );
    let mut sandbox = ferrogame::Application::new(window);
    sandbox.run();
}
