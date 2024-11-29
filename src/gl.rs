#![allow(non_snake_case)]

use std::{ffi::*, mem, ptr::null};

pub type GLenum = u32;
pub type GLboolean = u8;
pub type GLbitfield = u32;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type GLclampf = f32;

pub const FALSE: GLboolean = 0; // 139
pub const TRUE: GLboolean = 1;  // 140
pub const FLOAT: GLenum = 0x1406;   // 149
pub const TRIANGLE_STRIP: GLenum = 0x0005;  // 161
pub const DEPTH_TEST: GLenum = 0x0B71;  // 267
pub const RGBA: GLenum = 0x1908;    // 469
pub const NEAREST: GLenum = 0x2600; // 644
pub const NO_ERROR: GLenum = 0; // 659
pub const INVALID_ENUM: GLenum = 0x0500;    // 660
pub const INVALID_VALUE: GLenum = 0x0501;   // 661
pub const INVALID_OPERATION: GLenum = 0x0502;   // 662
pub const STACK_OVERFLOW: GLenum = 0x0503;  // 663
pub const STACK_UNDERFLOW: GLenum = 0x0504; // 664
pub const OUT_OF_MEMORY: GLenum = 0x0505;   // 665
pub const DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;    // 676
pub const STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;  // 678
pub const COLOR_BUFFER_BIT: GLbitfield = 0x00004000;    // 682

type PFNGLCLEARCOLORPROC = unsafe extern "system" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);   // 745
type PFNGLCLEARPROC = unsafe extern "system" fn(mask: GLbitfield);   // 747
type PFNGLENABLEPROC = unsafe extern "system" fn(cap: GLenum);   // 791
type PFNGLGETERRORPROC = unsafe extern "system" fn() -> GLenum;  // 824
type PFNGLVIEWPORTPROC = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei); // 871
type PFNGLDRAWARRAYSPROC = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);    // 1140

// glext.h

pub type GLsizeiptr = isize; // 450
pub type GLchar = c_char;   // 546

pub const ARRAY_BUFFER: GLenum = 0x8892;    // 458
pub const STATIC_DRAW: GLenum = 0x88E4;     // 472
pub const FRAGMENT_SHADER: GLenum = 0x8B30; // 580
pub const VERTEX_SHADER: GLenum = 0x8B31;   // 581
pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A; // 979
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;   // 968
pub const DEPTH24_STENCIL8: GLenum = 0x88F0;    // 983
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;    // 994
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;    // 995
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;    // 1003
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;   // 1010
pub const FRAMEBUFFER: GLenum = 0x8D40;     // 1044
pub const RENDERBUFFER: GLenum = 0x8D41;    // 1045

type PFNGLBINDBUFFERPROC = unsafe extern "system" fn(target: GLenum, buffer: GLuint);    // 510
type PFNGLDELETEBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint); // 511
type PFNGLGENBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);  // 512
type PFNGLBUFFERDATAPROC = unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);  // 514
type PFNGLATTACHSHADERPROC = unsafe extern "system" fn(program: GLuint, shader: GLuint); // 636
type PFNGLCOMPILESHADERPROC = unsafe extern "system" fn(shader: GLuint); // 638
type PFNGLCREATEPROGRAMPROC = unsafe extern "system" fn() -> GLuint; // 639
type PFNGLCREATESHADERPROC = unsafe extern "system" fn(type_: GLenum) -> GLuint; // 640
type PFNGLDELETEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);    // 641
type PFNGLDELETESHADERPROC = unsafe extern "system" fn(shader: GLuint);  // 642
type PFNGLDISABLEVERTEXATTRIBARRAYPROC = unsafe extern "system" fn(index: GLuint);   // 644
type PFNGLENABLEVERTEXATTRIBARRAYPROC = unsafe extern "system" fn(index: GLuint);    // 645
type PFNGLGETATTRIBLOCATIONPROC = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;  // 649
type PFNGLGETUNIFORMLOCATIONPROC = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint; // 655
type PFNGLLINKPROGRAMPROC = unsafe extern "system" fn(program: GLuint);  // 664
type PFNGLSHADERSOURCEPROC = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);  // 665
type PFNGLUSEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);   // 666
type PFNGLUNIFORMMATRIX4FVPROC = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);    // 685
type PFNGLVERTEXATTRIBPOINTERPROC = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);    // 723
type PFNGLBINDRENDERBUFFERPROC = unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint);    // 1161
type PFNGLDELETERENDERBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint); // 1162
type PFNGLGENRENDERBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);  // 1163
type PFNGLBINDFRAMEBUFFERPROC = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);  // 1167
type PFNGLDELETEFRAMEBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);   // 1168
type PFNGLGENFRAMEBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);    // 1169
type PFNGLCHECKFRAMEBUFFERSTATUSPROC = unsafe extern "system" fn(target: GLenum) -> GLenum;  // 1170
type PFNGLFRAMEBUFFERRENDERBUFFERPROC = unsafe extern "system" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint); // 1174
type PFNGLBLITFRAMEBUFFERPROC = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum); // 1177
type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei); // 1178
type PFNGLBINDVERTEXARRAYPROC = unsafe extern "system" fn(array: GLuint);    // 1182
type PFNGLDELETEVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint); // 1183
type PFNGLGENVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);  // 1184

