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

type PFNGLCLEARCOLORPROC = unsafe extern "C" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);   // 745
type PFNGLCLEARPROC = unsafe extern "C" fn(mask: GLbitfield);   // 747
type PFNGLENABLEPROC = unsafe extern "C" fn(cap: GLenum);   // 791
type PFNGLGETERRORPROC = unsafe extern "C" fn() -> GLenum;  // 824
type PFNGLVIEWPORTPROC = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei); // 871
type PFNGLDRAWARRAYSPROC = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei);    // 1140

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

type PFNGLBINDBUFFERPROC = unsafe extern "C" fn(target: GLenum, buffer: GLuint);    // 510
type PFNGLDELETEBUFFERSPROC = unsafe extern "C" fn(n: GLsizei, buffers: *const GLuint); // 511
type PFNGLGENBUFFERSPROC = unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint);  // 512
type PFNGLBUFFERDATAPROC = unsafe extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);  // 514
type PFNGLATTACHSHADERPROC = unsafe extern "C" fn(program: GLuint, shader: GLuint); // 636
type PFNGLCOMPILESHADERPROC = unsafe extern "C" fn(shader: GLuint); // 638
type PFNGLCREATEPROGRAMPROC = unsafe extern "C" fn() -> GLuint; // 639
type PFNGLCREATESHADERPROC = unsafe extern "C" fn(type_: GLenum) -> GLuint; // 640
type PFNGLDELETEPROGRAMPROC = unsafe extern "C" fn(program: GLuint);    // 641
type PFNGLDELETESHADERPROC = unsafe extern "C" fn(shader: GLuint);  // 642
type PFNGLDISABLEVERTEXATTRIBARRAYPROC = unsafe extern "C" fn(index: GLuint);   // 644
type PFNGLENABLEVERTEXATTRIBARRAYPROC = unsafe extern "C" fn(index: GLuint);    // 645
type PFNGLGETATTRIBLOCATIONPROC = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint;  // 649
type PFNGLGETUNIFORMLOCATIONPROC = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint; // 655
type PFNGLLINKPROGRAMPROC = unsafe extern "C" fn(program: GLuint);  // 664
type PFNGLSHADERSOURCEPROC = unsafe extern "C" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);  // 665
type PFNGLUSEPROGRAMPROC = unsafe extern "C" fn(program: GLuint);   // 666
type PFNGLUNIFORMMATRIX4FVPROC = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);    // 685
type PFNGLVERTEXATTRIBPOINTERPROC = unsafe extern "C" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);    // 723
type PFNGLBINDRENDERBUFFERPROC = unsafe extern "C" fn(target: GLenum, renderbuffer: GLuint);    // 1161
type PFNGLDELETERENDERBUFFERSPROC = unsafe extern "C" fn(n: GLsizei, renderbuffers: *const GLuint); // 1162
type PFNGLGENRENDERBUFFERSPROC = unsafe extern "C" fn(n: GLsizei, renderbuffers: *mut GLuint);  // 1163
type PFNGLBINDFRAMEBUFFERPROC = unsafe extern "C" fn(target: GLenum, framebuffer: GLuint);  // 1167
type PFNGLDELETEFRAMEBUFFERSPROC = unsafe extern "C" fn(n: GLsizei, framebuffers: *const GLuint);   // 1168
type PFNGLGENFRAMEBUFFERSPROC = unsafe extern "C" fn(n: GLsizei, framebuffers: *mut GLuint);    // 1169
type PFNGLCHECKFRAMEBUFFERSTATUSPROC = unsafe extern "C" fn(target: GLenum) -> GLenum;  // 1170
type PFNGLFRAMEBUFFERRENDERBUFFERPROC = unsafe extern "C" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint); // 1174
type PFNGLBLITFRAMEBUFFERPROC = unsafe extern "C" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum); // 1177
type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei); // 1178
type PFNGLBINDVERTEXARRAYPROC = unsafe extern "C" fn(array: GLuint);    // 1182
type PFNGLDELETEVERTEXARRAYSPROC = unsafe extern "C" fn(n: GLsizei, arrays: *const GLuint); // 1183
type PFNGLGENVERTEXARRAYSPROC = unsafe extern "C" fn(n: GLsizei, arrays: *mut GLuint);  // 1184

