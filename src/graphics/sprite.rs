use crate::prelude::{AttributeInfo, Color, GLbuffer, Shader, Vec3, Vertex};

pub struct Sprite<'a> {
    pub name: String,

    pub width: f32,
    pub height: f32,

    pub origin: Vec3,

    color: Color,
    u_color_location: gl::types::GLint,

    buffer: GLbuffer,
    vertices: [Vertex; 6],

    shader: &'a Shader,
}

impl<'a> Sprite<'a> {
    pub fn new(name: &str, shader: &'a Shader, width: f32, height: f32) -> Sprite<'a> {
        Sprite {
            name: String::from(name),
            width,
            height,
            origin: Vec3::zero(),
            color: Color::from_palette("red").unwrap(),
            u_color_location: shader.get_uniform_location("u_color"),
            buffer: GLbuffer::new(),
            vertices: [Vertex::new(0.0, 0.0, 0.0); 6],

            shader,
        }
    }

    pub fn load(&mut self) {
        let a_position_location = self.shader.get_attribute_location("a_position");

        self.buffer.configure(
            vec![AttributeInfo {
                location: a_position_location,
                component_size: 3,
            }],
            false,
        );

        self.calculate_vertices();
    }

    fn calculate_vertices(&mut self) {
        let min_x = -(self.width * self.origin.x);
        let max_x = self.width * (1.0 - self.origin.x);
        let min_y = -(self.height * self.origin.y);
        let max_y = self.height * (1.0 - self.origin.y);

        self.vertices[0] = Vertex::new(min_x, min_y, 0.0);
        self.vertices[1] = Vertex::new(min_x, max_y, 0.0);
        self.vertices[2] = Vertex::new(max_x, max_y, 0.0);
        self.vertices[3] = Vertex::new(max_x, max_y, 0.0);
        self.vertices[4] = Vertex::new(max_x, min_y, 0.0);
        self.vertices[5] = Vertex::new(min_x, min_y, 0.0);

        let vertices_map = self
            .vertices
            .iter()
            .flat_map(|v| vec![v.position.x, v.position.y, v.position.z])
            .collect::<Vec<f32>>();

        self.buffer.upload(&vertices_map);
    }

    pub fn draw(&self) {
        unsafe {
            gl::Uniform4f(
                self.u_color_location, // uniform position (u_color)
                self.color.r,
                self.color.g,
                self.color.b,
                self.color.a,
            );
        }

        self.buffer.draw();
    }
}
