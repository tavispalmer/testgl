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
pub const DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;    // 676
pub const STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;  // 678
pub const COLOR_BUFFER_BIT: GLbitfield = 0x00004000;    // 682

type PFNGLCLEARCOLORPROC = Option<unsafe extern "C" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf)>;   // 745
type PFNGLCLEARPROC = Option<unsafe extern "C" fn(mask: GLbitfield)>;   // 747
type PFNGLENABLEPROC = Option<unsafe extern "C" fn(cap: GLenum)>;   // 791
type PFNGLVIEWPORTPROC = Option<unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>; // 871
type PFNGLDRAWARRAYSPROC = Option<unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei)>;    // 1140

// glext.h

pub type GLsizeiptr = isize; // 450
pub type GLchar = c_char;   // 546

pub const ARRAY_BUFFER: GLenum = 0x8892;    // 458
pub const STATIC_DRAW: GLenum = 0x88E4;     // 472
pub const FRAGMENT_SHADER: GLenum = 0x8B30; // 580
pub const VERTEX_SHADER: GLenum = 0x8B31;   // 581
pub const FRAMEBUFFER: GLenum = 0x8D40;     // 1044

type PFNGLBINDBUFFERPROC = Option<unsafe extern "C" fn(target: GLenum, buffer: GLuint)>;    // 510
type PFNGLDELETEBUFFERSPROC = Option<unsafe extern "C" fn(n: GLsizei, buffers: *const GLuint)>; // 511
type PFNGLGENBUFFERSPROC = Option<unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint)>;  // 512
type PFNGLBUFFERDATAPROC = Option<unsafe extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum)>;  // 514
type PFNGLATTACHSHADERPROC = Option<unsafe extern "C" fn(program: GLuint, shader: GLuint)>; // 636
type PFNGLCOMPILESHADERPROC = Option<unsafe extern "C" fn(shader: GLuint)>; // 638
type PFNGLCREATEPROGRAMPROC = Option<unsafe extern "C" fn() -> GLuint>; // 639
type PFNGLCREATESHADERPROC = Option<unsafe extern "C" fn(type_: GLenum) -> GLuint>; // 640
type PFNGLDELETEPROGRAMPROC = Option<unsafe extern "C" fn(program: GLuint)>;    // 641
type PFNGLDELETESHADERPROC = Option<unsafe extern "C" fn(shader: GLuint)>;  // 642
type PFNGLDISABLEVERTEXATTRIBARRAYPROC = Option<unsafe extern "C" fn(index: GLuint)>;   // 644
type PFNGLENABLEVERTEXATTRIBARRAYPROC = Option<unsafe extern "C" fn(index: GLuint)>;    // 645
type PFNGLGETATTRIBLOCATIONPROC = Option<unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint>;  // 649
type PFNGLGETUNIFORMLOCATIONPROC = Option<unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint>; // 655
type PFNGLLINKPROGRAMPROC = Option<unsafe extern "C" fn(program: GLuint)>;  // 664
type PFNGLSHADERSOURCEPROC = Option<unsafe extern "C" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint)>;  // 665
type PFNGLUSEPROGRAMPROC = Option<unsafe extern "C" fn(program: GLuint)>;   // 666
type PFNGLUNIFORMMATRIX4FVPROC = Option<unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>;    // 685
type PFNGLVERTEXATTRIBPOINTERPROC = Option<unsafe extern "C" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void)>;    // 723
type PFNGLBINDFRAMEBUFFERPROC = Option<unsafe extern "C" fn(target: GLenum, framebuffer: GLuint)>;  // 1167
type PFNGLBINDVERTEXARRAYPROC = Option<unsafe extern "C" fn(array: GLuint)>;    // 1182
type PFNGLDELETEVERTEXARRAYSPROC = Option<unsafe extern "C" fn(n: GLsizei, arrays: *const GLuint)>; // 1183
type PFNGLGENVERTEXARRAYSPROC = Option<unsafe extern "C" fn(n: GLsizei, arrays: *mut GLuint)>;  // 1184

pub struct Context {
    glClearColor: PFNGLCLEARCOLORPROC,
    glClear: PFNGLCLEARPROC,
    glEnable: PFNGLENABLEPROC,
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
    glBindFramebuffer: PFNGLBINDFRAMEBUFFERPROC,
    glBindVertexArray: PFNGLBINDVERTEXARRAYPROC,
    glDeleteVertexArrays: PFNGLDELETEVERTEXARRAYSPROC,
    glGenVertexArrays: PFNGLGENVERTEXARRAYSPROC,
}

