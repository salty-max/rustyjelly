use super::prelude::{Color, Material, Vertex};
use crate::gl_utilities::prelude::{AttributeInfo, GLbuffer, Shader};
use crate::math::prelude::{Matrix4x4, Vec3};

pub struct Sprite<'a> {
    pub name: String,

    pub width: f32,
    pub height: f32,

    pub origin: Vec3,

    color: Color,

    a_position_location: gl::types::GLuint,
    a_tex_coord_location: gl::types::GLuint,
    u_tint_location: gl::types::GLint,
    u_model_location: gl::types::GLint,
    u_diffuse_location: gl::types::GLint,

    buffer: GLbuffer,
    vertices: [Vertex; 6],

    shader: &'a Shader,
    material: Material<'a>,
}

impl<'a> Sprite<'a> {
    pub fn new(
        name: &str,
        shader: &'a Shader,
        material: Material<'a>,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Sprite<'a> {
        Sprite {
            name: String::from(name),
            width: match width {
                Some(w) => w,
                _ => 10.0,
            },
            height: match height {
                Some(h) => h,
                _ => 10.0,
            },
            origin: Vec3::zero(),

            color: Color::from_palette("red").unwrap(),

            a_position_location: shader.get_attribute_location("a_position"),
            a_tex_coord_location: shader.get_attribute_location("a_tex_coord"),
            u_tint_location: shader.get_uniform_location("u_tint"),
            u_model_location: shader.get_uniform_location("u_model"),
            u_diffuse_location: shader.get_uniform_location("u_diffuse"),

            buffer: GLbuffer::new(),
            vertices: [Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0); 6],

            shader,
            material,
        }
    }

    pub fn load(&mut self) {
        self.buffer.configure(
            vec![
                AttributeInfo {
                    location: self.a_position_location,
                    component_size: 3,
                },
                AttributeInfo {
                    location: self.a_tex_coord_location,
                    component_size: 2,
                },
            ],
            false,
        );

        self.calculate_vertices();
    }

    fn calculate_vertices(&mut self) {
        let min_x = -(self.width * self.origin.x);
        let max_x = self.width * (1.0 - self.origin.x);
        let min_y = -(self.height * self.origin.y);
        let max_y = self.height * (1.0 - self.origin.y);

        self.vertices[0] = Vertex::new(min_x, min_y, 0.0, 0.0, 0.0);
        self.vertices[1] = Vertex::new(min_x, max_y, 0.0, 0.0, 1.0);
        self.vertices[2] = Vertex::new(max_x, max_y, 0.0, 1.0, 1.0);
        self.vertices[3] = Vertex::new(max_x, max_y, 0.0, 1.0, 1.0);
        self.vertices[4] = Vertex::new(max_x, min_y, 0.0, 1.0, 0.0);
        self.vertices[5] = Vertex::new(min_x, min_y, 0.0, 0.0, 0.0);

        let vertices_map = self
            .vertices
            .iter()
            .flat_map(|v| {
                vec![
                    v.position.x,
                    v.position.y,
                    v.position.z,
                    v.tex_coord.x,
                    v.tex_coord.y,
                ]
            })
            .collect::<Vec<f32>>();

        self.buffer.upload(&vertices_map);
    }

    pub fn draw(&self, model: &Matrix4x4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.u_model_location, // uniform position (u_projection)
                1,
                gl::FALSE,
                model.data.as_ptr(),
            );

            gl::Uniform4f(
                self.u_tint_location, // uniform position (u_color)
                self.material.tint.r,
                self.material.tint.g,
                self.material.tint.b,
                self.material.tint.a,
            );

            self.material.texture.activate();
            gl::Uniform1i(self.u_diffuse_location, 0);
        }

        self.buffer.draw();
    }
}
