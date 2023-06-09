pub struct Point3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3{
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x,
            y,
            z,
        }
    }
}

pub struct Mat4{
    mat: [[f32; 4]; 4],
}

impl Mat4{
    pub fn identity() -> Self{
        Self{
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }
    
    pub fn scale(&mut self, val: f32){
        self.mat[0][0] = self.mat[0][0] * val;
        self.mat[1][1] = self.mat[1][1] * val;
        self.mat[2][2] = self.mat[2][2] * val;
    }
    
    pub fn raw(&self) -> &[[f32; 4]; 4]{
        &self.mat
    }
}