impl Context {
    pub fn load<F>(get_proc_address: F) -> Self
        where F: Fn(&CStr) -> Option<unsafe extern "C" fn()> {
        let glClearColor = unsafe { mem::transmute(get_proc_address(c"glClearColor")) };
        let glClear = unsafe { mem::transmute(get_proc_address(c"glClear")) };
        let glEnable = unsafe { mem::transmute(get_proc_address(c"glEnable")) };
        let glViewport = unsafe { mem::transmute(get_proc_address(c"glViewport")) };
        let glDrawArrays = unsafe { mem::transmute(get_proc_address(c"glDrawArrays")) };
        let glBindBuffer = unsafe { mem::transmute(get_proc_address(c"glBindBuffer")) };
        let glDeleteBuffers = unsafe { mem::transmute(get_proc_address(c"glDeleteBuffers")) };
        let glGenBuffers = unsafe { mem::transmute(get_proc_address(c"glGenBuffers")) };
        let glBufferData = unsafe { mem::transmute(get_proc_address(c"glBufferData")) };
        let glAttachShader = unsafe { mem::transmute(get_proc_address(c"glAttachShader")) };
        let glCompileShader = unsafe { mem::transmute(get_proc_address(c"glCompileShader")) };
        let glCreateProgram = unsafe { mem::transmute(get_proc_address(c"glCreateProgram")) };
        let glCreateShader = unsafe { mem::transmute(get_proc_address(c"glCreateShader")) };
        let glDeleteProgram = unsafe { mem::transmute(get_proc_address(c"glDeleteProgram")) };
        let glDeleteShader = unsafe { mem::transmute(get_proc_address(c"glDeleteShader")) };
        let glDisableVertexAttribArray = unsafe { mem::transmute(get_proc_address(c"glDisableVertexAttribArray")) };
        let glEnableVertexAttribArray = unsafe { mem::transmute(get_proc_address(c"glEnableVertexAttribArray")) };
        let glGetAttribLocation = unsafe { mem::transmute(get_proc_address(c"glGetAttribLocation")) };
        let glGetUniformLocation = unsafe { mem::transmute(get_proc_address(c"glGetUniformLocation")) };
        let glLinkProgram = unsafe { mem::transmute(get_proc_address(c"glLinkProgram")) };
        let glShaderSource = unsafe { mem::transmute(get_proc_address(c"glShaderSource")) };
        let glUseProgram = unsafe { mem::transmute(get_proc_address(c"glUseProgram")) };
        let glUniformMatrix4fv = unsafe { mem::transmute(get_proc_address(c"glUniformMatrix4fv")) };
        let glVertexAttribPointer = unsafe { mem::transmute(get_proc_address(c"glVertexAttribPointer")) };
        let glBindFramebuffer = unsafe { mem::transmute(get_proc_address(c"glBindFramebuffer")) };
        let glBindVertexArray = unsafe { mem::transmute(get_proc_address(c"glBindVertexArray")) };
        let glDeleteVertexArrays = unsafe { mem::transmute(get_proc_address(c"glDeleteVertexArrays")) };
        let glGenVertexArrays = unsafe { mem::transmute(get_proc_address(c"glGenVertexArrays")) };
        Self {
            glClearColor,
            glClear,
            glEnable,
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
            glBindFramebuffer,
            glBindVertexArray,
            glDeleteVertexArrays,
            glGenVertexArrays,
        }
    }

