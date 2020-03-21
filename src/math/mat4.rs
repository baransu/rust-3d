use std::fmt;

use std::ops::Mul;
use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Mat4 {
    pub elements: [f32; 16],
}

impl Mat4 {
    /// Retuns new identity matrix
    pub fn new(value: f32) -> Mat4 {
        let mut elements: [f32; 16] = [0.0; 16];
        elements[0 + 0 * 4] = value;
        elements[1 + 1 * 4] = value;
        elements[2 + 2 * 4] = value;
        elements[3 + 3 * 4] = value;
        Mat4 { elements: elements }
    }

    /// Returns new perspective projection matrix
    pub fn from_perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
        let mut elements: [f32; 16] = [0.0; 16];

        let t = (0.5 * fov).to_radians().tan();
        let q = 1.0 / t;
        let a = q / aspect_ratio;

        let b = (far + near) / (near - far);
        let c = (2.0 * far * near) / (near - far);

        // col + row * 4
        elements[0 + 0 * 4] = a;
        elements[1 + 1 * 4] = q;
        elements[2 + 2 * 4] = b;
        elements[2 + 3 * 4] = -1.0;
        elements[3 + 2 * 4] = c;

        Mat4 { elements: elements }.transpose()
    }

    /// Returns new orthographic projection matrix
    pub fn from_ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        let mut elements: [f32; 16] = [0.0; 16];

        // col + row * 4
        elements[0 + 0 * 4] = 2.0 / (right - left);

        elements[1 + 1 * 4] = 2.0 / (top - bottom);

        elements[2 + 2 * 4] = 2.0 / (near - far);

        elements[3 + 0 * 4] = (left + right) / (left - right);
        elements[3 + 1 * 4] = (bottom + top) / (bottom - top);
        elements[3 + 2 * 4] = (far + near) / (far - near);

        Mat4 { elements: elements }
    }

    pub fn from_translation(translation: &Vec3) -> Mat4 {
        let mut mat = Mat4::new(1.0);

        // col + row * 4
        mat.elements[0 + 3 * 4] = translation.x;
        mat.elements[1 + 3 * 4] = translation.y;
        mat.elements[2 + 3 * 4] = translation.z;

        // mat
        Mat4 {
            elements: mat.elements,
        }
    }

    pub fn from_rotation(v: &Vec3) -> Mat4 {
        let mut mat = Mat4::new(1.0);

        // Roll = rotation about x axis
        let sx = v.x.to_radians().sin();
        let cx = v.x.to_radians().cos();

        // Yaw = rotation about y axis
        let sy = v.y.to_radians().sin();
        let cy = v.y.to_radians().cos();

        let sz = v.z.to_radians().sin();
        let cz = v.z.to_radians().cos();

        // col + row * 4
        mat.elements[0 + 0 * 4] = cy * cz;
        mat.elements[1 + 0 * 4] = cy * sz;
        mat.elements[2 + 0 * 4] = -sy;

        mat.elements[0 + 1 * 4] = -cx * sz + sx * sy * cz;
        mat.elements[1 + 1 * 4] = cx * cz + sx * sy * sz;
        mat.elements[2 + 1 * 4] = sx * cy;

        mat.elements[0 + 2 * 4] = sx * sz + cx * sy * cz;
        mat.elements[1 + 2 * 4] = -sx * cz + cx * sy * sz;
        mat.elements[2 + 2 * 4] = cx * cy;

        mat
    }

    pub fn from_scale(scale: &Vec3) -> Mat4 {
        let mut mat = Mat4::new(1.0);

        // col + row * 4
        mat.elements[0 + 0 * 4] = scale.x;
        mat.elements[1 + 1 * 4] = scale.y;
        mat.elements[2 + 2 * 4] = scale.z;

        mat
    }

    pub fn from_look_at(camera: Vec3, object: Vec3, up: Vec3) -> Mat4 {
        let mut mat = Mat4::new(1.0);

        let f = (object - camera).normalize();

        let s = Vec3::cross(f, up.normalize());

        let u = Vec3::cross(s, f);

        // col + row * 4
        mat.elements[0 + 0 * 4] = s.x;
        mat.elements[0 + 1 * 4] = s.y;
        mat.elements[0 + 2 * 4] = s.z;

        mat.elements[1 + 0 * 4] = u.x;
        mat.elements[1 + 1 * 4] = u.y;
        mat.elements[1 + 2 * 4] = u.z;

        mat.elements[2 + 0 * 4] = -f.x;
        mat.elements[2 + 1 * 4] = -f.y;
        mat.elements[2 + 2 * 4] = -f.z;

        let m = Mat4::from_translation(&Vec3::new(-camera.x, -camera.y, -camera.z));
        m * mat
    }

    pub fn transpose(&self) -> Mat4 {
        let mut mat = Mat4::new(1.0);

        mat.elements = self.elements;

        //col + row * 4
        mat.elements[0 + 0 * 4] = self.elements[0 + 0 * 4];
        mat.elements[0 + 2 * 4] = self.elements[2 + 0 * 4];
        mat.elements[0 + 3 * 4] = self.elements[3 + 0 * 4];

        mat.elements[1 + 0 * 4] = self.elements[0 + 1 * 4];
        mat.elements[1 + 2 * 4] = self.elements[2 + 1 * 4];
        mat.elements[1 + 3 * 4] = self.elements[3 + 1 * 4];

        mat.elements[2 + 0 * 4] = self.elements[0 + 2 * 4];
        mat.elements[2 + 1 * 4] = self.elements[1 + 2 * 4];
        mat.elements[2 + 3 * 4] = self.elements[3 + 2 * 4];

        mat.elements[3 + 0 * 4] = self.elements[0 + 3 * 4];
        mat.elements[3 + 1 * 4] = self.elements[1 + 3 * 4];
        mat.elements[3 + 2 * 4] = self.elements[2 + 3 * 4];

        mat
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f32 {
        &self.elements[0]
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut data: [f32; 16] = [0.0; 16];

        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for e in 0..4 {
                    sum += self.elements[e + row * 4] * other.elements[col + e * 4];
                }
                data[col + row * 4] = sum;
            }
        }

        Mat4 { elements: data }
    }
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[ {}, {}, {}, {}\n  {}, {}, {}, {}\n  {}, {}, {}, {}\n  {}, {}, {}, {} ]",
            self.elements[0],
            self.elements[1],
            self.elements[2],
            self.elements[3],
            self.elements[4],
            self.elements[5],
            self.elements[6],
            self.elements[7],
            self.elements[8],
            self.elements[9],
            self.elements[10],
            self.elements[11],
            self.elements[12],
            self.elements[13],
            self.elements[14],
            self.elements[15]
        )
    }
}
