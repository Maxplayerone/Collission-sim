pub struct Point2{
    pub x: f32,
    pub y: f32,
}

impl Point2{
    pub fn new(x: f32, y: f32) -> Self{
        Self{
            x,
            y,
        }
    }
}

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
    
    pub fn raw(&self) -> [f32; 3]{
        [self.x, self.y, self.z]
    }
}

pub struct Vec2{
    pub x: f32,
    pub y: f32,
}

impl Vec2{
    pub fn new(x: f32, y: f32) -> Self{
        Self{
            x,
            y,
        }
    }
}

pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3{
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
    
    pub fn scale(&mut self, val: f32) -> &mut Self{
        self.mat[0][0] = self.mat[0][0] * val;
        self.mat[1][1] = self.mat[1][1] * val;
        self.mat[2][2] = self.mat[2][2] * val;
        self
    }
    
    pub fn scale_non_uniform(&mut self, vals: Point3) -> &mut Self{
        self.mat[0][0] = self.mat[0][0] * vals.x;
        self.mat[1][1] = self.mat[1][1] * vals.y;
        self.mat[2][2] = self.mat[2][2] * vals.z;
        self
    }
    
    pub fn translate(&mut self, translation: Vec3) -> &mut Self{
        /*
        self.mat[0][3] = self.mat[0][3] + translation.x;    
        self.mat[1][3] = self.mat[1][3] + translation.y;    
        self.mat[2][3] = self.mat[2][3] + translation.z;    
        */
        self.mat[3][0] = self.mat[3][0] + translation.x;
        self.mat[3][1] = self.mat[3][1] + translation.y;
        self.mat[3][2] = self.mat[3][2] + translation.z;
        self
    }
    
    pub fn ortho(&mut self, right: f32, top: f32) -> &mut Self{
        self.translate(Vec3::new(-1.0, -1.0, 0.0)).scale_non_uniform(Point3::new(2.0 / right, 2.0 / top, 1.0))
    }
    
    pub fn raw(&self) -> &[[f32; 4]; 4]{
        &self.mat
    }
}
