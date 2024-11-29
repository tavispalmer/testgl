// libretro interface

use std::{ffi::{c_char, c_uint, c_void, CStr, CString}, mem, ptr};

use crate::{retro, TestGL};

static mut TESTGL: TestGL = TestGL::new();

static mut HW_RENDER: retro::HwRenderCallback = retro::HwRenderCallback {
    context_type: retro::HwContextType::None,
    context_reset: None,
    get_current_framebuffer: None,
    get_proc_address: None,
    depth: false,
    stencil: false,
    bottom_left_origin: false,
    version_major: 0,
    version_minor: 0,
    cache_context: false,
    context_destroy: None,
    debug_context: false,
};

static mut VIDEO_CB: retro::VideoRefresh = None;
static mut AUDIO_CB: retro::AudioSample = None;
static mut AUDIO_BATCH_CB: retro::AudioSampleBatch = None;
static mut ENVIRON_CB: retro::Environment = None;
static mut INPUT_POLL_CB: retro::InputPoll = None;
static mut INPUT_STATE_CB: retro::InputState = None;

#[no_mangle]
pub extern "C" fn retro_set_environment(cb: retro::Environment) {
    unsafe { ENVIRON_CB = cb };
    let cb = unsafe { cb.unwrap_unchecked() };

    const VARIABLES: [retro::Variable; 3] = [
        retro::Variable {
            key: c"testgl_resolution".as_ptr(),
            value: c"Internal resolution; 320x240|360x480|480x272|512x384|512x512|640x240|640x448|640x480|720x576|800x600|960x720|1024x768|1024x1024|1280x720|1280x960|1600x1200|1920x1080|1920x1440|1920x1600|2048x2048".as_ptr(),
        },
        retro::Variable {
            key: c"testgl_multisample".as_ptr(),
            value: c"Multisampling; 1x|2x|4x".as_ptr(),
        },
        retro::Variable { key: ptr::null(), value: ptr::null() },
    ];

    let no_rom = true;
    unsafe {
        cb(retro::ENVIRONMENT_SET_SUPPORT_NO_GAME, ptr::addr_of!(no_rom) as _);
        cb(retro::ENVIRONMENT_SET_VARIABLES, VARIABLES.as_ptr() as _);
    }
}