pub struct Context {
    glClearColor: PFNGLCLEARCOLORPROC,
    glClear: PFNGLCLEARPROC,
    glEnable: PFNGLENABLEPROC,
    glGetError: PFNGLGETERRORPROC,
    glViewport: PFNGLVIEWPORTPROC,
    glDrawArrays: PFNGLDRAWARRAYSPROC,
    glBindBuffer: PFNGLBINDBUFFERPROC,
    glDeleteBuffers: PFNGLDELETEBUFFERSPROC,
    glGenBuffers: PFNGLGENBUFFERSPROC,
    glBufferData: PFNGLBUFFERDATAPROC,
    glAttachShader: PFNGLATTACHSHADERPROC,
    glCompileShader: PFNGLCOMPILESHADERPROC,
    glCreateProgram: PFNGLCREATEPROGRAMPROC,
    glCreateShader: PFNGLCREATESHADERPROC,
    glDeleteProgram: PFNGLDELETEPROGRAMPROC,
    glDeleteShader: PFNGLDELETESHADERPROC,
    glDisableVertexAttribArray: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
    glEnableVertexAttribArray: PFNGLENABLEVERTEXATTRIBARRAYPROC,
    glGetAttribLocation: PFNGLGETATTRIBLOCATIONPROC,
    glGetUniformLocation: PFNGLGETUNIFORMLOCATIONPROC,
    glLinkProgram: PFNGLLINKPROGRAMPROC,
    glShaderSource: PFNGLSHADERSOURCEPROC,
    glUseProgram: PFNGLUSEPROGRAMPROC,
    glUniformMatrix4fv: PFNGLUNIFORMMATRIX4FVPROC,
    glVertexAttribPointer: PFNGLVERTEXATTRIBPOINTERPROC,
    glBindRenderbuffer: PFNGLBINDRENDERBUFFERPROC,
    glDeleteRenderbuffers: PFNGLDELETERENDERBUFFERSPROC,
    glGenRenderbuffers: PFNGLGENRENDERBUFFERSPROC,
    glBindFramebuffer: PFNGLBINDFRAMEBUFFERPROC,
    glDeleteFramebuffers: PFNGLDELETEFRAMEBUFFERSPROC,
    glGenFramebuffers: PFNGLGENFRAMEBUFFERSPROC,
    glCheckFramebufferStatus: PFNGLCHECKFRAMEBUFFERSTATUSPROC,
    glFramebufferRenderbuffer: PFNGLFRAMEBUFFERRENDERBUFFERPROC,
    glBlitFramebuffer: PFNGLBLITFRAMEBUFFERPROC,
    glRenderbufferStorageMultisample: PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC,
    glBindVertexArray: PFNGLBINDVERTEXARRAYPROC,
    glDeleteVertexArrays: PFNGLDELETEVERTEXARRAYSPROC,
    glGenVertexArrays: PFNGLGENVERTEXARRAYSPROC,
}

