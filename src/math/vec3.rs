use std::fmt;

use std::ops::{ Sub, Add, Mul, Div };

use mat4::Mat4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Returns new vec3
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn from_vec(vec: Vec<f32>) -> Vec3 {
        Vec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        }
    }

    pub fn multiply_mat(self, transform: Mat4) -> Vec3 {
        let x = transform.elements[0 + 0 * 4] * self.x + transform.elements[0 + 1 * 4] * self.y + transform.elements[0 + 2 * 4] * self.z + transform.elements[0 + 3 * 4];
        let y = transform.elements[1 + 0 * 4] * self.x + transform.elements[1 + 1 * 4] * self.y + transform.elements[2 + 2 * 4] * self.z + transform.elements[1 + 3 * 4];
        let z = transform.elements[2 + 0 * 4] * self.x + transform.elements[2 + 1 * 4] * self.y + transform.elements[1 + 2 * 4] * self.z + transform.elements[2 + 3 * 4];
        Vec3 { x: x, y: y, z: z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len2(&self) -> f32 {
        self.dot(self)
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        let x = a.y * b.z - a.z * b.y;
        let y = a.z * b.x - a.x * b.z;
        let z = a.x * b.y - a.y * b.x;
        Vec3 { x: x, y: y, z: z }
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.len();
        if len != 0.0 {
            let x = self.x / len;
            let y = self.y / len;
            let z = self.z / len;
            return Vec3 { x: x, y: y, z: z };
        }
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
