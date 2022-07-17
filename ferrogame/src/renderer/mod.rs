extern crate gl;
use gl::types::*;
use crate::layer::layerstack::LayerStack;
use std::rc::Rc;

pub struct Renderer {
    layerstack: Rc<LayerStack>,
}

impl Renderer {
    pub fn new(window: &mut glfw::Window, layerstack: Rc<LayerStack>) -> Renderer {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        Renderer {
            layerstack,
        }
    }

    pub fn render_frame(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.4, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}