
#[repr(C)]
pub struct Camera{
    proj: cgmath::Matrix4<f32>,
}

impl Camera{
    pub fn new(width: f32, height: f32) -> Self{
        let proj = cgmath::ortho(0.0, width, 0.0, height, 0.1, 100.0);
        Self{
            proj
        }
    }
}