impl Context {
    pub fn load<F>(mut get_proc_address: F) -> Option<Self>
        where F: FnMut(&str) -> Option<unsafe extern "C" fn()> {
        let glClearColor = unsafe { mem::transmute(get_proc_address("glClearColor")?) };
        let glClear = unsafe { mem::transmute(get_proc_address("glClear")?) };
        let glEnable = unsafe { mem::transmute(get_proc_address("glEnable")?) };
        let glGetError = unsafe { mem::transmute(get_proc_address("glGetError")?) };
        let glViewport = unsafe { mem::transmute(get_proc_address("glViewport")?) };
        let glDrawArrays = unsafe { mem::transmute(get_proc_address("glDrawArrays")?) };
        let glBindBuffer = unsafe { mem::transmute(get_proc_address("glBindBuffer")?) };
        let glDeleteBuffers = unsafe { mem::transmute(get_proc_address("glDeleteBuffers")?) };
        let glGenBuffers = unsafe { mem::transmute(get_proc_address("glGenBuffers")?) };
        let glBufferData = unsafe { mem::transmute(get_proc_address("glBufferData")?) };
        let glAttachShader = unsafe { mem::transmute(get_proc_address("glAttachShader")?) };
        let glCompileShader = unsafe { mem::transmute(get_proc_address("glCompileShader")?) };
        let glCreateProgram = unsafe { mem::transmute(get_proc_address("glCreateProgram")?) };
        let glCreateShader = unsafe { mem::transmute(get_proc_address("glCreateShader")?) };
        let glDeleteProgram = unsafe { mem::transmute(get_proc_address("glDeleteProgram")?) };
        let glDeleteShader = unsafe { mem::transmute(get_proc_address("glDeleteShader")?) };
        let glDisableVertexAttribArray = unsafe { mem::transmute(get_proc_address("glDisableVertexAttribArray")?) };
        let glEnableVertexAttribArray = unsafe { mem::transmute(get_proc_address("glEnableVertexAttribArray")?) };
        let glGetAttribLocation = unsafe { mem::transmute(get_proc_address("glGetAttribLocation")?) };
        let glGetUniformLocation = unsafe { mem::transmute(get_proc_address("glGetUniformLocation")?) };
        let glLinkProgram = unsafe { mem::transmute(get_proc_address("glLinkProgram")?) };
        let glShaderSource = unsafe { mem::transmute(get_proc_address("glShaderSource")?) };
        let glUseProgram = unsafe { mem::transmute(get_proc_address("glUseProgram")?) };
        let glUniformMatrix4fv = unsafe { mem::transmute(get_proc_address("glUniformMatrix4fv")?) };
        let glVertexAttribPointer = unsafe { mem::transmute(get_proc_address("glVertexAttribPointer")?) };
        let glBindRenderbuffer = unsafe { mem::transmute(get_proc_address("glBindRenderbuffer")?) };
        let glDeleteRenderbuffers = unsafe { mem::transmute(get_proc_address("glDeleteRenderbuffers")?) };
        let glGenRenderbuffers = unsafe { mem::transmute(get_proc_address("glGenRenderbuffers")?) };
        let glBindFramebuffer = unsafe { mem::transmute(get_proc_address("glBindFramebuffer")?) };
        let glDeleteFramebuffers = unsafe { mem::transmute(get_proc_address("glDeleteFramebuffers")?) };
        let glGenFramebuffers = unsafe { mem::transmute(get_proc_address("glGenFramebuffers")?) };
        let glCheckFramebufferStatus = unsafe { mem::transmute(get_proc_address("glCheckFramebufferStatus")?) };
        let glFramebufferRenderbuffer = unsafe { mem::transmute(get_proc_address("glFramebufferRenderbuffer")?) };
        let glBlitFramebuffer = unsafe { mem::transmute(get_proc_address("glBlitFramebuffer")?) };
        let glRenderbufferStorageMultisample = unsafe { mem::transmute(get_proc_address("glRenderbufferStorageMultisample")?) };
        let glBindVertexArray = unsafe { mem::transmute(get_proc_address("glBindVertexArray")?) };
        let glDeleteVertexArrays = unsafe { mem::transmute(get_proc_address("glDeleteVertexArrays")?) };
        let glGenVertexArrays = unsafe { mem::transmute(get_proc_address("glGenVertexArrays")?) };
        Some(Self {
            glClearColor,
            glClear,
            glEnable,
            glGetError,
            glViewport,
            glDrawArrays,
            glBindBuffer,
            glDeleteBuffers,
            glGenBuffers,
            glBufferData,
            glAttachShader,
            glCompileShader,
            glCreateProgram,
            glCreateShader,
            glDeleteProgram,
            glDeleteShader,
            glDisableVertexAttribArray,
            glEnableVertexAttribArray,
            glGetAttribLocation,
            glGetUniformLocation,
            glLinkProgram,
            glShaderSource,
            glUseProgram,
            glUniformMatrix4fv,
            glVertexAttribPointer,
            glBindRenderbuffer,
            glDeleteRenderbuffers,
            glGenRenderbuffers,
            glBindFramebuffer,
            glDeleteFramebuffers,
            glGenFramebuffers,
            glCheckFramebufferStatus,
            glFramebufferRenderbuffer,
            glBlitFramebuffer,
            glRenderbufferStorageMultisample,
            glBindVertexArray,
            glDeleteVertexArrays,
            glGenVertexArrays,
        })
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

    pub unsafe fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        (self.glClearColor)(red, green, blue, alpha);
        self.check_error();
    }

