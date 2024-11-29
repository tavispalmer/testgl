use std::{ffi::{c_char, c_void, CStr}, mem};

mod ffi;
mod gl;
mod retro;

pub struct TestGL {
    width: u32,
    height: u32,
    prog: u32,
    vbo: [u32; 1],
    #[cfg(feature = "core")]
    context_alive: bool,
    #[cfg(feature = "core")]
    multisample_fbo: bool,
    #[cfg(feature = "core")]
    multisample: u32,
    #[cfg(feature = "core")]
    vao: [u32; 1],
    #[cfg(feature = "core")]
    fbo: [u32; 1],
    #[cfg(feature = "core")]
    rbo_color: [u32; 1],
    #[cfg(feature = "core")]
    rbo_depth_stencil: [u32; 1],
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
            #[cfg(feature = "core")]
            context_alive: false,
            #[cfg(feature = "core")]
            multisample_fbo: false,
            #[cfg(feature = "core")]
            multisample: 0,
            #[cfg(feature = "core")]
            vao: [0],
            #[cfg(feature = "core")]
            fbo: [0],
            #[cfg(feature = "core")]
            rbo_color: [0],
            #[cfg(feature = "core")]
            rbo_depth_stencil: [0],
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

    fn compile_program(&mut self) {
        #[cfg(feature = "core")]
        const VERTEX_SHADER: [*const c_char; 9] = [
            c"#version 140\n".as_ptr(),
            c"uniform mat4 uMVP;".as_ptr(),
            c"in vec2 aVertex;".as_ptr(),
            c"in vec4 aColor;".as_ptr(),
            c"out vec4 color;".as_ptr(),
            c"void main() {".as_ptr(),
            c"  gl_Position = uMVP * vec4(aVertex, 0.0, 1.0);".as_ptr(),
            c"  color = aColor;".as_ptr(),
            c"}".as_ptr(),
        ];
        
        #[cfg(feature = "core")]
        const FRAGMENT_SHADER: [*const c_char; 6] = [
            c"#version 140\n".as_ptr(),
            c"in vec4 color;".as_ptr(),
            c"out vec4 FragColor;".as_ptr(),
            c"void main() {".as_ptr(),
            c"  FragColor = color;".as_ptr(),
            c"}".as_ptr(),
        ];
        
        #[cfg(not(feature = "core"))]
        const VERTEX_SHADER: [*const c_char; 8] = [
            c"uniform mat4 uMVP;".as_ptr(),
            c"attribute vec2 aVertex;".as_ptr(),
            c"attribute vec4 aColor;".as_ptr(),
            c"varying vec4 color;".as_ptr(),
            c"void main() {".as_ptr(),
            c"  gl_Position = uMVP * vec4(aVertex, 0.0, 1.0);".as_ptr(),
            c"  color = aColor;".as_ptr(),
            c"}".as_ptr(),
        ];

        #[cfg(not(feature = "core"))]
        const FRAGMENT_SHADER: [*const c_char; 7] = [
            c"#ifdef GL_ES\n".as_ptr(),
            c"precision mediump float;\n".as_ptr(),
            c"#endif\n".as_ptr(),
            c"varying vec4 color;".as_ptr(),
            c"void main() {".as_ptr(),
            c"  gl_FragColor = color;".as_ptr(),
            c"}".as_ptr(),
        ];

        let gl = self.gl.as_ref().unwrap();
        unsafe {
            self.prog = gl.create_program();
            let vert = gl.create_shader(gl::VERTEX_SHADER);
            let frag = gl.create_shader(gl::FRAGMENT_SHADER);

            gl.shader_source(vert, &VERTEX_SHADER, None);
            gl.shader_source(frag, &FRAGMENT_SHADER, None);
            gl.compile_shader(vert);
            gl.compile_shader(frag);

            gl.attach_shader(self.prog, vert);
            gl.attach_shader(self.prog, frag);
            gl.link_program(self.prog);
            gl.delete_shader(vert);
            gl.delete_shader(frag);
        }
    }