struct FnV2_0 {
    pub clear_color: PFNGLCLEARCOLORPROC,
    pub clear: PFNGLCLEARPROC,
    pub enable: PFNGLENABLEPROC,
    pub get_error: PFNGLGETERRORPROC,
    pub viewport: PFNGLVIEWPORTPROC,
    pub draw_arrays: PFNGLDRAWARRAYSPROC,
    pub bind_buffer: PFNGLBINDBUFFERPROC,
    pub delete_buffers: PFNGLDELETEBUFFERSPROC,
    pub gen_buffers: PFNGLGENBUFFERSPROC,
    pub buffer_data: PFNGLBUFFERDATAPROC,
    pub attach_shader: PFNGLATTACHSHADERPROC,
    pub compile_shader: PFNGLCOMPILESHADERPROC,
    pub create_program: PFNGLCREATEPROGRAMPROC,
    pub create_shader: PFNGLCREATESHADERPROC,
    pub delete_program: PFNGLDELETEPROGRAMPROC,
    pub delete_shader: PFNGLDELETESHADERPROC,
    pub disable_vertex_attrib_array: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
    pub enable_vertex_attrib_array: PFNGLENABLEVERTEXATTRIBARRAYPROC,
    pub get_attrib_location: PFNGLGETATTRIBLOCATIONPROC,
    pub get_uniform_location: PFNGLGETUNIFORMLOCATIONPROC,
    pub link_program: PFNGLLINKPROGRAMPROC,
    pub shader_source: PFNGLSHADERSOURCEPROC,
    pub use_program: PFNGLUSEPROGRAMPROC,
    pub uniform_matrix_4fv: PFNGLUNIFORMMATRIX4FVPROC,
    pub vertex_attrib_pointer: PFNGLVERTEXATTRIBPOINTERPROC,
}