    pub unsafe fn clear(&self, mask: GLbitfield) {
        (self.glClear)(mask);
        self.check_error();
    }

    pub unsafe fn enable(&self, cap: GLenum) {
        (self.glEnable)(cap);
        self.check_error();
    }

    pub unsafe fn get_error(&self) -> GLenum {
        (self.glGetError)()
    }

    pub unsafe fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        (self.glViewport)(x, y, width, height);
        self.check_error();
    }

    pub unsafe fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        (self.glDrawArrays)(mode, first, count);
        self.check_error();
    }

    pub unsafe fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
        (self.glBindBuffer)(target, buffer);
        self.check_error();
    }

    pub unsafe fn delete_buffers(&self, buffers: &[GLuint]) {
        (self.glDeleteBuffers)(buffers.len() as _, buffers.as_ptr());
        self.check_error();
    }

    pub unsafe fn gen_buffers(&self, buffers: &mut [GLuint]) {
        (self.glGenBuffers)(buffers.len() as _, buffers.as_mut_ptr());
        self.check_error();
    }

    pub unsafe fn buffer_data<T>(&self, target: GLenum, data: &[T], usage: GLenum) {
        (self.glBufferData)(target, (data.len() * mem::size_of::<T>()) as _, data.as_ptr() as _, usage);
        self.check_error();
    }

    pub unsafe fn attach_shader(&self, program: GLuint, shader: GLuint) {
        (self.glAttachShader)(program, shader);
        self.check_error();
    }

    pub unsafe fn compile_shader(&self, shader: GLuint) {
        (self.glCompileShader)(shader);
        self.check_error();
    }

    pub unsafe fn create_program(&self) -> GLuint {
        let result = (self.glCreateProgram)();
        self.check_error();
        result
    }

    pub unsafe fn create_shader(&self, type_: GLenum) -> GLuint {
        let result = (self.glCreateShader)(type_);
        self.check_error();
        result
    }

    pub unsafe fn delete_program(&self, program: GLuint) {
        (self.glDeleteProgram)(program);
        self.check_error();
    }

    pub unsafe fn delete_shader(&self, shader: GLuint) {
        (self.glDeleteShader)(shader);
        self.check_error();
    }

    pub unsafe fn disable_vertex_attrib_array(&self, index: GLuint) {
        (self.glDisableVertexAttribArray)(index);
        self.check_error();
    }

    pub unsafe fn enable_vertex_attrib_array(&self, index: GLuint) {
        (self.glEnableVertexAttribArray)(index);
        self.check_error();
    }

    pub unsafe fn get_attrib_location(&self, program: GLuint, name: &CStr) -> GLint {
        let result = (self.glGetAttribLocation)(program, name.as_ptr());
        self.check_error();
        result
    }

    pub unsafe fn get_uniform_location(&self, program: GLuint, name: &CStr) -> GLint {
        let result = (self.glGetUniformLocation)(program, name.as_ptr());
        self.check_error();
        result
    }

    pub unsafe fn link_program(&self, program: GLuint) {
        (self.glLinkProgram)(program);
        self.check_error();
    }

    pub unsafe fn shader_source(&self, shader: GLuint, string: &[*const GLchar], length: Option<&[GLint]>) {
        #[cfg(debug_assertions)]
        if let Some(length) = length {
            assert_eq!(string.len(), length.len());
        }
        (self.glShaderSource)(shader, string.len() as _, string.as_ptr(), if let Some(length) = length { length.as_ptr() } else { null() });
        self.check_error();
    }

    pub unsafe fn use_program(&self, program: GLuint) {
        (self.glUseProgram)(program);
        self.check_error();
    }

    pub unsafe fn uniform_matrix_4fv(&self, location: GLint, count: GLsizei, transpose: bool, value: &[GLfloat]) {
        (self.glUniformMatrix4fv)(location, count, if transpose { TRUE } else { FALSE }, value.as_ptr());
        self.check_error();
    }

    pub unsafe fn vertex_attrib_pointer(&self, index: GLuint, size: usize, type_: GLenum, normalized: bool, stride: usize, pointer: usize) {
        (self.glVertexAttribPointer)(index, size as _, type_, if normalized { TRUE } else { FALSE }, stride as _, pointer as _);
        self.check_error();
    }

    pub unsafe fn bind_renderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
        (self.glBindRenderbuffer)(target, renderbuffer);
        self.check_error();
    }

    pub unsafe fn delete_renderbuffers(&self, renderbuffers: &[GLuint]) {
        (self.glDeleteRenderbuffers)(renderbuffers.len() as _, renderbuffers.as_ptr());
        self.check_error();
    }

    pub unsafe fn gen_renderbuffers(&self, renderbuffers: &mut [GLuint]) {
        (self.glGenRenderbuffers)(renderbuffers.len() as _, renderbuffers.as_mut_ptr());
        self.check_error();
    }

    pub unsafe fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {
        (self.glBindFramebuffer)(target, framebuffer);
        self.check_error();
    }

    pub unsafe fn delete_framebuffers(&self, framebuffers: &[GLuint]) {
        (self.glDeleteFramebuffers)(framebuffers.len() as _, framebuffers.as_ptr());
        self.check_error();
    }

    pub unsafe fn gen_framebuffers(&self, framebuffers: &mut [GLuint]) {
        (self.glGenFramebuffers)(framebuffers.len() as _, framebuffers.as_mut_ptr());
        self.check_error();
    }

    pub unsafe fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        let result = (self.glCheckFramebufferStatus)(target);
        self.check_error();
        result
    }

    pub unsafe fn framebuffer_renderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
        (self.glFramebufferRenderbuffer)(target, attachment, renderbuffertarget, renderbuffer);
        self.check_error();
    }

    pub unsafe fn blit_framebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
        (self.glBlitFramebuffer)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
        self.check_error();
    }

    pub unsafe fn renderbuffer_storage_multisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        (self.glRenderbufferStorageMultisample)(target, samples, internalformat, width, height);
        self.check_error();
    }

    pub unsafe fn bind_vertex_array(&self, array: GLuint) {
        (self.glBindVertexArray)(array);
        self.check_error();
    }

    pub unsafe fn delete_vertex_arrays(&self, arrays: &[GLuint]) {
        (self.glDeleteVertexArrays)(arrays.len() as _, arrays.as_ptr());
        self.check_error();
    }

    pub unsafe fn gen_vertex_arrays(&self, arrays: &mut [GLuint]) {
        (self.glGenVertexArrays)(arrays.len() as _, arrays.as_mut_ptr());
        self.check_error();
    }
}