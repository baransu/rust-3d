use std::fmt;

use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Mat4 {
    pub elements: [f32; 16],
}

impl Mat4 {
    /// Retuns new identity matrix
    pub fn new_identity() -> Mat4 {
        let mut elements: [f32; 16] = [0.0; 16];
        elements[0 + 0 * 4] = 1.0;
		elements[1 + 1 * 4] = 1.0;
		elements[2 + 2 * 4] = 1.0;
		elements[3 + 3 * 4] = 1.0;
        Mat4 { elements: elements }
    }

    /// Returns new perspective projection matrix
    pub fn new_perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
        let mut elements: [f32; 16] = [0.0; 16];

        let q = 1.0/ (0.5 * fov).to_radians().tan();
        let a = q / aspect_ratio;

        let b = (near + far) / (near - far);
        let c = (2.0 * near * far) / (near - far);

        // col + row * 4
        elements[0 + 0 * 4] = a;
        elements[1 + 1 * 4] = q;
        elements[2 + 2 * 4] = b;
        elements[2 + 3 * 4] = -1.0;
        elements[3 + 2 * 4] = c;

        Mat4 { elements: elements }
    }

    /// Returns new orthographic projection matrix
    pub fn new_ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
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

    /// multiply by other mat4
    pub fn multiply(&self, other: &Mat4) -> Mat4 {

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

        Mat4 {elements: data }
    }

    pub fn translate(translation: &Vec3) -> Mat4 {
        let mut mat = Mat4::new_identity();

        // col + row * 4
        mat.elements[3 + 0 * 4] = translation.x;
        mat.elements[3 + 1 * 4] = translation.y;
        mat.elements[3 + 2 * 4] = translation.z;

        // mat
        Mat4 { elements: mat.elements }
    }

    pub fn rotate(angle: f32, axis: &Vec3) -> Mat4{
        let mut mat = Mat4::new_identity();

        let r = angle.to_radians();
        let c = r.cos();
        let s = r.sin();
        let omc = 1.0 - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        // col + row * 4
        mat.elements[0 + 0 * 4] = x * omc + c;
        mat.elements[0 + 1 * 4] = y * x * omc + z * s;
        mat.elements[0 + 2 * 4] = x * z * omc - y * s;

        mat.elements[1 + 0 * 4] = x * y * omc - z * s;
        mat.elements[1 + 1 * 4] = y * omc + c;
        mat.elements[1 + 2 * 4] = y * z * omc + x * s;

        mat.elements[2 + 0 * 4] = x * z * omc + y * s;
        mat.elements[2 + 1 * 4] = y * z * omc - x * s;
        mat.elements[2 + 2 * 4] = z * omc + c;

        mat
    }

    pub fn scale(scale: &Vec3) -> Mat4 {
        let mut mat = Mat4::new_identity();

        mat.elements[0 + 0 * 4] = scale.x;
        mat.elements[1 + 1 * 4] = scale.y;
        mat.elements[2 + 2 * 4] = scale.z;

        mat
    }

    pub fn new_look_at(camera: &Vec3, object: &Vec3, up: &Vec3) -> Mat4 {
        let mut mat = Mat4::new_identity();

        let f = object.sub(camera).normalize();

        let s = f.cross(&up.normalize());

        let u = s.cross(&f);

        mat.elements[0 + 0 * 4] = s.x;
        mat.elements[0 + 1 * 4] = s.y;
        mat.elements[0 + 2 * 4] = s.z;

        mat.elements[1 + 0 * 4] = u.x;
        mat.elements[1 + 1 * 4] = u.y;
        mat.elements[1 + 2 * 4] = u.z;

        mat.elements[2 + 0 * 4] = -f.x;
        mat.elements[2 + 1 * 4] = -f.y;
        mat.elements[2 + 2 * 4] = -f.z;

        let m = Mat4::translate(&Vec3::new(-camera.x, -camera.y, -camera.z));
        m.multiply(&mat)
    }


    #[inline]
    pub fn as_ptr(&self) -> *const f32 {
        &self.elements[0]
    }
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
        "[ {}, {}, {}, {}\n  {}, {}, {}, {}\n  {}, {}, {}, {}\n  {}, {}, {}, {} ]",
        self.elements[0], self.elements[1], self.elements[2], self.elements[3],
        self.elements[4], self.elements[5], self.elements[6], self.elements[7],
        self.elements[8], self.elements[9], self.elements[10], self.elements[11],
        self.elements[12], self.elements[13], self.elements[14], self.elements[15])
    }
}
