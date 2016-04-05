use std::fmt;

use mat4::Mat4;

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

    pub fn add(&self, other: &Vec3) -> Vec3 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vec3 {x: x , y: y, z: z}
    }

    pub fn sub(&self, other: &Vec3) -> Vec3{
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vec3 {x: x , y: y, z: z}
    }

    pub fn multiply(&self, other: &Vec3) -> Vec3 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        Vec3 {x: x, y: y, z: z}
    }

    // pub fn multiply_mat(&mut self, transform: &Mat4) {
    //     let mut x = transform.elements[0 + 0 * 4] * self.x + transform.elements[0 + 1 * 4] * self.y + transform.elements[0 + 2 * 4] * self.z + transform.elements[0 + 3 * 4];
    //     let mut y = transform.elements[1 + 0 * 4] * self.x + transform.elements[1 + 1 * 4] * self.y + transform.elements[2 + 2 * 4] * self.z + transform.elements[1 + 3 * 4];
    //     let mut z = transform.elements[2 + 0 * 4] * self.x + transform.elements[2 + 1 * 4] * self.y + transform.elements[1 + 2 * 4] * self.z + transform.elements[2 + 3 * 4];
    //     self.x = x;
    //     self.y = y;
    //     self.z = z;
    // }

    pub fn divide(&self, other: &Vec3) -> Vec3 {
        let x = self.x / other.x;
        let y = self.y / other.y;
        let z = self.z / other.z;
        Vec3 {x: x, y: y, z: z}
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

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
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


impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
