extern crate gl;
extern crate sdl2;

mod render_gl;

mod prelude {

    pub use crate::render_gl::{Program, Shader};
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

    let mut window = video_subsystem
        .window("JellyEngine", 800, 600)
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create window");

    let ctx = window.gl_create_context()?;
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

    let vert_shader = Shader::from_vert_source(&CString::new(include_str!("basic.vert")).unwrap())?;
    let frag_shader = Shader::from_frag_source(&CString::new(include_str!("basic.frag")).unwrap())?;
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader])?;
    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    // setup vertex buffer object
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes,
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // setup vertex array object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // attribute a_position in basic.vert shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    window.gl_swap_window();

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
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices
            )
        }
        window.gl_swap_window();
    }

    Ok(())
}
