use std::{collections::HashMap, ffi::CStr, str::FromStr};
use std::{
    ffi::CString,
    sync::atomic::{AtomicBool, Ordering},
};

/// Only one ShaderManager can be alive
// Set to false by default (not alive)
static IS_SHADER_MANAGER_ALIVE: AtomicBool = AtomicBool::new(false);

pub struct ShaderManager {
    shaders: HashMap<String, Shader>,
}

impl ShaderManager {
    pub fn init() -> ShaderManager {
        let was_alive = IS_SHADER_MANAGER_ALIVE.swap(true, Ordering::Relaxed);
        if !was_alive {
            ShaderManager {
                shaders: HashMap::new(),
            }
        } else {
            panic!("Cannot create two instance of ShaderManager")
        }
    }

    pub fn register(&mut self, name: &str, vertex_source: &str, fragment_source: &str) -> &Shader {
        let mut shader = Shader {
            name: String::from(name),
            program: 0,
            attributes: HashMap::new(),
            uniforms: HashMap::new(),
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
    attributes: HashMap<String, gl::types::GLuint>,
    uniforms: HashMap<String, gl::types::GLint>,
}

impl Shader {
    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn get_attribute_location(&self, name: &str) -> gl::types::GLuint {
        match self.attributes.get(name) {
            Some(&attribute) => attribute,
            _ => panic!("Unable to find attribute {} in shader {}", name, self.name),
        }
    }
    pub fn get_uniform_location(&self, name: &str) -> gl::types::GLint {
        match self.uniforms.get(name) {
            Some(&uniform) => uniform,
            _ => panic!("Unable to find uniform {} in shader {}", name, self.name),
        }
    }

    pub fn load(&mut self, vertex_source: &CString, fragment_source: &CString) {
        let vertex_shader = Shader::load_shader(vertex_source, gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::load_shader(fragment_source, gl::FRAGMENT_SHADER).unwrap();

        self.program = Shader::create_program(&[vertex_shader, fragment_shader]).unwrap();

        self.detect_attributes();
        self.detect_uniforms();

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

    fn detect_attributes(&mut self) {
        unsafe {
            let mut attributes_number: gl::types::GLint = 0;
            gl::GetProgramiv(self.program, gl::ACTIVE_ATTRIBUTES, &mut attributes_number);

            for i in 0..attributes_number {
                let mut size: gl::types::GLint = 0; // variable size
                let mut var_type: gl::types::GLenum = 0; // variable type (e.g. float, vec3, vec4, mat4)
                const BUF_SIZE: usize = 16; // maximum name length
                let name = [0; BUF_SIZE];
                let mut length: gl::types::GLsizei = 0; // name length

                gl::GetActiveAttrib(
                    self.program,
                    i as gl::types::GLuint,
                    BUF_SIZE as gl::types::GLint,
                    &mut length,
                    &mut size,
                    &mut var_type,
                    name.as_ptr() as *mut gl::types::GLchar,
                );

                if length == 0 {
                    break;
                }

                let location =
                    gl::GetAttribLocation(self.program, name.as_ptr() as *mut gl::types::GLchar);

                self.attributes.insert(
                    String::from_str(CStr::from_ptr(name.as_ptr()).to_str().unwrap()).unwrap(),
                    location as gl::types::GLuint,
                );
            }
        }
    }

    fn detect_uniforms(&mut self) {
        unsafe {
            let mut uniforms_number: gl::types::GLint = 0;
            gl::GetProgramiv(self.program, gl::ACTIVE_UNIFORMS, &mut uniforms_number);

            for i in 0..uniforms_number {
                let mut size: gl::types::GLint = 0; // variable size
                let mut var_type: gl::types::GLenum = 0; // variable type (e.g. float, vec3, vec4, mat4)
                const BUF_SIZE: usize = 16; // maximum name length
                let name = [0; BUF_SIZE];
                let mut length: gl::types::GLsizei = 0; // name length

                gl::GetActiveUniform(
                    self.program,
                    i as gl::types::GLuint,
                    BUF_SIZE as gl::types::GLint,
                    &mut length,
                    &mut size,
                    &mut var_type,
                    name.as_ptr() as *mut gl::types::GLchar,
                );

                if length == 0 {
                    break;
                }

                let location =
                    gl::GetUniformLocation(self.program, name.as_ptr() as *mut gl::types::GLchar);

                self.uniforms.insert(
                    String::from_str(CStr::from_ptr(name.as_ptr()).to_str().unwrap()).unwrap(),
                    location,
                );
            }
        }
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
