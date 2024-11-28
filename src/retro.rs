use std::ffi::{c_char, c_int, c_uint, c_void};

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameGeometry {
    pub base_width: c_uint,
    pub base_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub aspect_ratio: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemTiming {
    pub fps: f64,
    pub sample_rate: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemAvInfo {
    pub geometry: GameGeometry,
    pub timing: SystemTiming,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Variable {
    pub key: *const c_char,
    pub value: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameInfo {
    pub path: *const c_char,
    pub data: *const c_void,
    pub size: usize,
    pub meta: *const c_char,
}

pub type Environment = Option<unsafe extern "C" fn(cmd: c_uint, data: *mut c_void) -> bool>;
pub type VideoRefresh = Option<unsafe extern "C" fn(data: *const c_void, width: c_uint, height: c_uint, pitch: usize)>;
pub type AudioSample = Option<unsafe extern "C" fn(left: i16, right: i16)>;
pub type AudioSampleBatch = Option<unsafe extern "C" fn(data: *const i16, frames: usize) -> usize>;
pub type InputPoll = Option<unsafe extern "C" fn()>;
pub type InputState = Option<unsafe extern "C" fn(port: c_uint, device: c_uint, index: c_uint, id: c_uint) -> i16>;
