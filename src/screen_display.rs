use log::{debug, info};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, WebGlRenderingContext};

pub trait ScreenDisplay {
    fn clear(&self);
}

pub struct WebGLDisplay {
    gl_context: WebGlRenderingContext,
}

impl Default for WebGLDisplay {
    fn default() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .query_selector("#glCanvas")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let gl_context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();
        Self { gl_context }
    }
}

impl WebGLDisplay {
    pub fn clear(&self) {
        debug!("CLS");
        self.gl_context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl_context
            .clear(WebGlRenderingContext::COLOR_BUFFER_BIT)
    }

    pub fn draw(&self, x: usize, y: usize, n: u8) {
        debug!("DRW V{}, V{}, {:#01x}", x, y, n)
    }
}
