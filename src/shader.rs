use gl::types::*;
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderError{
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),
    #[error("Error while linking shaders: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
}

pub struct Shader{
    pub id: GLuint,
}

impl Shader{
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError>{
        let source_code = CString::new(source_code)?;
        let shader = Self{
            id: gl::CreateShader(shader_type),
        };
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);
        
        //error checking
        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        
        if success == 1{
            Ok(shader)
        } else{
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );
            
            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::CompilationError(log))
        }
    }
}

impl Drop for Shader{
    fn drop(&mut self){
        unsafe{
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram{
    pub id: GLuint,
}

impl ShaderProgram{
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError>{
        let program = Self{
            id: gl::CreateProgram(),
        };
        
        for shader in shaders{
            gl::AttachShader(program.id, shader.id);
        }
        gl::LinkProgram(program.id);
        
        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
        
    }
    
    pub unsafe fn apply(&self){
        gl::UseProgram(self.id);
    }
    
    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError>{
        let attrib = CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }
    
    pub unsafe fn set_uniform_2f(&self, name: &str, f1: f32, f2: f32){
        self.apply();
        let name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform2f(location, f1, f2);
    }
    
    pub unsafe fn set_uniform_mat4(&self, name: &str, mat: &[[f32; 4]; 4]){
        self.apply();
        let name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr() as *const f32);
    }
    
    pub unsafe fn set_uniform_1i(&self, name: &str, val: i32){
        self.apply();
        let name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform1i(location, val);
    }
    
}

impl Drop for ShaderProgram{
    fn drop(&mut self){
        unsafe{
            gl::DeleteProgram(self.id);
        }
    }
}