impl FnV2_0 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    
    fn load_erased(f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            clear_color: unsafe {
                unsafe extern "system" fn clear_color(_red: GLclampf, _green: GLclampf, _blue: GLclampf, _alpha: GLclampf) {
                    panic!(concat!("Unable to load ", stringify!(clear_color)))
                }
                let val = f(c"glClearColor");
                if val.is_null() {
                    clear_color
                } else {
                    mem::transmute(val)
                }
            },
            clear: unsafe {
                unsafe extern "system" fn clear(_mask: GLbitfield) {
                    panic!(concat!("Unable to load ", stringify!(clear)))
                }
                let val = f(c"glClear");
                if val.is_null() {
                    clear
                } else {
                    mem::transmute(val)
                }
            },
            enable: unsafe {
                unsafe extern "system" fn enable(_cap: GLenum) {
                    panic!(concat!("Unable to load ", stringify!(enable)))
                }
                let val = f(c"glEnable");
                if val.is_null() {
                    enable
                } else {
                    mem::transmute(val)
                }
            },
            get_error: unsafe {
                unsafe extern "system" fn get_error() -> GLenum {
                    panic!(concat!("Unable to load ", stringify!(get_error)))
                }
                let val = f(c"glGetError");
                if val.is_null() {
                    get_error
                } else {
                    mem::transmute(val)
                }
            },
            viewport: unsafe {
                unsafe extern "system" fn viewport(_x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei) {
                    panic!(concat!("Unable to load ", stringify!(viewport)))
                }
                let val = f(c"glViewport");
                if val.is_null() {
                    viewport
                } else {
                    mem::transmute(val)
                }
            },
            draw_arrays: unsafe {
                unsafe extern "system" fn draw_arrays(_mode: GLenum, _first: GLint, _count: GLsizei) {
                    panic!(concat!("Unable to load ", stringify!(draw_arrays)))
                }
                let val = f(c"glDrawArrays");
                if val.is_null() {
                    draw_arrays
                } else {
                    mem::transmute(val)
                }
            },
            bind_buffer: unsafe {
                unsafe extern "system" fn bind_buffer(_target: GLenum, _buffer: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(bind_buffer)))
                }
                let val = f(c"glBindBuffer");
                if val.is_null() {
                    bind_buffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_buffers: unsafe {
                unsafe extern "system" fn delete_buffers(_n: GLsizei, _buffers: *const GLuint) {
                    panic!(concat!("Unable to load ", stringify!(delete_buffers)))
                }
                let val = f(c"glDeleteBuffers");
                if val.is_null() {
                    delete_buffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_buffers: unsafe {
                unsafe extern "system" fn gen_buffers(_n: GLsizei, _buffers: *mut GLuint) {
                    panic!(concat!("Unable to load ", stringify!(gen_buffers)))
                }
                let val = f(c"glGenBuffers");
                if val.is_null() {
                    gen_buffers
                } else {
                    mem::transmute(val)
                }
            },
            buffer_data: unsafe {
                unsafe extern "system" fn buffer_data(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
                    panic!(concat!("Unable to load ", stringify!(buffer_data)))
                }
                let val = f(c"glBufferData");
                if val.is_null() {
                    buffer_data
                } else {
                    mem::transmute(val)
                }
            },
            attach_shader: unsafe {
                unsafe extern "system" fn attach_shader(program: GLuint, shader: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(attach_shader)))
                }
                let val = f(c"glAttachShader");
                if val.is_null() {
                    attach_shader
                } else {
                    mem::transmute(val)
                }
            },
            compile_shader: unsafe {
                unsafe extern "system" fn compile_shader(shader: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(compile_shader)))
                }
                let val = f(c"glCompileShader");
                if val.is_null() {
                    compile_shader
                } else {
                    mem::transmute(val)
                }
            },
            create_program: unsafe {
                unsafe extern "system" fn create_program() -> GLuint {
                    panic!(concat!("Unable to load ", stringify!(create_program)))
                }
                let val = f(c"glCreateProgram");
                if val.is_null() {
                    create_program
                } else {
                    mem::transmute(val)
                }
            },
            create_shader: unsafe {
                unsafe extern "system" fn create_shader(type_: GLenum) -> GLuint {
                    panic!(concat!("Unable to load ", stringify!(create_shader)))
                }
                let val = f(c"glCreateShader");
                if val.is_null() {
                    create_shader
                } else {
                    mem::transmute(val)
                }
            },
            delete_program: unsafe {
                unsafe extern "system" fn delete_program(program: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(delete_program)))
                }
                let val = f(c"glDeleteProgram");
                if val.is_null() {
                    delete_program
                } else {
                    mem::transmute(val)
                }
            },
            delete_shader: unsafe {
                unsafe extern "system" fn delete_shader(shader: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(delete_shader)))
                }
                let val = f(c"glDeleteShader");
                if val.is_null() {
                    delete_shader
                } else {
                    mem::transmute(val)
                }
            },
            disable_vertex_attrib_array: unsafe {
                unsafe extern "system" fn disable_vertex_attrib_array(index: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(disable_vertex_attrib_array)))
                }
                let val = f(c"glDisableVertexAttribArray");
                if val.is_null() {
                    disable_vertex_attrib_array
                } else {
                    mem::transmute(val)
                }
            },
            enable_vertex_attrib_array: unsafe {
                unsafe extern "system" fn enable_vertex_attrib_array(index: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(enable_vertex_attrib_array)))
                }
                let val = f(c"glEnableVertexAttribArray");
                if val.is_null() {
                    enable_vertex_attrib_array
                } else {
                    mem::transmute(val)
                }
            },
            get_attrib_location: unsafe {
                unsafe extern "system" fn get_attrib_location(program: GLuint, name: *const GLchar) -> GLint {
                    panic!(concat!("Unable to load ", stringify!(get_attrib_location)))
                }
                let val = f(c"glGetAttribLocation");
                if val.is_null() {
                    get_attrib_location
                } else {
                    mem::transmute(val)
                }
            },
            get_uniform_location: unsafe {
                unsafe extern "system" fn get_uniform_location(program: GLuint, name: *const GLchar) -> GLint {
                    panic!(concat!("Unable to load ", stringify!(get_uniform_location)))
                }
                let val = f(c"glGetUniformLocation");
                if val.is_null() {
                    get_uniform_location
                } else {
                    mem::transmute(val)
                }
            },
            link_program: unsafe {
                unsafe extern "system" fn link_program(program: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(link_program)))
                }
                let val = f(c"glLinkProgram");
                if val.is_null() {
                    link_program
                } else {
                    mem::transmute(val)
                }
            },
            shader_source: unsafe {
                unsafe extern "system" fn shader_source(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
                    panic!(concat!("Unable to load ", stringify!(shader_source)))
                }
                let val = f(c"glShaderSource");
                if val.is_null() {
                    shader_source
                } else {
                    mem::transmute(val)
                }
            },
            use_program: unsafe {
                unsafe extern "system" fn use_program(program: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(use_program)))
                }
                let val = f(c"glUseProgram");
                if val.is_null() {
                    use_program
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix_4fv: unsafe {
                unsafe extern "system" fn uniform_matrix_4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
                    panic!(concat!("Unable to load ", stringify!(uniform_matrix_4fv)))
                }
                let val = f(c"glUniformMatrix4fv");
                if val.is_null() {
                    uniform_matrix_4fv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_pointer: unsafe {
                unsafe extern "system" fn vertex_attrib_pointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
                    panic!(concat!("Unable to load ", stringify!(vertex_attrib_pointer)))
                }
                let val = f(c"glVertexAttribPointer");
                if val.is_null() {
                    vertex_attrib_pointer
                } else {
                    mem::transmute(val)
                }
            },
        }
    }
}

