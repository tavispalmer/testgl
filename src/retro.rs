#![allow(non_camel_case_types)]

use std::{ffi::{c_char, c_int, c_uint, c_void, CStr}, marker::PhantomData, ptr};

pub const API_VERSION: c_uint = 1;  // 97

pub const REGION_NTSC: c_uint = 0;

pub const ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;
pub const ENVIRONMENT_SET_HW_RENDER: c_uint = 14;
pub const ENVIRONMENT_GET_VARIABLE: c_uint = 15;
pub const ENVIRONMENT_SET_VARIABLES: c_uint = 16;
pub const ENVIRONMENT_GET_VARIABLE_UPDATE: c_uint = 17;
pub const ENVIRONMENT_SET_SUPPORT_NO_GAME: c_uint = 18;

pub type proc_address_t = Option<unsafe extern "C" fn()>;

pub const HW_FRAME_BUFFER_VALID: *const c_void = -1 as isize as _;

pub type hw_context_reset_t = Option<unsafe extern "C" fn()>;
pub type hw_get_current_framebuffer_t = Option<unsafe extern "C" fn() -> usize>;
pub type hw_get_proc_address_t = Option<unsafe extern "C" fn(sym: *const c_char) -> proc_address_t>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum hw_context_type {
    NONE = 0,
    OPENGL = 1,
    OPENGLES2 = 2,
    OPENGL_CORE = 3,
    OPENGLES3 = 4,
    OPENGLES_VERSION = 5,
    VULKAN = 6,
    D3D11 = 7,
    D3D10 = 8,
    D3D12 = 9,
    D3D9 = 10,
    DUMMY = c_int::MAX as isize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hw_render_callback {
    pub context_type: hw_context_type,
    pub context_reset: hw_context_reset_t,
    pub get_current_framebuffer: hw_get_current_framebuffer_t,
    pub get_proc_address: hw_get_proc_address_t,
    pub depth: bool,
    pub stencil: bool,
    pub bottom_left_origin: bool,
    pub version_major: c_uint,
    pub version_minor: c_uint,
    pub cache_context: bool,
    pub context_destroy: hw_context_reset_t,
    pub debug_context: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    TYPE_0RGB1555 = 0,
    XRGB8888 = 1,
    RGB565 = 2,
    UNKNOWN = c_int::MAX as isize,
}

// 5966
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct system_info<'a> {
    pub library_name: *const c_char,
    pub library_version: *const c_char,
    pub valid_extensions: *const c_char,
    pub need_fullpath: bool,
    pub block_extract: bool,
    pub _marker: PhantomData<&'a ()>,
}

impl<'a> system_info<'a> {
    #[inline]
    pub fn library_name(mut self, library_name: &'a CStr) -> Self {
        self.library_name = library_name.as_ptr();
        self
    }

    #[inline]
    pub fn library_version(mut self, library_version: &'a CStr) -> Self {
        self.library_version = library_version.as_ptr();
        self
    }

    #[inline]
    pub fn valid_extensions(mut self, valid_extensions: &'a CStr) -> Self {
        self.valid_extensions = valid_extensions.as_ptr();
        self
    }

    #[inline]
    pub fn need_fullpath(mut self, need_fullpath: bool) -> Self {
        self.need_fullpath = need_fullpath;
        self
    }

    #[inline]
    pub fn block_extract(mut self, block_extract: bool) -> Self {
        self.block_extract = block_extract;
        self
    }
}

impl Default for system_info<'_> {
    fn default() -> Self {
        Self {
            library_name: ptr::null(),
            library_version: ptr::null(),
            valid_extensions: ptr::null(),
            need_fullpath: false,
            block_extract: false,
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct game_geometry {
    pub base_width: c_uint,
    pub base_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub aspect_ratio: f32,
}

impl game_geometry {
    #[inline]
    pub fn base_width(mut self, base_width: c_uint) -> Self {
        self.base_width = base_width;
        self
    }

    #[inline]
    pub fn base_height(mut self, base_height: c_uint) -> Self {
        self.base_height = base_height;
        self
    }

    #[inline]
    pub fn max_width(mut self, max_width: c_uint) -> Self {
        self.max_width = max_width;
        self
    }

    #[inline]
    pub fn max_height(mut self, max_height: c_uint) -> Self {
        self.max_height = max_height;
        self
    }

    #[inline]
    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }
}

impl Default for game_geometry {
    fn default() -> Self {
        Self {
            base_width: 0,
            base_height: 0,
            max_width: 0,
            max_height: 0,
            aspect_ratio: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct system_timing {
    pub fps: f64,
    pub sample_rate: f64,
}

impl system_timing {
    #[inline]
    pub fn fps(mut self, fps: f64) -> Self {
        self.fps = fps;
        self
    }

    #[inline]
    pub fn sample_rate(mut self, sample_rate: f64) -> Self {
        self.sample_rate = sample_rate;
        self
    }
}

impl Default for system_timing {
    fn default() -> Self {
        Self {
            fps: 0.0,
            sample_rate: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct system_av_info {
    pub geometry: game_geometry,
    pub timing: system_timing,
}

impl system_av_info {
    #[inline]
    pub fn geometry(mut self, geometry: game_geometry) -> Self {
        self.geometry = geometry;
        self
    }

    #[inline]
    pub fn timing(mut self, timing: system_timing) -> Self {
        self.timing = timing;
        self
    }
}

impl Default for system_av_info {
    fn default() -> Self {
        Self {
            geometry: game_geometry::default(),
            timing: system_timing::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct variable<'a> {
    pub key: *const c_char,
    pub value: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}

impl<'a> variable<'a> {
    #[inline]
    pub fn key(mut self, key: &'a CStr) -> Self {
        self.key = key.as_ptr();
        self
    }

    #[inline]
    pub fn value(mut self, value: &'a CStr) -> Self {
        self.value = value.as_ptr();
        self
    }
}

impl Default for variable<'_> {
    fn default() -> Self {
        Self {
            key: ptr::null(),
            value: ptr::null(),
            _marker: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct game_info {
    pub path: *const c_char,
    pub data: *const c_void,
    pub size: usize,
    pub meta: *const c_char,
}

pub type environment_t = unsafe extern "C" fn(cmd: c_uint, data: *mut c_void) -> bool;
pub type video_refresh_t = unsafe extern "C" fn(data: *const c_void, width: c_uint, height: c_uint, pitch: usize);
pub type audio_sample_t = unsafe extern "C" fn(left: i16, right: i16);
pub type audio_sample_batch_t = unsafe extern "C" fn(data: *const i16, frames: usize) -> usize;
pub type input_poll_t = unsafe extern "C" fn();
pub type input_state_t = unsafe extern "C" fn(port: c_uint, device: c_uint, index: c_uint, id: c_uint) -> i16;
