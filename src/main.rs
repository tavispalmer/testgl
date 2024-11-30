use glfw::Context;
use testgl::TestGL;

fn main() {
    let mut testgl = TestGL::new();

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    #[cfg(feature = "core")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    }

    let (mut window, events) = glfw.create_window(testgl.width(), testgl.height(), "TestGL", glfw::WindowMode::Windowed)
        .unwrap();
    
    let (width, height) = window.get_framebuffer_size();
    testgl.set_width(width as _);
    testgl.set_height(height as _);
    #[cfg(feature = "core")]
    testgl.init_multisample(1);

    window.set_framebuffer_size_polling(true);
    window.make_current();

    testgl.context_reset(|cstr| {
        let str = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        window.get_proc_address(&str)
    });

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    while !window.should_close() {
        testgl.run(0);
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    testgl.set_width(width as _);
                    testgl.set_height(height as _);
                }
                _ => {}
            }
        }
    }
}