    #[cfg(feature = "core")]
    pub fn init_multisample(&mut self, samples: u32) {
        self.multisample = samples;
        if !self.context_alive {
            return;
        }

        let gl = self.gl.as_ref().unwrap();

        unsafe {
            if self.rbo_color[0] != 0 {
                gl.delete_renderbuffers(&self.rbo_color);
            }
            if self.rbo_depth_stencil[0] != 0 {
                gl.delete_renderbuffers(&self.rbo_depth_stencil);
            }
            if self.fbo[0] != 0 {
                gl.delete_framebuffers(&self.fbo);
            }

            self.rbo_color[0] = 0;
            self.rbo_depth_stencil[0] = 0;
            self.fbo[0] = 0;
            self.multisample_fbo = false;
            if samples <= 1 {
                return;
            }

            gl.gen_renderbuffers(&mut self.rbo_color);
            gl.gen_renderbuffers(&mut self.rbo_depth_stencil);
            gl.gen_framebuffers(&mut self.fbo);

            gl.bind_renderbuffer(gl::RENDERBUFFER, self.rbo_color[0]);
            gl.renderbuffer_storage_multisample(gl::RENDERBUFFER,
                samples as _, gl::RGBA, Self::MAX_WIDTH as _, Self::MAX_HEIGHT as _);
            gl.bind_renderbuffer(gl::RENDERBUFFER, self.rbo_depth_stencil[0]);
            gl.renderbuffer_storage_multisample(gl::RENDERBUFFER,
                samples as _, gl::DEPTH24_STENCIL8, Self::MAX_WIDTH as _, Self::MAX_HEIGHT as _);
            gl.bind_renderbuffer(gl::RENDERBUFFER, 0);

            gl.gen_framebuffers(&mut self.fbo);
            gl.bind_framebuffer(gl::FRAMEBUFFER, self.fbo[0]);

            gl.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                gl::RENDERBUFFER, self.rbo_color[0]);
            gl.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER, self.rbo_depth_stencil[0]);
            
            let ret = gl.check_framebuffer_status(gl::FRAMEBUFFER);
            if ret == gl::FRAMEBUFFER_COMPLETE {
                eprintln!("Using multisampled FBO.");
                self.multisample_fbo = true;
            } else {
                eprintln!("Multisample FBO failed.");
            }

            gl.bind_framebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    fn setup_vao(&mut self) {
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

        let gl = self.gl.as_ref().unwrap();

        unsafe {
            #[cfg(feature = "core")]
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
            #[cfg(feature = "core")]
            {
                gl.bind_vertex_array(self.vao[0]);
                if self.multisample_fbo {
                    gl.bind_framebuffer(gl::FRAMEBUFFER, self.fbo[0]);
                } else {
                    gl.bind_framebuffer(gl::FRAMEBUFFER, framebuffer);
                }
            }

            #[cfg(not(feature = "core"))]
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

            #[cfg(feature = "core")]
            {
                gl.bind_vertex_array(0);
                if self.multisample_fbo {
                    gl.bind_framebuffer(gl::READ_FRAMEBUFFER, self.fbo[0]);
                    gl.bind_framebuffer(gl::DRAW_FRAMEBUFFER, framebuffer);
                    gl.blit_framebuffer(0, 0, self.width as _, self.height as _,
                        0, 0, self.width as _, self.height as _,
                        gl::COLOR_BUFFER_BIT, gl::NEAREST);
                    gl.bind_framebuffer(gl::READ_FRAMEBUFFER, 0);
                    gl.bind_framebuffer(gl::DRAW_FRAMEBUFFER, 0);
                }
            }

            self.frame_count += 1;
        }
    }

    pub fn context_reset<F>(&mut self, get_proc_address: F)
        where F: FnMut(&CStr) -> *const c_void {
        self.gl = Some(gl::Context::load(get_proc_address));
        self.compile_program();
        self.setup_vao();
        #[cfg(feature = "core")]
        {
            self.context_alive = true;
            self.init_multisample(self.multisample);
        }
    }

    pub fn context_destroy(&mut self) {
        unsafe {
            #[cfg(feature = "core")]
            {
                self.gl.as_ref().unwrap().delete_vertex_arrays(&self.vao);
                self.vao[0] = 0;
                self.init_multisample(0);
                self.context_alive = false;
            }
            let gl = self.gl.as_ref().unwrap();
            gl.delete_buffers(&self.vbo);
            self.vbo[0] = 0;
            gl.delete_program(self.prog);
            self.prog = 0;
        }
    }
}

impl Drop for TestGL {
    fn drop(&mut self) {
        if self.gl.is_some() {
            self.context_destroy();
        }
    }
}
