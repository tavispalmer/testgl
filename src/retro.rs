use std::{ffi::{c_char, c_int, c_uint, c_void, CStr}, ptr};

pub const API_VERSION: c_uint = 1;  // 97

pub const REGION_NTSC: c_uint = 0;

pub const ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;
pub const ENVIRONMENT_SET_HW_RENDER: c_uint = 14;
pub const ENVIRONMENT_GET_VARIABLE: c_uint = 15;
pub const ENVIRONMENT_SET_VARIABLES: c_uint = 16;
pub const ENVIRONMENT_GET_VARIABLE_UPDATE: c_uint = 17;
pub const ENVIRONMENT_SET_SUPPORT_NO_GAME: c_uint = 18;

pub type ProcAddress = Option<unsafe extern "C" fn()>;

pub const HW_FRAME_BUFFER_VALID: *const c_void = -1 as isize as _;

pub type HwContextReset = Option<unsafe extern "C" fn()>;
pub type HwGetCurrentFramebuffer = Option<unsafe extern "C" fn() -> usize>;
pub type HwGetProcAddress = Option<unsafe extern "C" fn(sym: *const c_char) -> ProcAddress>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum HwContextType {
    None = 0,
    OpenGL = 1,
    OpenGLES2 = 2,
    OpenGLCore = 3,
    OpenGLES3 = 4,
    OpenGLESVersion = 5,
    Vulkan = 6,
    D3D11 = 7,
    D3D10 = 8,
    D3D12 = 9,
    D3D9 = 10,
    Dummy = c_int::MAX as isize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HwRenderCallback {
    pub context_type: HwContextType,
    pub context_reset: HwContextReset,
    pub get_current_framebuffer: HwGetCurrentFramebuffer,
    pub get_proc_address: HwGetProcAddress,
    pub depth: bool,
    pub stencil: bool,
    pub bottom_left_origin: bool,
    pub version_major: c_uint,
    pub version_minor: c_uint,
    pub cache_context: bool,
    pub context_destroy: HwContextReset,
    pub debug_context: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    _0RGB1555 = 0,
    XRGB8888 = 1,
    RGB565 = 2,
    Unknown = c_int::MAX as isize,
}

// 5966
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemInfo {
    pub library_name: *const c_char,
    pub library_version: *const c_char,
    pub valid_extensions: *const c_char,
    pub need_fullpath: bool,
    pub block_extract: bool,
}

impl SystemInfo {
    #[inline]
    pub fn library_name(mut self, library_name: &CStr) -> Self {
        self.library_name = library_name.as_ptr();
        self
    }

    #[inline]
    pub fn library_version(mut self, library_version: &CStr) -> Self {
        self.library_version = library_version.as_ptr();
        self
    }

    #[inline]
    pub fn valid_extensions(mut self, valid_extensions: &CStr) -> Self {
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

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            library_name: ptr::null(),
            library_version: ptr::null(),
            valid_extensions: ptr::null(),
            need_fullpath: false,
            block_extract: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameGeometry {
    pub base_width: c_uint,
    pub base_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub aspect_ratio: f32,
}

impl GameGeometry {
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

impl Default for GameGeometry {
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
pub struct SystemTiming {
    pub fps: f64,
    pub sample_rate: f64,
}

impl SystemTiming {
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

impl Default for SystemTiming {
    fn default() -> Self {
        Self {
            fps: 0.0,
            sample_rate: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemAvInfo {
    pub geometry: GameGeometry,
    pub timing: SystemTiming,
}

impl SystemAvInfo {
    #[inline]
    pub fn geometry(mut self, geometry: GameGeometry) -> Self {
        self.geometry = geometry;
        self
    }

    #[inline]
    pub fn timing(mut self, timing: SystemTiming) -> Self {
        self.timing = timing;
        self
    }
}

impl Default for SystemAvInfo {
    fn default() -> Self {
        Self {
            geometry: GameGeometry::default(),
            timing: SystemTiming::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Variable {
    pub key: *const c_char,
    pub value: *const c_char,
}

impl Variable {
    #[inline]
    pub fn key(mut self, key: &CStr) -> Self {
        self.key = key.as_ptr();
        self
    }

    #[inline]
    pub fn value(mut self, value: &CStr) -> Self {
        self.value = value.as_ptr();
        self
    }
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            key: ptr::null(),
            value: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameInfo {
    pub path: *const c_char,
    pub data: *const c_void,
    pub size: usize,
    pub meta: *const c_char,
}

pub type Environment = unsafe extern "C" fn(cmd: c_uint, data: *mut c_void) -> bool;
pub type VideoRefresh = unsafe extern "C" fn(data: *const c_void, width: c_uint, height: c_uint, pitch: usize);
pub type AudioSample = unsafe extern "C" fn(left: i16, right: i16);
pub type AudioSampleBatch = unsafe extern "C" fn(data: *const i16, frames: usize) -> usize;
pub type InputPoll = unsafe extern "C" fn();
pub type InputState = unsafe extern "C" fn(port: c_uint, device: c_uint, index: c_uint, id: c_uint) -> i16;
