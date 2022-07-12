use std::sync::mpsc::Receiver;
use glfw::{WindowEvent, Context};

use crate::logger;
use crate::events::{self, eventlistener::EventListener, eventdispatcher::EventDispatcher};

pub struct Window {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    window: glfw::Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: glfw::Glfw,
    shouldclose: bool,
    eventdispatcher: EventDispatcher,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, vsync: bool, eventlistener: Box<dyn EventListener>) -> Window {
        logger::info("Creating window...".to_string());
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(width, height, &title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        glfw.make_context_current(Some(&window));
        logger::info("Window created sucessfully.".to_string());
        window.make_current();
        window.set_all_polling(true);
        Window {
            title: title.to_string(),
            width,
            height,
            vsync,
            window,
            reciever: events,
            glfw,
            shouldclose: false,
            eventdispatcher: EventDispatcher::new(eventlistener),
        }
    }

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_vsync(&self) -> bool {
        return self.vsync;
    }

    pub fn should_close(&self) -> bool {
        return self.shouldclose;
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        if self.vsync {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
        }
    
        self.vsync = vsync;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window.set_title(&self.title);
    }

    pub fn on_update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        self.create_events();
    }

    fn create_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.reciever) {
            match event {
                WindowEvent::Close => {
                    let event = events::Event::WindowClosed;
                    self.eventdispatcher.dispatch_event(&event);
                    self.shouldclose = true;
                },
                WindowEvent::Key(key, _, action, _) => {
                    let event = match action {
                        glfw::Action::Press => events::Event::KeyPressed(events::Key::from_glfw_key(key), false),
                        glfw::Action::Release => events::Event::KeyReleased(events::Key::from_glfw_key(key)),
                        glfw::Action::Repeat => events::Event::KeyPressed(events::Key::from_glfw_key(key), true),
                    };
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::MouseButton(button, action, _) => {
                    let event = match action {
                        glfw::Action::Press => events::Event::MouseButtonPressed(events::MouseButton::from_glfw_mouse_button(button)),
                        glfw::Action::Release => events::Event::MouseButtonReleased(events::MouseButton::from_glfw_mouse_button(button)),
                        _ => events::Event::MouseButtonPressed(events::MouseButton::from_glfw_mouse_button(button)),
                    };
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::CursorPos(x, y) => {
                    let event = events::Event::MouseMoved((x, y));
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::Scroll(x, y) => {
                    let event = events::Event::MouseScrolled(x, y);
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::Focus(focused) => {
                    if focused { 
                        let event = events::Event::WindowFocused; 
                        self.eventdispatcher.dispatch_event(&event);
                    }
                },
                WindowEvent::Iconify(_) => {
                    let event = events::Event::WindowMinimized;
                    self.eventdispatcher.dispatch_event(&event);
                },
                _ => {}
            }
        }   
    }
}
