fn main() {
    let eventlistener = sandbox::eventlistener::EventListener {};
    let mut sandbox = ferrogame::Application::new(Box::new(eventlistener));
    sandbox.run();
}
