// libretro interface

use std::{ffi::{c_char, c_uint, c_void, CStr}, marker::PhantomData, mem, ptr};

use crate::{retro, TestGL};

static mut TESTGL: TestGL = TestGL::new();

static mut HW_RENDER: retro::hw_render_callback = retro::hw_render_callback {
    context_type: retro::hw_context_type::NONE,
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

static mut VIDEO_CB: retro::video_refresh_t = {
    unsafe extern "C" fn cb(_data: *const c_void, _width: c_uint, _height: c_uint, _pitch: usize) {
        panic!(concat!("Unable to load ", stringify!(VIDEO_CB)))
    }
    cb
};
static mut AUDIO_CB: retro::audio_sample_t = {
    unsafe extern "C" fn cb(_left: i16, _right: i16) {
        panic!(concat!("Unable to load ", stringify!(AUDIO_CB)))
    }
    cb
};
static mut AUDIO_BATCH_CB: retro::audio_sample_batch_t = {
    unsafe extern "C" fn cb(_data: *const i16, _frames: usize) -> usize {
        panic!(concat!("Unable to load ", stringify!(AUDIO_BATCH_CB)))
    }
    cb
};
static mut ENVIRON_CB: retro::environment_t = {
    unsafe extern "C" fn cb(_cmd: c_uint, _data: *mut c_void) -> bool {
        panic!(concat!("Unable to load ", stringify!(ENVIRON_CB)))
    }
    cb
};
static mut INPUT_POLL_CB: retro::input_poll_t = {
    unsafe extern "C" fn cb() {
        panic!(concat!("Unable to load ", stringify!(INPUT_POLL_CB)))
    }
    cb
};
static mut INPUT_STATE_CB: retro::input_state_t = {
    unsafe extern "C" fn cb(_port: c_uint, _device: c_uint, _index: c_uint, _id: c_uint) -> i16 {
        panic!(concat!("Unable to load ", stringify!(INPUT_STATE_CB)))
    }
    cb
};

#[no_mangle]
pub extern "C" fn retro_set_environment(cb: retro::environment_t) {
    unsafe { ENVIRON_CB = cb };

    #[cfg(feature = "core")]
    const VARIABLES: [retro::variable; 3] = [
        retro::variable {
            key: c"testgl_resolution".as_ptr(),
            value: c"Internal resolution; 320x240|360x480|480x272|512x384|512x512|640x240|640x448|640x480|720x576|800x600|960x720|1024x768|1024x1024|1280x720|1280x960|1600x1200|1920x1080|1920x1440|1920x1600|2048x2048".as_ptr(),
            _marker: PhantomData,
        },
        retro::variable {
            key: c"testgl_multisample".as_ptr(),
            value: c"Multisampling; 1x|2x|4x".as_ptr(),
            _marker: PhantomData,
        },
        retro::variable { key: ptr::null(), value: ptr::null(), _marker: PhantomData },
    ];

    #[cfg(not(feature = "core"))]
    const VARIABLES: [retro::variable; 2] = [
        retro::variable {
            key: c"testgl_resolution".as_ptr(),
            value: c"Internal resolution; 320x240|360x480|480x272|512x384|512x512|640x240|640x448|640x480|720x576|800x600|960x720|1024x768|1024x1024|1280x720|1280x960|1600x1200|1920x1080|1920x1440|1920x1600|2048x2048".as_ptr(),
            _marker: PhantomData,
        },
        retro::variable { key: ptr::null(), value: ptr::null(), _marker: PhantomData },
    ];

    let no_rom = true;
    unsafe {
        cb(retro::ENVIRONMENT_SET_SUPPORT_NO_GAME, ptr::addr_of!(no_rom) as _);
        cb(retro::ENVIRONMENT_SET_VARIABLES, VARIABLES.as_ptr() as _);
    }
}

#[no_mangle]
pub extern "C" fn retro_set_video_refresh(cb: retro::video_refresh_t) {
    unsafe { VIDEO_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_audio_sample(cb: retro::audio_sample_t) {
    unsafe { AUDIO_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_audio_sample_batch(cb: retro::audio_sample_batch_t) {
    unsafe { AUDIO_BATCH_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_input_poll(cb: retro::input_poll_t) {
    unsafe { INPUT_POLL_CB = cb };
}

#[no_mangle]
pub extern "C" fn retro_set_input_state(cb: retro::input_state_t) {
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
pub extern "C" fn retro_get_system_info(info: *mut retro::system_info) {
    unsafe {
        *info = retro::system_info::default()
            .library_name(c"TestCore GL")
            .library_version(c"v1");
    }
}

#[no_mangle]
pub extern "C" fn retro_get_system_av_info(info: *mut retro::system_av_info) {
    unsafe {
        *info = retro::system_av_info::default()
            .timing(retro::system_timing::default()
                .fps(60.0))
            .geometry(retro::game_geometry::default()
                .base_width(TestGL::BASE_WIDTH)
                .base_height(TestGL::BASE_HEIGHT)
                .max_width(TestGL::MAX_WIDTH)
                .max_height(TestGL::MAX_HEIGHT)
                .aspect_ratio(4.0 / 3.0));
    }
}

#[no_mangle]
pub extern "C" fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}

#[no_mangle]
pub extern "C" fn retro_reset() {}

fn update_variables() {
    let mut var = retro::variable::default()
        .key(c"testgl_resolution");

    if unsafe { ENVIRON_CB(retro::ENVIRONMENT_GET_VARIABLE, ptr::addr_of_mut!(var) as _) && var.value != ptr::null() } {
        let cstr = unsafe { CStr::from_ptr(var.value) };
        let string = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        let result: Vec<_> = string.split('x').collect();
        unsafe {
            TESTGL.set_width(result[0].parse().unwrap());
            TESTGL.set_height(result[1].parse().unwrap());
            eprintln!("[libretro-test]: Got size: {} x {}.", TESTGL.width(), TESTGL.height());
        }
    }

    #[cfg(feature = "core")]
    {
        var = retro::variable::default()
            .key(c"testgl_multisample");

        if unsafe { ENVIRON_CB(retro::ENVIRONMENT_GET_VARIABLE, ptr::addr_of_mut!(var) as _) && var.value != ptr::null() } {
            match unsafe { *var.value } as u8 {
                b'1' => unsafe { TESTGL.init_multisample(1) }
                b'2' => unsafe { TESTGL.init_multisample(2) }
                b'4' => unsafe { TESTGL.init_multisample(4) }
                _ => {}
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn retro_run() {
    let mut updated = false;
    if unsafe { ENVIRON_CB(retro::ENVIRONMENT_GET_VARIABLE_UPDATE, ptr::addr_of_mut!(updated) as _) && updated } {
        update_variables();
    }

    unsafe { INPUT_POLL_CB() };

    let framebuffer = unsafe { HW_RENDER.get_current_framebuffer.unwrap_unchecked()() } as _;
    unsafe { TESTGL.run(framebuffer) };
    unsafe { VIDEO_CB(retro::HW_FRAME_BUFFER_VALID, TESTGL.width, TESTGL.height, 0) };
}

#[no_mangle]
pub extern "C" fn retro_serialize_size() -> usize { 0 }

#[no_mangle]
pub extern "C" fn retro_serialize(_data: *mut c_void, _size: usize) -> bool { false }

#[no_mangle]
pub extern "C" fn retro_unserialize(_data: *const c_void, _size: usize) -> bool { false }

#[no_mangle]
pub extern "C" fn retro_cheat_reset() {}

#[no_mangle]
pub extern "C" fn retro_cheat_set(_index: c_uint, _enabled: bool, _code: *const c_char) {}

fn retro_init_hw_context() -> bool {
    extern "C" fn context_reset() {
        unsafe {
            TESTGL.context_reset(|cstr| {
                mem::transmute(HW_RENDER.get_proc_address.unwrap_unchecked()(cstr.as_ptr()))
            });
        }
    }

    extern "C" fn context_destroy() {
        unsafe {
            TESTGL.context_destroy();
        }
    }

    unsafe {
        #[cfg(feature = "core")]
        {
            HW_RENDER.context_type = retro::hw_context_type::OPENGL_CORE;
            HW_RENDER.version_major = 3;
            HW_RENDER.version_minor = 1;
        }
        #[cfg(not(feature = "core"))]
        {
            HW_RENDER.context_type = retro::hw_context_type::OPENGL;
        }
        HW_RENDER.context_reset = Some(context_reset);
        HW_RENDER.context_destroy = Some(context_destroy);
        HW_RENDER.depth = true;
        HW_RENDER.stencil = true;
        HW_RENDER.bottom_left_origin = true;
    }

    if !unsafe { ENVIRON_CB(retro::ENVIRONMENT_SET_HW_RENDER, ptr::addr_of_mut!(HW_RENDER) as _) } {
        false
    } else {
        true
    }
}

#[no_mangle]
pub extern "C" fn retro_load_game(_info: *const retro::game_info) -> bool {
    update_variables();

    let fmt = retro::PixelFormat::XRGB8888;
    if !unsafe { ENVIRON_CB(retro::ENVIRONMENT_SET_PIXEL_FORMAT, ptr::addr_of!(fmt) as _) } {
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
pub extern "C" fn retro_load_game_special(_type: c_uint, _info: *const retro::game_info, _num: usize) -> bool { false }

#[no_mangle]
pub extern "C" fn retro_unload_game() {}

#[no_mangle]
pub extern "C" fn retro_get_region() -> c_uint { retro::REGION_NTSC }

#[no_mangle]
pub extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut c_void { ptr::null_mut() }

#[no_mangle]
pub extern "C" fn retro_get_memory_size(_id: c_uint) -> usize { 0 }
