extern crate gl;
extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use crate::graphics::prelude::Sprite;
use crate::math::prelude::{Matrix4x4, Transform};
use crate::{gl_utilities::prelude::ShaderManager, graphics::prelude::Color};

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

pub struct Config {
    pub title: String,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub screen_width: u32,
    pub screen_height: u32,
}
pub fn start(config: Config) -> Result<(), String> {
    println!("Hello, JellyEngine!");

    // Init window
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);

    let window = video_subsystem
        .window(
            config.title.as_ref(),
            config.screen_width,
            config.screen_height,
        )
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

    let projection = Matrix4x4::orthographic(
        0.0,
        config.virtual_width as f32,
        0.0,
        config.virtual_height as f32,
        -100.0,
        100.0,
    );

    let mut shader_manager = ShaderManager::init();
    let basic_shader = shader_manager.register(
        "basic",
        include_str!("basic.vert"),
        include_str!("basic.frag"),
    );

    let u_projection_location = basic_shader.get_uniform_location("u_projection");

    let mut sprite = Sprite::new("test", &basic_shader, Some(8.0), Some(8.0));
    sprite.load();

    let mut transform = Transform::new();
    transform.position.x = 8.0;
    transform.position.y = 8.0;

    basic_shader.use_shader();

    unsafe {
        let (r, g, b, a) = Color::black().as_tuple();
        gl::ClearColor(r, g, b, a);
    }

    resize(None, &config);

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

            gl::UniformMatrix4fv(
                u_projection_location, // uniform position (u_projection)
                1,
                gl::FALSE,
                projection.data.as_ptr(),
            );

            sprite.draw(&transform.get_transformation_matrix());
        }
        window.gl_swap_window();
    }

    Ok(())
}

fn resize(new_size: Option<(i32, i32)>, config: &Config) {
    let target_aspect_ratio = config.virtual_width as f32 / config.virtual_height as f32;
    let width: i32;
    let height: i32;

    match new_size {
        Some(new_size) => {
            width = new_size.0;
            height = new_size.1;
        }
        None => {
            width = config.screen_width as i32;
            height = config.screen_height as i32;
        }
    }

    let mut calculated_height = (width as f32 / target_aspect_ratio) as i32;
    let mut calculated_width = width;

    if calculated_height > height {
        calculated_height = height;
        calculated_width = (calculated_height as f32 * target_aspect_ratio) as i32;
    }

    let vp_x = (width / 2) - (calculated_width / 2);
    let vp_y = (height / 2) - (calculated_height / 2);

    unsafe {
        let (r, g, b, a) = Color::white().as_tuple();
        gl::ClearColor(r, g, b, a);
        gl::Viewport(vp_x, vp_y, calculated_width, calculated_height);
        gl::Scissor(vp_x, vp_y, calculated_width, calculated_height);
        gl::Enable(gl::SCISSOR_TEST)
    }
}
