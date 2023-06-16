use std::ptr;
use std::path::Path;

use crate::buffer::Buffer;
use crate::shader::{Shader, ShaderError, ShaderProgram};
use crate::vertex_array::VertexArray;
use crate::set_attribute;
use crate::texture::Texture;
use crate::math;
/*
const VERTEX_QUAD: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec3 a_color;

    out vec3 f_color;

    uniform mat4 u_world_mat;

    void main() {
        f_color = a_color;
        gl_Position = u_world_mat * vec4(a_pos.x, a_pos.y, 0.0, 1.0);
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

const VERTEX_TEXTURE: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec2 a_tex_coords;

    out vec2 f_tex_coords;

    uniform mat4 u_world_mat;

    void main() {
        f_tex_coords = a_tex_coords;
        gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
    }
"#;

const FRAGMENT_TEXTURE: &str = r#" 
    #version 330 core 
    out vec4 fragColor;

    in vec2 f_tex_coords;
    
    uniform sampler2D texture0;

    void main() {
        fragColor = texture(texture0, f_tex_coords);
    }
"#;
*/
const VERTEX_SHADER: &str = r#"
    #version 330 core
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec3 a_color;
    layout(location = 2) in vec2 a_tex_coords;
    layout(location = 3) in float a_tex_index;

    out vec2 f_tex_coords;
    out vec3 f_color;
    out float f_tex_index;

    uniform mat4 u_world_mat;

    void main() {
        f_tex_coords = a_tex_coords;
        f_color = a_color;
        f_tex_index = a_tex_index;

        gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#" 
    #version 330 core 
    out vec4 fragColor;

    in vec2 f_tex_coords;
    in vec3 f_color;
    in float t_tex_index;
    
    uniform sampler2D textures[16];

    void main() {
        fragColor = texture(texture0, f_tex_coords);
    }
"#;

type Pos = [f32; 2];
type Color = [f32; 3];
type TexCoords = [f32; 2];
type TexIndex = f32;

#[repr(C, packed)]
struct Vertex(Pos, Color, TexCoords, TexIndex);

const BATCH_SIZE: u16 = 500;

struct RenderingTools{
    program: ShaderProgram,
    vao: VertexArray,
    vbo: Buffer,
    ibo: Buffer,
}
pub struct Renderer{
    world_mat: math::Mat4,
    tools: RenderingTools,
    vertices: Vec<f32>,
}

impl Renderer{
    pub fn new(world_mat: math::Mat4) -> Self{
        Self{
            world_mat,
            vertices: Vec::with_capacity(std::mem::size_of(Vertex) * BATCH_SIZE),
            tools: None,
        }
    }
    
    pub fn setup_quad_info(&mut self, quad_position: math::Point3, color: math::Point3) -> Result<(), ShaderError>{
        unsafe{
            let vertex_shader = Shader::new(VERTEX_QUAD, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_QUAD, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            
            let color: [f32; 3] = color.raw();
            let vertices: [Vertex; 4] = [
                Vertex([quad_position.x, quad_position.y], color),
                Vertex([quad_position.x + 1.0, quad_position.y], color),
                Vertex([quad_position.x + 1.0, quad_position.y + 1.0], color),
                Vertex([quad_position.x, quad_position.y + 1.0], color),
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
    //NOTE: right now I'm hard-coding texture coordinates cuz I don't see a reason as not to    
    pub fn setup_texture_info(&mut self, quad_position: math::Point3, filepath: &str) -> Result<(), ShaderError>{
        unsafe{
            let vertex_shader = Shader::new(VERTEX_TEXTURE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_TEXTURE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            
            //tex_coords need to be flipped
            let vertices: [VertexTexture; 4] = [
                VertexTexture([quad_position.x, quad_position.y], [1.0, 1.0]),
                VertexTexture([quad_position.x + 1.0, quad_position.y], [0.0, 1.0]),
                VertexTexture([quad_position.x, quad_position.y + 1.0], [1.0, 0.0]),
                VertexTexture([quad_position.x + 1.0, quad_position.y + 1.0], [0.0, 0.0]),
            ];
            
            let indices: [i32; 6] = [
                2, 0, 1,
                2, 3, 1,
            ];
            
            let vao = VertexArray::new();
            vao.bind();
            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            vbo.set_data(&vertices, gl::STATIC_DRAW);
            ibo.set_data(&indices, gl::STATIC_DRAW);
            
            let pos_attrib = program.get_attrib_location("a_pos")?;
            set_attribute!(vao, pos_attrib, VertexTexture::0);
            let color_attrib = program.get_attrib_location("a_tex_coords")?;
            set_attribute!(vao, color_attrib, VertexTexture::1);
            
            let texture = Texture::new();
            texture.set_wrapping(gl::REPEAT);
            texture.set_filtering(gl::LINEAR);
            texture.load(&Path::new(filepath)).unwrap();
            program.set_uniform_1i("texture0", 0);
            
            let texture_info = TextureInfo{
                program,
                vao,
                vbo,
                ibo,
                texture,
            };
            self.texture_info = Some(texture_info);
            Ok(())
        }
    }
    
    pub fn setup_texture_color_info(&mut self, quad_position: math::Point3, color: math::Point3, filepath: &str) -> Result<(), ShaderError>{
        unsafe{
            let vertex_shader = Shader::new(VERTEX_TEXTURE_COLOR, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_TEXTURE_COLOR, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            
            //tex_coords need to be flipped
            let color = color.raw();
            let vertices: [VertexTextureColor; 4] = [
                VertexTextureColor([quad_position.x, quad_position.y], color, [1.0, 1.0]),
                VertexTextureColor([quad_position.x + 1.0, quad_position.y], color, [0.0, 1.0]),
                VertexTextureColor([quad_position.x, quad_position.y + 1.0], color, [1.0, 0.0]),
                VertexTextureColor([quad_position.x + 1.0, quad_position.y + 1.0], color, [0.0, 0.0]),
            ];
            
            let indices: [i32; 6] = [
                2, 0, 1,
                2, 3, 1,
            ];
            
            let vao = VertexArray::new();
            vao.bind();
            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            vbo.set_data(&vertices, gl::STATIC_DRAW);
            ibo.set_data(&indices, gl::STATIC_DRAW);
            
            let pos_attrib = program.get_attrib_location("a_pos")?;
            set_attribute!(vao, pos_attrib, VertexTextureColor::0);
            let color_attrib = program.get_attrib_location("a_color")?;
            set_attribute!(vao, color_attrib, VertexTextureColor::1);
            let tex_coords_attrib = program.get_attrib_location("a_tex_coords")?;
            set_attribute!(vao, tex_coords_attrib, VertexTextureColor::2);
            
            let texture = Texture::new();
            texture.set_wrapping(gl::REPEAT);
            texture.set_filtering(gl::LINEAR);
            texture.load(&Path::new(filepath)).unwrap();
            program.set_uniform_1i("texture0", 0);
            
            let texture_info = TextureInfo{
                program,
                vao,
                vbo,
                ibo,
                texture,
            };
            self.texture_color_info = Some(texture_info);
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
    
    pub fn draw_texture(&self){
        match &self.texture_info{
            None => {
                println!("texture info is not setup yet!");
                assert!(false);
            },
            Some(info) => {
                unsafe{
                    info.texture.activate(gl::TEXTURE0);
                    info.program.apply();
                    info.program.set_uniform_mat4("u_world_mat", self.world_mat.raw());
                    info.vao.bind();
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                }
            }
        }
    }
    
    pub fn draw_texture_color(&self){
        match &self.texture_color_info{
            None => {
                println!("texture info is not setup yet!");
                assert!(false);
            },
            Some(info) => {
                unsafe{
                    info.texture.activate(gl::TEXTURE0);
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
