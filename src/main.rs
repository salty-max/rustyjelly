extern crate gl;
extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

extern "system" fn dbg_callback(
    source: gl::types::GLenum,
    etype: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _msg_length: gl::types::GLsizei,
    msg: *const gl::types::GLchar,
    _user_data: *mut std::ffi::c_void,
) {
    unsafe {
        println!(
            "dbg_callback {:#X} {:#X} {:#X} {:?}",
            source,
            etype,
            severity,
            std::ffi::CStr::from_ptr(msg),
        );
    }
}

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
        }
        window.gl_swap_window();
    }

    Ok(())
}
