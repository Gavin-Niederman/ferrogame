use std::sync::mpsc::Receiver;
use glfw::{WindowEvent, Context};

use crate::event::Event;
use crate::event::{self, eventdispatcher::EventDispatcher};

pub struct Window {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    frameratecap: u32,
    window: glfw::Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: glfw::Glfw,
    shouldclose: bool,
    eventdispatcher: EventDispatcher,
    //wgpu_instance: wgpu::Instance,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, vsync: bool, frameratecap: u32, eventdispatcher: EventDispatcher) -> Window {
        log::info!("Creating window...");
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut glfw_window, events) = glfw.create_window(
            width, 
            height, 
            title, 
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");
        glfw.make_context_current(Some(&glfw_window));
        log::info!("Window created sucessfully.");
        glfw_window.make_current();
        glfw_window.set_all_polling(true);
        let mut window = Window {
            title: title.to_string(),
            width,
            height,
            vsync,
            frameratecap,
            window: glfw_window,
            reciever: events,
            glfw,
            shouldclose: false,
            eventdispatcher,
            //wgpu_instance: wgpu::Instance::new(wgpu::Backends::VULKAN),
        };
        window.set_vsync(vsync);
        window
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_vsync(&self) -> bool {
        self.vsync
    }

    pub fn get_frameratecap(&self) -> u32 {
        self.frameratecap
    }

    pub fn get_glfw_window(&mut self) -> &mut glfw::Window {
        &mut self.window
    }

    pub fn should_close(&self) -> bool {
        self.shouldclose
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
        self.eventdispatcher.dispatch_update();
    }

    fn create_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.reciever) {
            match event {
                WindowEvent::Close => {
                    let event = Event::new(event::EventType::WindowClosed);
                    self.eventdispatcher.dispatch_event(&event);
                    self.shouldclose = true;
                },
                WindowEvent::Key(key, _, action, _) => {
                    let event = match action {
                        glfw::Action::Press => Event::new(event::EventType::KeyPressed(event::Key::from_glfw_key(key), false)),
                        glfw::Action::Release => Event::new(event::EventType::KeyReleased(event::Key::from_glfw_key(key))),
                        glfw::Action::Repeat => Event::new(event::EventType::KeyPressed(event::Key::from_glfw_key(key), true)),
                    };
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::MouseButton(button, action, _) => {
                    let event = match action {
                        glfw::Action::Press => Event::new(event::EventType::MouseButtonPressed(event::MouseButton::from_glfw_mouse_button(button))),
                        glfw::Action::Release => Event::new(event::EventType::MouseButtonReleased(event::MouseButton::from_glfw_mouse_button(button))),
                        _ => Event::new(event::EventType::MouseButtonPressed(event::MouseButton::from_glfw_mouse_button(button))),
                    };
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::CursorPos(x, y) => {
                    let event = Event::new(event::EventType::MouseMoved((x, y)));
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::Scroll(x, y) => {
                    let event = Event::new(event::EventType::MouseScrolled((x, y)));
                    self.eventdispatcher.dispatch_event(&event);
                },
                WindowEvent::Focus(focused) => {
                    if focused { 
                        let event = Event::new(event::EventType::WindowFocused); 
                        self.eventdispatcher.dispatch_event(&event);
                    }
                },
                WindowEvent::Iconify(_) => {
                    let event = Event::new(event::EventType::WindowMinimized);
                    self.eventdispatcher.dispatch_event(&event);
                },
                _ => {}
            }
        }   
    }
}