struct FnV3_0 {
    pub bind_renderbuffer: PFNGLBINDRENDERBUFFERPROC,
    pub delete_renderbuffers: PFNGLDELETERENDERBUFFERSPROC,
    pub gen_renderbuffers: PFNGLGENRENDERBUFFERSPROC,
    pub bind_framebuffer: PFNGLBINDFRAMEBUFFERPROC,
    pub delete_framebuffers: PFNGLDELETEFRAMEBUFFERSPROC,
    pub gen_framebuffers: PFNGLGENFRAMEBUFFERSPROC,
    pub check_framebuffer_status: PFNGLCHECKFRAMEBUFFERSTATUSPROC,
    pub framebuffer_renderbuffer: PFNGLFRAMEBUFFERRENDERBUFFERPROC,
    pub blit_framebuffer: PFNGLBLITFRAMEBUFFERPROC,
    pub renderbuffer_storage_multisample: PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC,
    pub bind_vertex_array: PFNGLBINDVERTEXARRAYPROC,
    pub delete_vertex_arrays: PFNGLDELETEVERTEXARRAYSPROC,
    pub gen_vertex_arrays: PFNGLGENVERTEXARRAYSPROC,
}

impl FnV3_0 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }

    fn load_erased(f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            bind_renderbuffer: unsafe {
                unsafe extern "system" fn bind_renderbuffer(target: GLenum, renderbuffer: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(bind_renderbuffer)))
                }
                let val = f(c"glBindRenderbuffer");
                if val.is_null() {
                    bind_renderbuffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_renderbuffers: unsafe {
                unsafe extern "system" fn delete_renderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
                    panic!(concat!("Unable to load ", stringify!(delete_renderbuffers)))
                }
                let val = f(c"glDeleteRenderbuffers");
                if val.is_null() {
                    delete_renderbuffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_renderbuffers: unsafe {
                unsafe extern "system" fn gen_renderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
                    panic!(concat!("Unable to load ", stringify!(gen_renderbuffers)))
                }
                let val = f(c"glGenRenderbuffers");
                if val.is_null() {
                    gen_renderbuffers
                } else {
                    mem::transmute(val)
                }
            },
            bind_framebuffer: unsafe {
                unsafe extern "system" fn bind_framebuffer(target: GLenum, framebuffer: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(bind_framebuffer)))
                }
                let val = f(c"glBindFramebuffer");
                if val.is_null() {
                    bind_framebuffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_framebuffers: unsafe {
                unsafe extern "system" fn delete_framebuffers(n: GLsizei, framebuffers: *const GLuint) {
                    panic!(concat!("Unable to load ", stringify!(delete_framebuffers)))
                }
                let val = f(c"glDeleteFramebuffers");
                if val.is_null() {
                    delete_framebuffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_framebuffers: unsafe {
                unsafe extern "system" fn gen_framebuffers(n: GLsizei, framebuffers: *mut GLuint) {
                    panic!(concat!("Unable to load ", stringify!(gen_framebuffers)))
                }
                let val = f(c"glGenFramebuffers");
                if val.is_null() {
                    gen_framebuffers
                } else {
                    mem::transmute(val)
                }
            },
            check_framebuffer_status: unsafe {
                unsafe extern "system" fn check_framebuffer_status(target: GLenum) -> GLenum {
                    panic!(concat!("Unable to load ", stringify!(check_framebuffer_status)))
                }
                let val = f(c"glCheckFramebufferStatus");
                if val.is_null() {
                    check_framebuffer_status
                } else {
                    mem::transmute(val)
                }
            },
            framebuffer_renderbuffer: unsafe {
                unsafe extern "system" fn framebuffer_renderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(framebuffer_renderbuffer)))
                }
                let val = f(c"glFramebufferRenderbuffer");
                if val.is_null() {
                    framebuffer_renderbuffer
                } else {
                    mem::transmute(val)
                }
            },
            blit_framebuffer: unsafe {
                unsafe extern "system" fn blit_framebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
                    panic!(concat!("Unable to load ", stringify!(blit_framebuffer)))
                }
                let val = f(c"glBlitFramebuffer");
                if val.is_null() {
                    blit_framebuffer
                } else {
                    mem::transmute(val)
                }
            },
            renderbuffer_storage_multisample: unsafe {
                unsafe extern "system" fn renderbuffer_storage_multisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
                    panic!(concat!("Unable to load ", stringify!(renderbuffer_storage_multisample)))
                }
                let val = f(c"glRenderbufferStorageMultisample");
                if val.is_null() {
                    renderbuffer_storage_multisample
                } else {
                    mem::transmute(val)
                }
            },
            bind_vertex_array: unsafe {
                unsafe extern "system" fn bind_vertex_array(array: GLuint) {
                    panic!(concat!("Unable to load ", stringify!(bind_vertex_array)))
                }
                let val = f(c"glBindVertexArray");
                if val.is_null() {
                    bind_vertex_array
                } else {
                    mem::transmute(val)
                }
            },
            delete_vertex_arrays: unsafe {
                unsafe extern "system" fn delete_vertex_arrays(n: GLsizei, arrays: *const GLuint) {
                    panic!(concat!("Unable to load ", stringify!(delete_vertex_arrays)))
                }
                let val = f(c"glDeleteVertexArrays");
                if val.is_null() {
                    delete_vertex_arrays
                } else {
                    mem::transmute(val)
                }
            },
            gen_vertex_arrays: unsafe {
                unsafe extern "system" fn gen_vertex_arrays(n: GLsizei, arrays: *mut GLuint) {
                    panic!(concat!("Unable to load ", stringify!(gen_vertex_arrays)))
                }
                let val = f(c"glGenVertexArrays");
                if val.is_null() {
                    gen_vertex_arrays
                } else {
                    mem::transmute(val)
                }
            },
        }
    }
}

