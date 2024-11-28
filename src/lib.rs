use std::{ffi::{c_char, CStr}, mem};

mod ffi;
mod gl;
mod retro;

pub struct TestGL {
    width: u32,
    height: u32,
    prog: u32,
    vbo: [u32; 1],
    context_alive: bool,
    multisample_fbo: bool,
    multisample: u32,
    vao: [u32; 1],
    fbo: u32,
    rbo_color: u32,
    rbo_depth_stencil: u32,
    frame_count: u32,

    gl: Option<gl::Context>,
}

impl TestGL {
    pub const BASE_WIDTH: u32 = 320;
    pub const BASE_HEIGHT: u32 = 240;
    pub const MAX_WIDTH: u32 = 2048;
    pub const MAX_HEIGHT: u32 = 2048;

    pub const fn new() -> Self {
        Self {
            width: Self::BASE_WIDTH,
            height: Self::BASE_HEIGHT,
            prog: 0,
            vbo: [0],
            context_alive: false,
            multisample_fbo: false,
            multisample: 0,
            vao: [0],
            fbo: 0,
            rbo_color: 0,
            rbo_depth_stencil: 0,
            frame_count: 0,
            gl: None,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    const VERTEX_SHADER: [*const c_char; 1] = [
c"#version 140
uniform mat4 uMVP;
in vec2 aVertex;
in vec4 aColor;
out vec4 color;
void main() {
    gl_Position = uMVP * vec4(aVertex, 0.0, 1.0);
    color = aColor;
}".as_ptr()];

    const FRAGMENT_SHADER: [*const c_char; 1] = [
c"#version 140
in vec4 color;
out vec4 FragColor;
void main() {
    FragColor = color;
}".as_ptr()];

    fn compile_program(&mut self) {
        let gl = self.gl.as_ref().unwrap();
        unsafe {
            self.prog = gl.create_program();
            let vert = gl.create_shader(gl::VERTEX_SHADER);
            let frag = gl.create_shader(gl::FRAGMENT_SHADER);

            gl.shader_source(vert, &Self::VERTEX_SHADER, None);
            gl.shader_source(frag, &Self::FRAGMENT_SHADER, None);
            gl.compile_shader(vert);
            gl.compile_shader(frag);

            gl.attach_shader(self.prog, vert);
            gl.attach_shader(self.prog, frag);
            gl.link_program(self.prog);
            gl.delete_shader(vert);
            gl.delete_shader(frag);
        }
    }

    fn setup_vao(&mut self) {
        let gl = self.gl.as_ref().unwrap();

        const VERTEX_DATA: [f32; 24] = [
            -0.5, -0.5,
            0.5, -0.5,
            -0.5, 0.5,
            0.5, 0.5,
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 0.0, 1.0,
            0.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 1.0,
        ];

        unsafe {
            gl.gen_vertex_arrays(&mut self.vao);

            gl.use_program(self.prog);

            gl.gen_buffers(&mut self.vbo);
            gl.bind_buffer(gl::ARRAY_BUFFER, self.vbo[0]);
            gl.buffer_data(gl::ARRAY_BUFFER, &VERTEX_DATA, gl::STATIC_DRAW);

            gl.bind_buffer(gl::ARRAY_BUFFER, 0);
            gl.use_program(0);
        }
    }

    pub fn run(&mut self, framebuffer: u32) {
        let gl = self.gl.as_ref().unwrap();

        unsafe {
            gl.bind_vertex_array(self.vao[0]);
            gl.bind_framebuffer(gl::FRAMEBUFFER, framebuffer);

            gl.clear_color(0.3, 0.4, 0.5, 1.0);
            gl.viewport(0, 0, self.width as _, self.height as _);
            gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            gl.use_program(self.prog);

            gl.enable(gl::DEPTH_TEST);

            gl.bind_buffer(gl::ARRAY_BUFFER, self.vbo[0]);
            let vloc = gl.get_attrib_location(self.prog, c"aVertex");
            gl.vertex_attrib_pointer(vloc as _, 2, gl::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(vloc as _);
            let cloc = gl.get_attrib_location(self.prog, c"aColor");
            gl.vertex_attrib_pointer(cloc as _, 4, gl::FLOAT, false, 0, 8 * mem::size_of::<f32>());
            gl.enable_vertex_attrib_array(cloc as _);
            gl.bind_buffer(gl::ARRAY_BUFFER, 0);

            let loc = gl.get_uniform_location(self.prog, c"uMVP");

            let angle = self.frame_count as f32 / 100.0;
            let mut cos_angle = angle.cos();
            let mut sin_angle = angle.sin();

            let mvp = [
                cos_angle, -sin_angle, 0.0, 0.0,
                sin_angle, cos_angle, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ];
            gl.uniform_matrix_4fv(loc, 1, false, &mvp);
            gl.draw_arrays(gl::TRIANGLE_STRIP, 0, 4);

            cos_angle *= 0.5;
            sin_angle *= 0.5;
            let mvp2 = [
                cos_angle, -sin_angle, 0.0, 0.0,
                sin_angle, cos_angle, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.4, 0.4, 0.2, 1.0,
            ];

            gl.uniform_matrix_4fv(loc, 1, false, &mvp2);
            gl.draw_arrays(gl::TRIANGLE_STRIP, 0, 4);
            gl.disable_vertex_attrib_array(vloc as _);
            gl.disable_vertex_attrib_array(cloc as _);

            gl.use_program(0);

            gl.bind_vertex_array(0);

            self.frame_count += 1;
        }
    }

    pub fn context_reset<F>(&mut self, get_proc_address: F)
        where F: FnMut(&str) -> Option<unsafe extern "C" fn()> {
        self.gl = Some(gl::Context::load(get_proc_address));
        self.compile_program();
        self.setup_vao();
    }

    pub fn context_destroy(&mut self) {
        let gl = self.gl.as_ref().unwrap();
        unsafe {
            gl.delete_vertex_arrays(&self.vao);
            self.vao[0] = 0;
            gl.delete_buffers(&self.vbo);
            self.vbo[0] = 0;
            gl.delete_program(self.prog);
            self.prog = 0;
        }
        self.gl = None;
    }
}