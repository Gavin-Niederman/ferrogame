use std::sync::mpsc::Receiver;
use glfw::{WindowEvent, Context};

use crate::logger;

pub struct Window {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    window: glfw::Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: glfw::Glfw,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, vsync: bool) -> Window {
        logger::info("Creating window...".to_string());
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(width, height, &title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        glfw.make_context_current(Some(&window));
        logger::info("Window created sucessfully.".to_string());
        gl::load_with(|name| window.get_proc_address(name) as *const _);
        Window {
            title: title.to_string(),
            width,
            height,
            vsync,
            window,
            reciever: events,
            glfw,
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

    pub fn set_vsync(&mut self, vsync: bool) {
        if self.vsync {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
        }
    
        self.vsync = vsync;
    }

    pub fn on_update(&mut self) {
        self.glfw.poll_events();
        self.window.swap_buffers();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

}