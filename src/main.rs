use std::mem;

use glfw::Context;
use testgl::TestGL;

fn main() {
    let mut testgl = TestGL::new();

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(testgl.width(), testgl.height(), "TestGL", glfw::WindowMode::Windowed)
        .unwrap();
    
    let (width, height) = window.get_framebuffer_size();
    testgl.set_width(width as _);
    testgl.set_height(height as _);

    window.set_size_polling(true);
    window.make_current();

    testgl.context_reset(|str| unsafe { mem::transmute(window.get_proc_address(str)) } );

    while !window.should_close() {
        testgl.run(0);
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Size(_, _) => {
                    let (width, height) = window.get_framebuffer_size();
                    testgl.set_width(width as _);
                    testgl.set_height(height as _);
                }
                _ => {}
            }
        }
    }

    testgl.context_destroy();
}