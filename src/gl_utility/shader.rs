use std::collections::HashMap;

use crate::prelude::*;

pub struct ShaderManager {
    shaders: HashMap<String, Shader>,
}

impl ShaderManager {
    pub fn init() -> ShaderManager {
        ShaderManager {
            shaders: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, vertex_source: &str, fragment_source: &str) -> &Shader {
        let mut shader = Shader {
            name: String::from(name),
            program: 0,
        };

        shader.load(
            &CString::new(vertex_source).expect("CString::new failed"),
            &CString::new(fragment_source).expect("CString::new failed"),
        );

        self.shaders.insert(String::from(name), shader);

        self.get(name)
    }

    pub fn get(&self, name: &str) -> &Shader {
        match self.shaders.get(name) {
            Some(shader) => shader,
            _ => panic!("Unable to find shader {}", name),
        }
    }
}

pub struct Shader {
    pub name: String,
    pub program: gl::types::GLuint,
}

impl Shader {
    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
    pub fn load(&mut self, vertex_source: &CString, fragment_source: &CString) {
        let vertex_shader = Shader::load_shader(vertex_source, gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::load_shader(fragment_source, gl::FRAGMENT_SHADER).unwrap();
        self.program = Shader::create_program(&[vertex_shader, fragment_shader]).unwrap();

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
    }

    fn load_shader(
        source: &CString,
        shader_type: gl::types::GLenum,
    ) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);

            let mut success: gl::types::GLint = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len: gl::types::GLint = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

                let error_msg = create_whitespace_cstring_with_len(len as usize);

                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error_msg.as_ptr() as *mut gl::types::GLchar,
                );

                return Err(error_msg.to_string_lossy().into_owned());
            }

            Ok(id)
        }
    }

    fn create_program(shaders: &[gl::types::GLuint]) -> Result<gl::types::GLuint, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, *shader);
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, *shader);
            }
        }

        Ok(program_id)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }

        println!("Destroyed shader {}", self.name);
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // Convert buffer to cstring
    unsafe { CString::from_vec_unchecked(buffer) }
}
