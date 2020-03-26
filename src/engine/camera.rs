extern crate math;

use self::math::mat4::Mat4;
use self::math::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,

    pub projection_matrix: Mat4,
}

impl Camera {
    pub fn new(width: f32, height: f32, position: Vec3, rotation: Vec3) -> Camera {
        Camera {
            position: position,
            rotation: rotation,
            forward: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            // near - as big as posible (0.1)
            // far - as small as posible (100 - far and small enought)
            projection_matrix: Mat4::from_perspective(45.0, width / height, 0.1, 100.0),
        }
    }

    pub fn get_look_at_target_matrix(&self, target: Vec3) -> Mat4 {
        let roll = self.rotation.x.to_radians();
        let up = Vec3::new(roll.sin(), roll.cos(), 0.0);
        Mat4::from_look_at(self.position, target, up)
    }

    pub fn get_look_at_matrix(&self) -> Mat4 {
        // let roll = self.rotation.x.to_radians();
        let pitch = self.rotation.y.to_radians();
        let yaw = self.rotation.z.to_radians();

        let mut forward = Vec3::new(self.forward.x, self.forward.y, self.forward.z);
        forward.x = pitch.cos() * yaw.cos();
        forward.y = pitch.sin();
        forward.z = pitch.cos() * yaw.sin();
        forward = forward.normalize();

        // let up = Vec3::new(roll.sin(), roll.cos(), 0.0);

        // let right = Vec3::cross(self.forward, self.up).normalize();
        let up = Vec3::cross(self.right, self.forward).normalize();

        Mat4::from_look_at(self.position, self.position + forward, up)
    }
}
