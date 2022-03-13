use log::debug;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub trait ScreenDisplay {
    fn clear(&self);
}

pub struct WebGLDisplay {
    gl_context: WebGl2RenderingContext,
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
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        Self { gl_context }
    }
}

impl WebGLDisplay {
    pub fn clear(&self) {
        debug!("CLS");
        self.gl_context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl_context
            .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn draw(&self, x: usize, y: usize, n: u8) {
        debug!("DRW V{}, V{}, {:#01x}", x, y, n);
        self.draw_box(1, 1, 50.0);
    }

    pub fn draw_box(&self, x: u8, y: u8, block_size: f32) {
        let x1 = x as f32;
        let x2 = x1 + block_size;
        let y1 = y as f32;
        let y2 = y1 + block_size;
        let buffer_data: [f32; 12] = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        unsafe {
            let positions_array_buffer_view = js_sys::Float32Array::view(&buffer_data);
            self.gl_context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        /*self.gl_context.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &buffer_data,
            WebGl2RenderingContext::STATIC_DRAW,
        );*/
        self.gl_context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
    }

    pub fn init(&self) {
        let program = self.get_program();
        let position_attribute_location =
            self.gl_context.get_attrib_location(&program, "a_position");
        let resolution_uniform_location = self
            .gl_context
            .get_uniform_location(&program, "u_resolution")
            .unwrap();
        let color_uniform_location = self
            .gl_context
            .get_uniform_location(&program, "u_color")
            .unwrap();

        let position_buffer = self.gl_context.create_buffer().unwrap();
        self.gl_context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        self.gl_context.viewport(0, 0, 1024, 960);
        self.gl_context
            .enable_vertex_attrib_array(position_attribute_location as u32);
        self.gl_context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        // set resolution
        self.gl_context
            .uniform2f(Some(&resolution_uniform_location), 1024.0, 960.0);

        // set color
        self.gl_context
            .uniform4f(Some(&color_uniform_location), 0.5, 0.1, 0.3, 1.0);
    }

    fn get_program(&self) -> WebGlProgram {
        let vertex_shader = self.compile_shader(
            WebGl2RenderingContext::VERTEX_SHADER,
            r#"
            attribute vec2 a_position;
            uniform vec2 u_resolution;

            void main() {
                // convert the rectangle from pixels to 0.0 to 1.0
               vec2 zeroToOne = a_position / u_resolution;

               // convert from 0->1 to 0->2
               vec2 zeroToTwo = zeroToOne * 2.0;

               // convert from 0->2 to -1->+1 (clipspace)
               vec2 clipSpace = zeroToTwo - 1.0;

               gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
            }
            "#,
        );
        let fragment_shader = self.compile_shader(
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r#"
            precision mediump float;
            uniform vec4 u_color;

            void main() {
               gl_FragColor = u_color;
            }
            "#,
        );

        let program = self
            .link_program(&vertex_shader.unwrap(), &fragment_shader.unwrap())
            .unwrap();
        self.gl_context.use_program(Some(&program));

        program
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = self
            .gl_context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        self.gl_context.shader_source(&shader, source);
        self.gl_context.compile_shader(&shader);

        if self
            .gl_context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .gl_context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
        &self,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = self
            .gl_context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        self.gl_context.attach_shader(&program, vert_shader);
        self.gl_context.attach_shader(&program, frag_shader);
        self.gl_context.link_program(&program);

        if self
            .gl_context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(self
                .gl_context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
