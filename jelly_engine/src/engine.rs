use crate::core::{Scene, System, Transaction, World};

#[derive(Debug, Default)]
pub struct Engine {
    world: World,
    systems: Vec<Box<dyn System>>,
}

impl Engine {
    pub fn with_system<S: System>(mut self, system: S) -> Self {
        self.systems.push(Box::new(system));

        self
    }

    pub fn run<S: Scene>(mut self, mut scene: S) {
        println!("Engine started...");

        println!("Init systems");
        for s in self.systems.iter_mut() {
            s.init(&mut self.world);
        }

        scene.on_enter(&mut self.world);

        'main_loop: loop {
            for s in self.systems.iter_mut() {
                s.run(&mut self.world);
            }

            if let Transaction::Quit = scene.update(&mut self.world) {
                println!("Quit transaction received");
                break 'main_loop;
            }
        }

        scene.on_exit(&mut self.world);

        println!("Dispose systems");
        for s in self.systems.iter_mut() {
            s.dispose(&mut self.world);
        }

        println!("Engine stopped");
    }
}

// extern crate gl;
// extern crate sdl2;

// use sdl2::{
//     event::Event,
//     keyboard::Keycode,
//     video::{DisplayMode, FullscreenType, GLProfile, Window},
//     VideoSubsystem,
// };

// use crate::graphics::prelude::{Material, Sprite, Texture};
// use crate::math::prelude::{Matrix4x4, Transform};
// use crate::{gl_utilities::prelude::ShaderManager, graphics::prelude::Color};

// // Crash on macOS
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

// pub struct Config {
//     pub title: String,
//     pub fullscreen: bool,
//     pub virtual_width: u32,
//     pub virtual_height: u32,
//     pub screen_width: u32,
//     pub screen_height: u32,
// }
// pub fn start(config: Config) -> Result<(), String> {
//     println!("Hello, JellyEngine!");

//     // Init window
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video().unwrap();

//     let gl_attr = video_subsystem.gl_attr();
//     gl_attr.set_context_profile(GLProfile::Core);

//     if cfg!(target_os = "macos") {
//         gl_attr.set_context_version(4, 1);
//     } else {
//         gl_attr.set_context_version(4, 6);
//     }
//     gl_attr.set_double_buffer(true);

//     let mut window = video_subsystem
//         .window(
//             config.title.as_ref(),
//             config.screen_width,
//             config.screen_height,
//         )
//         .opengl()
//         .allow_highdpi()
//         .build()
//         .expect("Failed to create window");

//     if config.fullscreen {
//         let display_mode = get_display_mode(&video_subsystem, &config);
//         window.set_display_mode(display_mode)?;
//         window.set_fullscreen(FullscreenType::True)?;
//     }

//     let _ctx = window.gl_create_context()?;
//     gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

//     unsafe {
//         if !cfg!(target_os = "macos") {
//             gl::Enable(gl::DEBUG_OUTPUT);
//             gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());
//         }
//         gl::Enable(gl::BLEND);
//         gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
//     }

//     println!(
//         "Pixel format of the window's GL context: {:?}",
//         window.window_pixel_format()
//     );
//     println!(
//         "OpenGL Profile: {:?} - OpenGL version: {:?}",
//         gl_attr.context_profile(),
//         gl_attr.context_version(),
//     );

//     let projection = Matrix4x4::orthographic(
//         0.0,
//         config.virtual_width as f32,
//         0.0,
//         config.virtual_height as f32,
//         -100.0,
//         100.0,
//     );

//     let mut shader_manager = ShaderManager::init();
//     let basic_shader = shader_manager.register(
//         "basic",
//         include_str!("basic.vert"),
//         include_str!("basic.frag"),
//     );

//     let u_projection_location = basic_shader.get_uniform_location("u_projection");

//     let texture1 = Texture::new("dude_single.png");

//     let mut sprite = Sprite::new(
//         "test",
//         &basic_shader,
//         Material::new(Color::white(), &texture1),
//         None,
//         None,
//     );
//     sprite.load();

//     let mut transform = Transform::new();
//     transform.position.x = 8.0;
//     transform.position.y = 8.0;

//     basic_shader.use_shader();

//     resize(&window, (config.virtual_width, config.virtual_height));

//     let mut event_pump = sdl_context.event_pump()?;
//     'main_loop: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => {
//                     break 'main_loop;
//                 }
//                 Event::KeyUp {
//                     keycode: Some(keycode),
//                     keymod,
//                     ..
//                 } => match (keycode, keymod) {
//                     (Keycode::R, _) => unsafe {
//                         let (r, g, b, a) = Color::from_palette("red").unwrap().as_tuple();
//                         gl::ClearColor(r, g, b, a);
//                     },
//                     (Keycode::G, _) => unsafe {
//                         let (r, g, b, a) = Color::from_palette("green").unwrap().as_tuple();
//                         gl::ClearColor(r, g, b, a);
//                     },
//                     (Keycode::B, _) => unsafe {
//                         let (r, g, b, a) = Color::from_palette("blue").unwrap().as_tuple();
//                         gl::ClearColor(r, g, b, a);
//                     },
//                     _ => (),
//                 },
//                 _ => (),
//             }
//         }

//         unsafe {
//             gl::Disable(gl::SCISSOR_TEST);

//             let (r, g, b, a) = Color::black().as_tuple();
//             gl::ClearColor(r, g, b, a);

//             gl::Clear(gl::COLOR_BUFFER_BIT);
//             gl::Enable(gl::SCISSOR_TEST);

//             let (r, g, b, a) = Color::white().as_tuple();
//             gl::ClearColor(r, g, b, a);

//             gl::Clear(gl::COLOR_BUFFER_BIT);

//             gl::UniformMatrix4fv(
//                 u_projection_location, // uniform position (u_projection)
//                 1,
//                 gl::FALSE,
//                 projection.data.as_ptr(),
//             );

//             sprite.draw(&transform.get_transformation_matrix());
//         }
//         window.gl_swap_window();
//     }

//     Ok(())
// }

// fn resize(window: &Window, virtual_size: (u32, u32)) {
//     let target_aspect_ratio = virtual_size.0 as f32 / virtual_size.1 as f32;

//     let size = window.drawable_size();
//     let width = size.0 as i32;
//     let height = size.1 as i32;

//     let mut calculated_height = (width as f32 / target_aspect_ratio) as i32;
//     let mut calculated_width = width;

//     if calculated_height > height {
//         calculated_height = height;
//         calculated_width = (calculated_height as f32 * target_aspect_ratio) as i32;
//     }

//     let vp_x = (width / 2) - (calculated_width / 2);
//     let vp_y = (height / 2) - (calculated_height / 2);

//     unsafe {
//         gl::Viewport(vp_x, vp_y, calculated_width, calculated_height);
//         gl::Scissor(vp_x, vp_y, calculated_width, calculated_height);
//     }
// }

// fn get_display_mode(video_subsystem: &VideoSubsystem, config: &Config) -> DisplayMode {
//     for i in 0..video_subsystem.num_display_modes(0).unwrap() {
//         let display_mode = video_subsystem.display_mode(0, i).unwrap();
//         if display_mode.w == config.screen_width as i32
//             && display_mode.h == config.screen_height as i32
//         {
//             return display_mode;
//         }
//     }

//     panic!(
//         "No display mode available for aspect ratio {}x{}",
//         config.screen_width, config.screen_height
//     );
// }