pub struct Context {
    fn_2_0: FnV2_0,
    fn_3_0: FnV3_0,
}

impl Context {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self {
            fn_2_0: FnV2_0::load(&mut f),
            fn_3_0: FnV3_0::load(&mut f),
        }
    }

    unsafe fn check_error(&self) {
        #[cfg(debug_assertions)]
        {
            let error_code = self.get_error();
            if error_code != NO_ERROR {
                let error_str = match error_code {
                    INVALID_ENUM => "GL_INVALID_ENUM",
                    INVALID_VALUE => "GL_INVALID_VALUE",
                    INVALID_OPERATION => "GL_INVALID_OPERATION",
                    INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                    OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
                    STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
                    STACK_OVERFLOW => "GL_STACK_OVERFLOW",
                    _ => unreachable!(),
                };
                let error_desc = match error_code {
                    INVALID_ENUM => "An unacceptable value is specified for an enumerated argument.",
                    INVALID_VALUE => "A numeric argument is out of range.",
                    INVALID_OPERATION => "The specified operation is not allowed in the current state.",
                    INVALID_FRAMEBUFFER_OPERATION => "The framebuffer object is not complete.",
                    OUT_OF_MEMORY => "There is not enough memory left to execute the command.",
                    STACK_UNDERFLOW => "An attempt has been made to perform an operation that would cause an internal stack to underflow.",
                    STACK_OVERFLOW => "An attempt has been made to perform an operation that would cause an internal stack to overflow.",
                    _ => unreachable!(),
                };
                panic!("{}: {}", error_str, error_desc);
            }
        }
    }

    #[inline]
    pub unsafe fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        (self.fn_2_0.clear_color)(red, green, blue, alpha);
        self.check_error();
    }

    #[inline]
    pub unsafe fn clear(&self, mask: GLbitfield) {
        (self.fn_2_0.clear)(mask);
        self.check_error();
    }

    #[inline]
    pub unsafe fn enable(&self, cap: GLenum) {
        (self.fn_2_0.enable)(cap);
        self.check_error();
    }

    #[inline]
    pub unsafe fn get_error(&self) -> GLenum {
        (self.fn_2_0.get_error)()
    }

    #[inline]
    pub unsafe fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        (self.fn_2_0.viewport)(x, y, width, height);
        self.check_error();
    }

    #[inline]
    pub unsafe fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        (self.fn_2_0.draw_arrays)(mode, first, count);
        self.check_error();
    }

    #[inline]
    pub unsafe fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
        (self.fn_2_0.bind_buffer)(target, buffer);
        self.check_error();
    }

    #[inline]
    pub unsafe fn delete_buffers(&self, buffers: &[GLuint]) {
        (self.fn_2_0.delete_buffers)(buffers.len() as _, buffers.as_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn gen_buffers(&self, buffers: &mut [GLuint]) {
        (self.fn_2_0.gen_buffers)(buffers.len() as _, buffers.as_mut_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn buffer_data<T>(&self, target: GLenum, data: &[T], usage: GLenum) {
        (self.fn_2_0.buffer_data)(target, (data.len() * mem::size_of::<T>()) as _, data.as_ptr() as _, usage);
        self.check_error();
    }

    #[inline]
    pub unsafe fn attach_shader(&self, program: GLuint, shader: GLuint) {
        (self.fn_2_0.attach_shader)(program, shader);
        self.check_error();
    }

    #[inline]
    pub unsafe fn compile_shader(&self, shader: GLuint) {
        (self.fn_2_0.compile_shader)(shader);
        self.check_error();
    }

    #[inline]
    pub unsafe fn create_program(&self) -> GLuint {
        let result = (self.fn_2_0.create_program)();
        self.check_error();
        result
    }

    #[inline]
    pub unsafe fn create_shader(&self, type_: GLenum) -> GLuint {
        let result = (self.fn_2_0.create_shader)(type_);
        self.check_error();
        result
    }

    #[inline]
    pub unsafe fn delete_program(&self, program: GLuint) {
        (self.fn_2_0.delete_program)(program);
        self.check_error();
    }

    #[inline]
    pub unsafe fn delete_shader(&self, shader: GLuint) {
        (self.fn_2_0.delete_shader)(shader);
        self.check_error();
    }

    #[inline]
    pub unsafe fn disable_vertex_attrib_array(&self, index: GLuint) {
        (self.fn_2_0.disable_vertex_attrib_array)(index);
        self.check_error();
    }

    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, index: GLuint) {
        (self.fn_2_0.enable_vertex_attrib_array)(index);
        self.check_error();
    }

    #[inline]
    pub unsafe fn get_attrib_location(&self, program: GLuint, name: &CStr) -> GLint {
        let result = (self.fn_2_0.get_attrib_location)(program, name.as_ptr());
        self.check_error();
        result
    }

    #[inline]
    pub unsafe fn get_uniform_location(&self, program: GLuint, name: &CStr) -> GLint {
        let result = (self.fn_2_0.get_uniform_location)(program, name.as_ptr());
        self.check_error();
        result
    }

    #[inline]
    pub unsafe fn link_program(&self, program: GLuint) {
        (self.fn_2_0.link_program)(program);
        self.check_error();
    }

    #[inline]
    pub unsafe fn shader_source(&self, shader: GLuint, string: &[*const GLchar], length: Option<&[GLint]>) {
        #[cfg(debug_assertions)]
        if let Some(length) = length {
            assert_eq!(string.len(), length.len());
        }
        (self.fn_2_0.shader_source)(shader, string.len() as _, string.as_ptr(), if let Some(length) = length { length.as_ptr() } else { null() });
        self.check_error();
    }

    #[inline]
    pub unsafe fn use_program(&self, program: GLuint) {
        (self.fn_2_0.use_program)(program);
        self.check_error();
    }

    #[inline]
    pub unsafe fn uniform_matrix_4fv(&self, location: GLint, count: GLsizei, transpose: bool, value: &[GLfloat]) {
        (self.fn_2_0.uniform_matrix_4fv)(location, count, if transpose { TRUE } else { FALSE }, value.as_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn vertex_attrib_pointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: bool, stride: GLsizei, pointer: usize) {
        (self.fn_2_0.vertex_attrib_pointer)(index, size, type_, if normalized { TRUE } else { FALSE }, stride, pointer as _);
        self.check_error();
    }

    #[inline]
    pub unsafe fn bind_renderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
        (self.fn_3_0.bind_renderbuffer)(target, renderbuffer);
        self.check_error();
    }

    #[inline]
    pub unsafe fn delete_renderbuffers(&self, renderbuffers: &[GLuint]) {
        (self.fn_3_0.delete_renderbuffers)(renderbuffers.len() as _, renderbuffers.as_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn gen_renderbuffers(&self, renderbuffers: &mut [GLuint]) {
        (self.fn_3_0.gen_renderbuffers)(renderbuffers.len() as _, renderbuffers.as_mut_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {
        (self.fn_3_0.bind_framebuffer)(target, framebuffer);
        self.check_error();
    }

    #[inline]
    pub unsafe fn delete_framebuffers(&self, framebuffers: &[GLuint]) {
        (self.fn_3_0.delete_framebuffers)(framebuffers.len() as _, framebuffers.as_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn gen_framebuffers(&self, framebuffers: &mut [GLuint]) {
        (self.fn_3_0.gen_framebuffers)(framebuffers.len() as _, framebuffers.as_mut_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        let result = (self.fn_3_0.check_framebuffer_status)(target);
        self.check_error();
        result
    }

    #[inline]
    pub unsafe fn framebuffer_renderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
        (self.fn_3_0.framebuffer_renderbuffer)(target, attachment, renderbuffertarget, renderbuffer);
        self.check_error();
    }

    #[inline]
    pub unsafe fn blit_framebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
        (self.fn_3_0.blit_framebuffer)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
        self.check_error();
    }

    #[inline]
    pub unsafe fn renderbuffer_storage_multisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        (self.fn_3_0.renderbuffer_storage_multisample)(target, samples, internalformat, width, height);
        self.check_error();
    }

    #[inline]
    pub unsafe fn bind_vertex_array(&self, array: GLuint) {
        (self.fn_3_0.bind_vertex_array)(array);
        self.check_error();
    }

    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, arrays: &[GLuint]) {
        (self.fn_3_0.delete_vertex_arrays)(arrays.len() as _, arrays.as_ptr());
        self.check_error();
    }

    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, arrays: &mut [GLuint]) {
        (self.fn_3_0.gen_vertex_arrays)(arrays.len() as _, arrays.as_mut_ptr());
        self.check_error();
    }
}