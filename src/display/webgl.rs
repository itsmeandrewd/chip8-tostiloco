use crate::display::Display;
use log::debug;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation,
};

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

const SUPER_CHIP8_WIDTH: u8 = 128;
const SUPER_CHIP8_HEIGHT: u8 = 64;

pub struct WebGLDisplay {
    gl_context: WebGl2RenderingContext,
    vram: [u8; (CHIP8_HEIGHT * CHIP8_WIDTH)],
    color_uniform_location: Option<WebGlUniformLocation>,
    canvas: HtmlCanvasElement,
}

impl Default for WebGLDisplay {
    fn default() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        // prevent webgl from clearing the buffer on each draw automatically so we can
        // preserve our previous pixels
        let context_options = js_sys::Object::new();
        js_sys::Reflect::set(
            &context_options,
            &"preserveDrawingBuffer".into(),
            &wasm_bindgen::JsValue::TRUE,
        );
        let canvas = document
            .query_selector("#glCanvas")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let gl_context = canvas
            .get_context_with_context_options("webgl2", &context_options)
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        Self {
            gl_context,
            vram: [0; (CHIP8_HEIGHT * CHIP8_WIDTH) as usize],
            color_uniform_location: None,
            canvas,
        }
    }
}

impl Display for WebGLDisplay {
    fn clear(&mut self) {
        self.vram.iter_mut().for_each(|m| *m = 0);
        self.gl_context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl_context
            .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    fn get_width(&self) -> usize {
        CHIP8_WIDTH
    }

    fn get_height(&self) -> usize {
        CHIP8_HEIGHT
    }

    fn draw_pixel(&mut self, x: usize, y: usize, block_size: f32, turn_on: bool) {
        // set color
        if turn_on {
            self.gl_context
                .uniform4f(self.color_uniform_location.as_ref(), 0.5, 0.1, 0.3, 1.0);
            self.vram[y * self.get_width() + x] = 1;
        } else {
            self.gl_context
                .uniform4f(self.color_uniform_location.as_ref(), 0.0, 0.0, 0.0, 1.0);
            self.vram[y * self.get_width() + x] = 0;
        }


        let x1 = x as f32 * block_size;
        let x2 = x1 + block_size;
        let y1 = y as f32 * block_size;
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

        self.gl_context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.vram[y * self.get_width() + x] == 1
    }

    fn initialize(&mut self) {
        let program = self.get_program();

        let position_attribute_location =
            self.gl_context.get_attrib_location(&program, "a_position");

        let resolution_uniform_location = self
            .gl_context
            .get_uniform_location(&program, "u_resolution");
        self.color_uniform_location = self.gl_context.get_uniform_location(&program, "u_color");

        let position_buffer = self.gl_context.create_buffer().unwrap();

        let vao = self.gl_context.create_vertex_array();
        self.gl_context.bind_vertex_array(vao.as_ref());
        self.gl_context
            .enable_vertex_attrib_array(position_attribute_location as u32);
        self.gl_context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        self.gl_context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,                             // components per iteration
            WebGl2RenderingContext::FLOAT, // data type
            false,                         // data normalization
            0,                             // stride
            0,                             // offset
        );

        self.gl_context.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        self.gl_context.clear_color(0.0, 0.0, 0.0, 0.0);
        self.gl_context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        self.gl_context.bind_vertex_array(vao.as_ref());

        // set resolution
        self.gl_context.uniform2f(
            resolution_uniform_location.as_ref(),
            self.canvas.width() as f32,
            self.canvas.height() as f32,
        );
    }
}

impl WebGLDisplay {
    fn get_program(&self) -> WebGlProgram {
        let vertex_shader = self.compile_shader(
            WebGl2RenderingContext::VERTEX_SHADER,
            r#"#version 300 es

            in vec2 a_position;
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
            r#"#version 300 es

            precision highp float;
            uniform vec4 u_color;

            out vec4 outColor;

            void main() {
               outColor = u_color;
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
