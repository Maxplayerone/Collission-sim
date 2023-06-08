use std::ffi::CString;
use std::ptr;

use crate::renderer::buffer::Buffer;
use crate::renderer::shader::{Shader, ShaderError, ShaderProgram};
use crate::renderer::vertex_array::VertexArray;
use crate::set_attribute;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec3 aColor;

    out vec2 v_pos;
    out vec3 v_color;

    void main() {
        v_pos = aPos;
        v_color = aColor;
       gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 fragColor;

    uniform vec2 u_resolution;

    in vec2 v_pos;
    in vec3 v_color;

    void main() {
        vec2 uv = vec2(v_pos.x * 2.0, v_pos.y * 2.0);

        float aspect = u_resolution.x / u_resolution.y;
        uv.x *= aspect;

        fragColor.rg = uv;
        fragColor.b = 0.0;
        float distance = 1.0 - length(uv); 
        distance = step(0.0, distance);
        fragColor.rgb = vec3(distance * v_color);       
        if(distance <= 0.0){
            fragColor.a = 0.0;    
        }else{
            fragColor.a = 1.0;
        }
    }
"#;


type Pos = [f32; 2];
type Color = [f32; 3];

#[repr(C, packed)]
struct Vertex(Pos, Color);

#[rustfmt::skip]
const VERTICES: [Vertex; 4] = [
    Vertex([-0.5, -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.5, -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.5, 0.5], [0.0, 1.0, 0.0]),
    Vertex([-0.5, 0.5], [0.0, 1.0, 0.0]),
];

#[rustfmt::skip]
const INDICES: [i32; 6] = [
    0, 1, 3,
    1, 2, 3,    
];

pub struct Renderer{
    shader_program: ShaderProgram,
    vbo: Buffer,
    ibo: Buffer,
    vao: VertexArray,
}

impl Renderer{
    pub fn new() -> Result<Self, ShaderError>{
        unsafe{
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            
            let vertex_array = VertexArray::new();
            vertex_array.bind();
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
            index_buffer.set_data(&INDICES, gl::STATIC_DRAW);
            
            let pos_attrib = shader_program.get_attrib_location("aPos")?;
            set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = shader_program.get_attrib_location("aColor")?;
            set_attribute!(vertex_array, color_attrib, Vertex::1);
            
            Ok(Self{
                shader_program,
                vbo: vertex_buffer,
                ibo: index_buffer,
                vao: vertex_array,
            })
        }
    }
    
    pub fn draw(&self){
        unsafe{
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader_program.apply();
            let name = CString::new("u_resolution").unwrap();
            let location = gl::GetUniformLocation(self.shader_program.id, name.as_ptr());
            gl::Uniform2f(location, SCR_WIDTH as f32, SCR_HEIGHT as f32);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
