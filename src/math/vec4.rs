use std::fmt;

use std::ops::{ Sub, Add, Mul, Div };

use mat4::Mat4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Returns new Vec4
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn from_vec(vec: Vec<f32>) -> Vec4 {
        Vec4 {
            x: vec[0],
            y: vec[1],
            z: vec[2],
            w: vec[3],
        }
    }

    pub fn dot(&self, other: &Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn len2(&self) -> f32 {
        self.dot(self)
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn normalize(&self) -> Vec4 {
        let len = self.len();
        if len != 0.0 {
            let x = self.x / len;
            let y = self.y / len;
            let z = self.z / len;
            let w = self.w / len;
            return Vec4 { x: x, y: y, z: z, w: w };
        }
        Vec4 { x: self.x, y: self.y, z: self.z, w: self.w }
    }

}

impl Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Div for Vec4 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.z - other.w,
        }
    }
}


impl fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