    pub unsafe fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        self.glClearColor.unwrap_unchecked()(red, green, blue, alpha)
    }

    pub unsafe fn clear(&self, mask: GLbitfield) {
        self.glClear.unwrap_unchecked()(mask)
    }

    pub unsafe fn enable(&self, cap: GLenum) {
        self.glEnable.unwrap_unchecked()(cap)
    }

    pub unsafe fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        self.glViewport.unwrap_unchecked()(x, y, width, height)
    }

    pub unsafe fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        self.glDrawArrays.unwrap_unchecked()(mode, first, count)
    }

    pub unsafe fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
        self.glBindBuffer.unwrap_unchecked()(target, buffer)
    }

    pub unsafe fn delete_buffers(&self, buffers: &[GLuint]) {
        self.glDeleteBuffers.unwrap_unchecked()(buffers.len() as _, buffers.as_ptr())
    }

    pub unsafe fn gen_buffers(&self, buffers: &mut [GLuint]) {
        self.glGenBuffers.unwrap_unchecked()(buffers.len() as _, buffers.as_mut_ptr())
    }

    pub unsafe fn buffer_data<T>(&self, target: GLenum, data: &[T], usage: GLenum) {
        self.glBufferData.unwrap_unchecked()(target, (data.len() * mem::size_of::<T>()) as _, data.as_ptr() as _, usage)
    }

    pub unsafe fn attach_shader(&self, program: GLuint, shader: GLuint) {
        self.glAttachShader.unwrap_unchecked()(program, shader)
    }

    pub unsafe fn compile_shader(&self, shader: GLuint) {
        self.glCompileShader.unwrap_unchecked()(shader)
    }

    pub unsafe fn create_program(&self) -> GLuint {
        self.glCreateProgram.unwrap_unchecked()()
    }

    pub unsafe fn create_shader(&self, type_: GLenum) -> GLuint {
        self.glCreateShader.unwrap_unchecked()(type_)
    }

    pub unsafe fn delete_program(&self, program: GLuint) {
        self.glDeleteProgram.unwrap_unchecked()(program)
    }

    pub unsafe fn delete_shader(&self, shader: GLuint) {
        self.glDeleteShader.unwrap_unchecked()(shader)
    }

    pub unsafe fn disable_vertex_attrib_array(&self, index: GLuint) {
        self.glDisableVertexAttribArray.unwrap_unchecked()(index)
    }

    pub unsafe fn enable_vertex_attrib_array(&self, index: GLuint) {
        self.glEnableVertexAttribArray.unwrap_unchecked()(index)
    }

    pub unsafe fn get_attrib_location(&self, program: GLuint, name: &CStr) -> GLint {
        self.glGetAttribLocation.unwrap_unchecked()(program, name.as_ptr())
    }

    pub unsafe fn get_uniform_location(&self, program: GLuint, name: &CStr) -> GLint {
        self.glGetUniformLocation.unwrap_unchecked()(program, name.as_ptr())
    }

    pub unsafe fn link_program(&self, program: GLuint) {
        self.glLinkProgram.unwrap_unchecked()(program)
    }

    pub unsafe fn shader_source(&self, shader: GLuint, string: &[*const GLchar], length: Option<&[GLint]>) {
        if let Some(length) = length {
            assert_eq!(string.len(), length.len());
        }
        self.glShaderSource.unwrap_unchecked()(shader, string.len() as _, string.as_ptr(), if let Some(length) = length { length.as_ptr() } else { null() });
    }

    pub unsafe fn use_program(&self, program: GLuint) {
        self.glUseProgram.unwrap_unchecked()(program)
    }

    pub unsafe fn uniform_matrix_4fv(&self, location: GLint, count: GLsizei, transpose: bool, value: &[GLfloat]) {
        self.glUniformMatrix4fv.unwrap_unchecked()(location, count, if transpose { TRUE } else { FALSE }, value.as_ptr())
    }

    pub unsafe fn vertex_attrib_pointer(&self, index: GLuint, size: usize, type_: GLenum, normalized: bool, stride: usize, pointer: usize) {
        self.glVertexAttribPointer.unwrap_unchecked()(index, size as _, type_, if normalized { TRUE } else { FALSE }, stride as _, pointer as _)
    }

    pub unsafe fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {
        self.glBindFramebuffer.unwrap_unchecked()(target, framebuffer)
    }

    pub unsafe fn bind_vertex_array(&self, array: GLuint) {
        self.glBindVertexArray.unwrap_unchecked()(array)
    }

    pub unsafe fn delete_vertex_arrays(&self, arrays: &[GLuint]) {
        self.glDeleteVertexArrays.unwrap_unchecked()(arrays.len() as _, arrays.as_ptr())
    }

    pub unsafe fn gen_vertex_arrays(&self, arrays: &mut [GLuint]) {
        self.glGenVertexArrays.unwrap_unchecked()(arrays.len() as _, arrays.as_mut_ptr())
    }
}