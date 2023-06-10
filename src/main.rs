use std::sync::mpsc::Receiver;

extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

mod renderer;
mod buffer;
mod shader;
mod math;
mod vertex_array;

use renderer::Renderer;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe{
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
    }
     
    //NOT SETUP CODE HERE    
    let mut world_mat = math::Mat4::identity();
    world_mat.ortho(SCR_WIDTH as f32, SCR_HEIGHT as f32);

    let mut renderer = Renderer::new(world_mat);
    let result = renderer.setup_quad_info(math::Point3::new(0.0, 0.0, 1.0), math::Point3::new(400.0, 300.0, 0.0), math::Point3::new(0.0, 1.0, 0.0));

    match result{
        Err(err_message) => {
            println!("{}", err_message);
            assert!(false);
        },
        Ok(_) => (),
    }

    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);
        renderer.clear_surface(math::Point3::new(0.2, 0.3, 0.4));
        renderer.draw_quad();
        
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}