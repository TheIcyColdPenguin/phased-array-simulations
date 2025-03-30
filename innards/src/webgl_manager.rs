// code derived from https://rustwasm.github.io/wasm-bindgen/examples/webgl.html

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

#[wasm_bindgen]
pub struct WebglManager {
    context: WebGl2RenderingContext,

    canvas_width: f32,
    canvas_height: f32,

    vert_count: i32,

    u_time_location: WebGlUniformLocation,
    u_resolution_location: WebGlUniformLocation,
    u_phase_location: WebGlUniformLocation,
    u_wavelength_location: WebGlUniformLocation,
    // u_frequency_location: WebGlUniformLocation,
    u_num_array_elements_location: WebGlUniformLocation,
    u_array_elements_location: WebGlUniformLocation,
}

#[wasm_bindgen]
impl WebglManager {
    pub fn new(
        context: WebGl2RenderingContext,
        canvas_width: f32,
        canvas_height: f32,
    ) -> Result<WebglManager, JsValue> {
        let vert_shader = Self::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("./shader.vert"),
        )?;
        let frag_shader = Self::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("./shader.frag"),
        )?;

        let program = Self::link_program(&context, &vert_shader, &frag_shader)?;

        let position_attribute_location = context.get_attrib_location(&program, "position");
        let vertices: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
        let vertex_buf = context.create_buffer().ok_or("Couldn't create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buf));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // create vertex array object
        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.bind_vertex_array(Some(&vao));

        let u_time_location = context
            .get_uniform_location(&program, "u_time")
            .ok_or("Couldn't get uniform location for u_time")?;
        let u_resolution_location = context
            .get_uniform_location(&program, "u_resolution")
            .ok_or("Couldn't get uniform location for u_resolution")?;
        let u_phase_location = context
            .get_uniform_location(&program, "u_phase")
            .ok_or("Couldn't get uniform location for u_phase")?;
        let u_wavelength_location = context
            .get_uniform_location(&program, "u_wavelength")
            .ok_or("Couldn't get uniform location for u_wavelength")?;
        // let u_frequency_location = context
        //     .get_uniform_location(&program, "u_frequency")
        //     .ok_or("Couldn't get uniform location for u_frequency")?;
        let u_num_array_elements_location = context
            .get_uniform_location(&program, "u_num_array_elements")
            .ok_or("Couldn't get uniform location for u_num_array_elements")?;
        let u_array_elements_location = context
            .get_uniform_location(&program, "u_array_elements")
            .ok_or("Couldn't get uniform location for u_array_elements")?;

        Ok(WebglManager {
            context,

            canvas_width,
            canvas_height,

            vert_count: vertices.len() as i32 / 2,

            u_time_location,
            u_resolution_location,
            u_wavelength_location,
            u_phase_location,
            // u_frequency_location,
            u_array_elements_location,
            u_num_array_elements_location,
        })
    }

    pub fn draw_frame(
        &self,
        u_time: f32,
        u_num_array_elements: usize,
        spacing: f32,
        u_frequency: f32,
        u_phase: f32,
        zoom: f32,
    ) {
        let context = &self.context;

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.uniform1f(Some(&self.u_time_location), u_time);
        context.uniform1f(Some(&self.u_phase_location), u_phase);
        context.uniform2f(
            Some(&self.u_resolution_location),
            self.canvas_width,
            self.canvas_height,
        );
        context.uniform1ui(
            Some(&self.u_num_array_elements_location),
            u_num_array_elements as u32,
        );

        let array_space = zoom; // fraction of height of screen

        let wavelength = array_space / (u_num_array_elements as f32 - 1.0) / spacing;
        context.uniform1f(Some(&self.u_wavelength_location), wavelength);

        // context.uniform1f(Some(&self.u_frequency_location), u_frequency);

        let x = vec![0.1f32; u_num_array_elements];
        let mut y = Vec::with_capacity(u_num_array_elements);
        for i in 0..u_num_array_elements {
            y.push(
                (1.0 - array_space) / 2.0
                    + (i as f32) * array_space / (u_num_array_elements as f32 - 1.0),
            );
        }

        let positions: Vec<f32> = x.into_iter().zip(y).flat_map(|(a, b)| [a, b]).collect();

        context
            .uniform2fv_with_f32_array(Some(&self.u_array_elements_location), positions.as_slice());

        context.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, self.vert_count);
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas_width = (0.75 * width).min(height);
        self.canvas_height = (0.75 * width).min(height);
    }

    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            context.use_program(Some(&program));
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
