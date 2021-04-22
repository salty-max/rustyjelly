extern crate gl;
extern crate sdl2;

mod gl_utility;

mod prelude {
    pub use crate::gl_utility::prelude::*;
    pub use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};
    pub use std::ffi::{CStr, CString};
}

use prelude::*;

// Crash on macOS
// extern "system" fn dbg_callback(
//     source: gl::types::GLenum,
//     etype: gl::types::GLenum,
//     _id: gl::types::GLuint,
//     severity: gl::types::GLenum,
//     _msg_length: gl::types::GLsizei,
//     msg: *const gl::types::GLchar,
//     _user_data: *mut std::ffi::c_void,
// ) {
//     unsafe {
//         println!(
//             "dbg_callback {:#X} {:#X} {:#X} {:?}",
//             source,
//             etype,
//             severity,
//             std::ffi::CStr::from_ptr(msg),
//         );
//     }
// }

fn main() -> Result<(), String> {
    println!("Hello, JellyEngine!");

    // Init window
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);

    let window = video_subsystem
        .window("JellyEngine", 800, 600)
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create window");

    let _ctx = window.gl_create_context()?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        // gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());
    }

    println!(
        "Pixel format of the window's GL context: {:?}",
        window.window_pixel_format()
    );
    println!(
        "OpenGL Profile: {:?} - OpenGL version: {:?}",
        gl_attr.context_profile(),
        gl_attr.context_version(),
    );

    let mut shader_manager = ShaderManager::init();
    let basic_shader = shader_manager.register(
        "basic",
        include_str!("basic.vert"),
        include_str!("basic.frag"),
    );

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0, -0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5,
        0.0,
    ];

    let a_position_location = basic_shader.get_attribute_location("a_position");
    let attrib_info = AttributeInfo {
        location: a_position_location,
        component_size: 3,
    };

    let mut buffer = GLBuffer::new();
    buffer.configure(vec![attrib_info], false);
    buffer.set_data(&vertices);
    buffer.upload();

    basic_shader.use_shader();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut event_pump = sdl_context.event_pump()?;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main_loop;
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::R, _) => {
                        println!("red");
                        unsafe {
                            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::G, _) => {
                        println!("green");
                        unsafe {
                            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::B, _) => {
                        println!("blue");
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw triangle
            let colors: Vec<f32> = vec![1.0, 0.5, 0.5, 1.0];
            gl::Uniform4fv(
                basic_shader.get_uniform_location("u_color"), // uniform position (u_color)
                1,
                colors.as_ptr() as *const gl::types::GLfloat,
            );

            buffer.draw();
        }
        window.gl_swap_window();
    }

    Ok(())
}
