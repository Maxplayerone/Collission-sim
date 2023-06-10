use std::ptr;

use crate::buffer::Buffer;
use crate::shader::{Shader, ShaderError, ShaderProgram};
use crate::vertex_array::VertexArray;
use crate::set_attribute;
use crate::math;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_QUAD: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec3 a_color;

    out vec3 f_color;

    uniform mat4 u_world_mat;

    void main() {
        f_color = a_color;
        vec4 calc_pos = u_world_mat * vec4(a_pos.x, a_pos.y, 0.0, 1.0);
        gl_Position = calc_pos;
    }
"#;

const FRAGMENT_QUAD: &str = r#"
    #version 330 core
    out vec4 fragColor;

    in vec3 f_color;

    void main() {
        fragColor = vec4(f_color, 1.0);
    }
"#;


type Pos = [f32; 2];
type Color = [f32; 3];

#[repr(C, packed)]
struct Vertex(Pos, Color);

struct QuadInfo{
    program: ShaderProgram,
    vao: VertexArray,
    vbo: Buffer,
    ibo: Buffer,
}

pub struct Renderer{
    world_mat: math::Mat4,
    quad_info: Option<QuadInfo>
}

impl Renderer{
    pub fn new(world_mat: math::Mat4) -> Self{
        Self{
            world_mat,
            quad_info: None,
        }
    }
    
    pub fn setup_quad_info(&mut self, quad_position: math::Point3, size: math::Point3, color: math::Point3) -> Result<(), ShaderError>{
        unsafe{
            let vertex_shader = Shader::new(VERTEX_QUAD, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_QUAD, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            
            let color: [f32; 3] = color.raw();
            let vertices: [Vertex; 4] = [
                Vertex([quad_position.x, quad_position.y], color),
                Vertex([quad_position.x + size.x, quad_position.y], color),
                Vertex([quad_position.x + size.x, quad_position.y + size.y], color),
                Vertex([quad_position.x, quad_position.y + size.y], color),
            ];
            
            let indices: [i32; 6] = [
                0, 1, 3,
                1, 2, 3,
            ];
            
            let vao = VertexArray::new();
            vao.bind();
            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            vbo.set_data(&vertices, gl::STATIC_DRAW);
            ibo.set_data(&indices, gl::STATIC_DRAW);
            
            let pos_attrib = program.get_attrib_location("a_pos")?;
            set_attribute!(vao, pos_attrib, Vertex::0);
            let color_attrib = program.get_attrib_location("a_color")?;
            set_attribute!(vao, color_attrib, Vertex::1);
            
            let quad_info = QuadInfo{
                program,
                vao,
                vbo,
                ibo,
            };
            self.quad_info = Some(quad_info);
            Ok(())
        }
    }
    
    pub fn draw_quad(&self){
        match &self.quad_info{
            None => {
                println!("Quad info is not setup yet!");
                assert!(false);
            },
            Some(info) => {
                unsafe{
                    info.program.apply();
                    info.program.set_uniform_mat4("u_world_mat", self.world_mat.raw());
                    info.vao.bind();
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                }
            }
        }
    }
    
    pub fn clear_surface(&self, color: math::Point3){
        unsafe{
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