#[no_mangle]
pub extern "C" fn retro_set_video_refresh(cb: retro::VideoRefresh) {
    unsafe { VIDEO_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_audio_sample(cb: retro::AudioSample) {
    unsafe { AUDIO_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_audio_sample_batch(cb: retro::AudioSampleBatch) {
    unsafe { AUDIO_BATCH_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_input_poll(cb: retro::InputPoll) {
    unsafe { INPUT_POLL_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_input_state(cb: retro::InputState) {
    unsafe { INPUT_STATE_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_init() {}

#[no_mangle]
pub extern "C" fn retro_deinit() {}

#[no_mangle]
pub extern "C" fn retro_api_version() -> c_uint {
    retro::API_VERSION
}

#[no_mangle]
pub extern "C" fn retro_get_system_info(info: *mut retro::SystemInfo) {
    let info = unsafe { &mut *info };
    *info = unsafe { mem::zeroed() };
    info.library_name = c"TestCore GL".as_ptr();
    info.library_version = c"v1".as_ptr();
    info.need_fullpath = false;
    info.valid_extensions = ptr::null();
}

#[no_mangle]
pub extern "C" fn retro_get_system_av_info(info: *mut retro::SystemAvInfo) {
    let info = unsafe { &mut *info };
    info.timing = retro::SystemTiming {
        fps: 60.0,
        sample_rate: 0.0,
    };

    info.geometry = retro::GameGeometry {
        base_width: TestGL::BASE_WIDTH,
        base_height: TestGL::BASE_HEIGHT,
        max_width: TestGL::MAX_WIDTH,
        max_height: TestGL::MAX_HEIGHT,
        aspect_ratio: 4.0 / 3.0,
    };
}

#[no_mangle]
pub extern "C" fn retro_set_controller_port_device(port: c_uint, device: c_uint) {}

#[no_mangle]
pub extern "C" fn retro_reset() {}

fn update_variables() {
    let environ_cb = unsafe { ENVIRON_CB.unwrap_unchecked() };

    let mut var = retro::Variable {
        key: c"testgl_resolution".as_ptr(),
        value: ptr::null(),
    };

    if unsafe { environ_cb(retro::ENVIRONMENT_GET_VARIABLE, ptr::addr_of_mut!(var) as _) && var.value != ptr::null() } {
        let cstr = unsafe { CStr::from_ptr(var.value) };
        let string = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        let result: Vec<_> = string.split('x').collect();
        unsafe {
            TESTGL.width = result[0].parse().unwrap();
            TESTGL.height = result[1].parse().unwrap();
            eprintln!("[libretro-test]: Got size: {} x {}.", TESTGL.width, TESTGL.height);
        }
    }

    var.key = c"testgl_multisample".as_ptr();
    var.value = ptr::null();

    if unsafe { environ_cb(retro::ENVIRONMENT_GET_VARIABLE, ptr::addr_of_mut!(var) as _) && var.value != ptr::null() } {
        match unsafe { *var.value } as u8 {
            b'1' => unsafe { TESTGL.init_multisample(1) }
            b'2' => unsafe { TESTGL.init_multisample(2) }
            b'4' => unsafe { TESTGL.init_multisample(4) }
            _ => {}
        }
    }
}

#[no_mangle]
pub extern "C" fn retro_run() {
    let environ_cb = unsafe { ENVIRON_CB.unwrap_unchecked() };
    let video_cb = unsafe { VIDEO_CB.unwrap_unchecked() };
    let input_poll_cb = unsafe { INPUT_POLL_CB.unwrap_unchecked() };

    let mut updated = false;
    if unsafe { environ_cb(retro::ENVIRONMENT_GET_VARIABLE_UPDATE, ptr::addr_of_mut!(updated) as _) && updated } {
        update_variables();
    }

    unsafe { input_poll_cb() };

    let framebuffer = unsafe { HW_RENDER.get_current_framebuffer.unwrap_unchecked()() } as _;
    unsafe { TESTGL.run(framebuffer) };
    unsafe { video_cb(retro::HW_FRAME_BUFFER_VALID, TESTGL.width, TESTGL.height, 0) };
}

#[no_mangle]
pub extern "C" fn retro_serialize_size() -> usize { 0 }

#[no_mangle]
pub extern "C" fn retro_serialize(data: *mut c_void, size: usize) -> bool { false }

#[no_mangle]
pub extern "C" fn retro_unserialize(data: *const c_void, size: usize) -> bool { false }

#[no_mangle]
pub extern "C" fn retro_cheat_reset() {}

#[no_mangle]
pub extern "C" fn retro_cheat_set(index: c_uint, enabled: bool, code: *const c_char) {}


fn retro_init_hw_context() -> bool {
    let environ_cb = unsafe { ENVIRON_CB.unwrap_unchecked() };

    extern "C" fn context_reset() {
        eprintln!("Context reset!");
        unsafe {
            TESTGL.context_reset(|str| {
                let cstr = CString::new(str).unwrap();
                HW_RENDER.get_proc_address.unwrap_unchecked()(cstr.as_ptr())
            });
        }
    }

    extern "C" fn context_destroy() {
        eprintln!("Context destroy!");
        unsafe {
            TESTGL.context_destroy();
        }
    }

    unsafe {
        HW_RENDER.context_type = retro::HwContextType::OpenGLCore;
        HW_RENDER.version_major = 3;
        HW_RENDER.version_minor = 1;
        HW_RENDER.context_reset = Some(context_reset);
        HW_RENDER.context_destroy = Some(context_destroy);
        HW_RENDER.depth = true;
        HW_RENDER.stencil = true;
        HW_RENDER.bottom_left_origin = true;
    }

    if !unsafe { environ_cb(retro::ENVIRONMENT_SET_HW_RENDER, ptr::addr_of_mut!(HW_RENDER) as _) } {
        false
    } else {
        true
    }
}

#[no_mangle]
pub extern "C" fn retro_load_game(info: *const retro::GameInfo) -> bool {
    let environ_cb = unsafe { ENVIRON_CB.unwrap_unchecked() };

    update_variables();

    let fmt = retro::PixelFormat::XRGB8888;
    if !unsafe { environ_cb(retro::ENVIRONMENT_SET_PIXEL_FORMAT, ptr::addr_of!(fmt) as _) } {
        eprintln!("XRGB8888 is not supported.");
        return false;
    }

    if !retro_init_hw_context() {
        eprintln!("HW Context could not be initialized, exiting...");
        return false;
    }

    eprintln!("Loaded game!");
    true
}

#[no_mangle]
pub extern "C" fn retro_load_game_special(type_: c_uint, info: *const retro::GameInfo, num: usize) -> bool { false }

#[no_mangle]
pub extern "C" fn retro_unload_game() {}

#[no_mangle]
pub extern "C" fn retro_get_region() -> c_uint { retro::REGION_NTSC }

#[no_mangle]
pub extern "C" fn retro_get_memory_data(id: c_uint) -> *mut c_void { ptr::null_mut() }

#[no_mangle]
pub extern "C" fn retro_get_memory_size(id: c_uint) -> usize { 0 }
